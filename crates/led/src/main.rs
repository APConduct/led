use anyhow::Result as AnyResult;
use eframe::egui;
use led::led::*;
use mlua::Lua;
use saran::context::Context;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "LED Editor",
        options,
        Box::new(|cc| Ok(Box::new(txt::edtr::App::new(cc)))),
    )
}

pub mod lua {
    use anyhow::Result as AnyResult;
    use std::collections::HashMap;

    use led::led::commands::editor::Command;
    use mlua::Lua;

    pub struct Runtime {
        lua: Lua,
        pending_cmds: Vec<Command>,
    }

    impl Runtime {
        pub fn new() -> AnyResult<Self> {
            let lua = Lua::new();
            Ok(Self {
                lua,
                pending_cmds: Vec::new(),
            })
        }

        pub fn load_default_config(&mut self) -> AnyResult<()> {
            let config_script = r##"
        -- Default KUP Editor Configuration

        -- Key bindings
        kup = {}
        kup.keybindings = {}

        function kup.bind_key(key, action)
            kup.keybindings[key] = action
        end

        -- Example keybindings
        kup.bind_key("ctrl+s", function()
            -- Save file
            return { type = "SaveBuffer", buffer_id = kup.current_buffer }
        end)

        kup.bind_key("ctrl+o", function()
            -- Open file
            return { type = "OpenFile" }
        end)

        -- Theme configuration
        kup.theme = {
            background = "#282c34",
            foreground = "#abb2bf",
            cursor = "#ffffff",
            selection = "#3d85c6",
            line_numbers = "#808080"
        }

        -- Editor settings
        kup.settings = {
            tab_size = 4,
            show_line_numbers = true,
            font_size = 14,
            auto_save = true
        }

        print("KUP Editor configuration loaded")
        "##;

            self.lua.load(config_script).exec()?;
            Ok(())
        }

        pub fn proccess_frame_commands(
            &mut self,
        ) -> AnyResult<Vec<led::led::commands::editor::Command>> {
            let cmds = self.pending_cmds.clone();
            self.pending_cmds.clear();
            Ok(cmds)
        }

        pub fn execute_keybinding(&mut self, key: &str) -> AnyResult<()> {
            let script = format!(
                r#"
                        if kup.keybindings["{}"] then
                            local result = kup.keybindings["{}"]()
                            if result then
                                return result
                            end
                        end
                        return nil
                        "#,
                key, key
            );

            let result: Option<mlua::Value> = self.lua.load(&script).eval()?;

            if let Some(value) = result {
                // TODO: Convert Lua result to editor::Command
                println!("Lua keybinding result: {:?}", value);
            }
            Ok(())
        }
    }
}

pub mod txt {
    pub mod edtr {
        use super::super::lua::Runtime;
        use egui::{Context as EguiContext, Rect, Ui};
        use led::led::{buffer::editor::State, commands::editor::Response, types::Range};
        use saran::{context::Context as GuiContext, theme::Theme};

        pub struct App {
            edtr_state: State,
            gui_ctx: GuiContext,
            lua_runtime: Runtime,

            show_line_numbers: bool,
            font_size: f32,
            tab_size: usize,

            frame_time: f32,
            last_frame_time: std::time::Instant,
        }

        impl App {
            pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
                let mut app = Self {
                    edtr_state: State::new(),
                    gui_ctx: GuiContext::new(cc.egui_ctx.clone()),
                    lua_runtime: Runtime::new().expect("Failed to create Lua runtime"),
                    show_line_numbers: true,
                    font_size: 14.0,
                    tab_size: 4,

                    frame_time: 0.0,
                    last_frame_time: std::time::Instant::now(),
                };

                let content = r#"// Welcome to LED!!!!
                // The Editor 4U!!!!
                fn main() {
                    println!("Hello, world!");
                }
                "#
                .to_string();

                app.edtr_state.create_buffer(content);

                // TODO: load and configure initial Lua state

                app
            }

            fn configure_equi_style(&self, ctx: &egui::Context) {
                let mut style = (*ctx.style()).clone();

                style.text_styles.insert(
                    egui::TextStyle::Monospace,
                    egui::FontId::monospace(self.font_size),
                );

                style.visuals.dark_mode = true;
                style.visuals.override_text_color = Some(egui::Color32::from_rgb(172, 178, 191));
                style.visuals.extreme_bg_color = egui::Color32::from_rgb(40, 44, 52);
                style.visuals.code_bg_color = egui::Color32::from_rgb(50, 54, 62);

                ctx.set_style(style);
            }
        }

        impl eframe::App for App {
            fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
                let now = std::time::Instant::now();
                self.frame_time = now.duration_since(self.last_frame_time).as_secs_f32();
                self.last_frame_time = now;

                if let Ok(commands) = self.lua_runtime.proccess_frame_commands() {
                    for command in commands {
                        let _ = self.edtr_state.execute_command(command);
                    }
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render_editor_ui(ui);
                });

                // Menu bar
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    self.render_menu_bar(ui);
                });

                ctx.request_repaint_after(std::time::Duration::from_millis(500));
            }
        }

        impl App {
            fn render_editor_ui(&mut self, ui: &mut egui::Ui) {
                if let Some(buffer_id) = self.edtr_state.get_active_buffer() {
                    let avail_rect = ui.available_rect_before_wrap();

                    let mut text_editor =
                        Widget::new(buffer_id, &mut self.edtr_state, &mut self.gui_ctx);
                    text_editor.show_line_numbers = self.show_line_numbers;
                    text_editor.tab_size = self.tab_size;

                    let response = text_editor.show(ui, avail_rect);

                    if let Some(edtr_response) = response {
                        for command in edtr_response.commands {
                            let _ = self.edtr_state.execute_command(command);
                        }
                    }
                }

                todo!("Implement the editor UI rendering logic here");
            }

            fn render_status_bar(&self, ui: &mut egui::Ui) {
                ui.horizontal(|ui| ui.label(format!("Frame: {:.1}ms", self.frame_time * 1000.0)));
                ui.separator();
                // Cursor pos
                if let Some(buffer_id) = self.edtr_state.get_active_buffer() {
                    if let Some(cursor) = self.edtr_state.get_cursor_state(buffer_id) {
                        ui.label(format!(
                            "Ln {}, Col {}",
                            cursor.position().line + 1,
                            cursor.position().column + 1
                        ));
                    }
                }
                ui.separator();

                // Buffer info
                ui.label("UTF-8");
                ui.label("Rust");
            }

            fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            let buffer_id = self.edtr_state.create_buffer(String::new());
                        }

                        if ui.button("Open").clicked() {
                            todo!("Implement Open functionality");
                        }

                        if ui.button("Save").clicked() {
                            todo!("Implement Save functionality");
                        }

                        ui.separator();

                        if ui.button("Exit").clicked() {
                            std::process::exit(0);
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo").clicked() {
                            todo!("Implement editing support");
                        }

                        if ui.button("Redo").clicked() {
                            todo!("Implement editing support");
                        }

                        ui.separator();

                        if ui.button("Find").clicked() {
                            todo!("Implement Find functionality");
                        }
                    });
                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut self.show_line_numbers, "Show Line Numbers");
                        ui.separator();

                        ui.label("Font Size:");
                        ui.add(egui::Slider::new(&mut self.font_size, 8.0..=24.0));

                        ui.label("Tab Size:");
                        ui.add(egui::Slider::new(&mut self.tab_size, 2..=8));
                    });
                });
            }
        }

        pub struct Widget<'a> {
            buffer_id: led::led::buffer::ID,
            edtr_state: &'a mut led::led::buffer::editor::State,
            gui_ctx: &'a mut saran::context::Context,
            show_line_numbers: bool,

            font_size: f32,
            tab_size: usize,

            cursor_blink_time: f32,
            scroll_offset: egui::Vec2,
        }

        impl<'a> Widget<'a> {
            pub fn new(
                buffer_id: led::led::buffer::ID,
                edtr_state: &'a mut led::led::buffer::editor::State,
                gui_ctx: &'a mut saran::context::Context,
            ) -> Self {
                Self {
                    buffer_id,
                    edtr_state,
                    gui_ctx,
                    show_line_numbers: true,
                    font_size: 14.0,
                    tab_size: 4,
                    cursor_blink_time: 0.0,
                    scroll_offset: egui::Vec2::ZERO,
                }
            }

            pub fn show(
                &mut self,
                ui: &mut Ui,
                _rect: Rect,
            ) -> Option<led::led::commands::editor::Response> {
                let mut response = Response {
                    commands: Vec::new(),
                    cursor_moved: false,
                    text_changed: false,
                };

                let text = self.edtr_state.get_buffer_text(self.buffer_id)?;
                let crsr_state = self.edtr_state.get_cursor_state(self.buffer_id)?;

                let font_id = egui::FontId::monospace(self.font_size);
                let line_height = ui.fonts(|f| f.row_height(&font_id));
                let char_width = ui.fonts(|f| f.glyph_width(&font_id, ' '));

                let scroll_area = egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .stick_to_right(false)
                    .stick_to_bottom(false);

                // let scroll_response = scroll_area.show(ui, |ui| {
                //     let avail_width = ui.available_width();
                //     let line_number_width = if self.show_line_numbers {
                //         let line_count = text.lines().count();
                //         let digits = line_count.to_string().len();
                //         (digits as f32 * char_width) + (char_width * 2.0)
                //     } else {
                //         0.0
                //     };

                //     let text_width = avail_width - line_number_width;

                //     if self.show_line_numbers {
                //         self.render_line_numbers(ui, &text, line_height, line_number_width);
                //     }
                //     ui.allocate_ui_at_rect(
                //         egui::Rect::from_min_size(
                //             egui::pos2(line_number_width, 0.0),
                //             egui::vec2(text_width, ui.available_height()),
                //         ),
                //         |ui| {
                //             self.render_text_content(
                //                 ui,
                //                 &text,
                //                 crsr_state,
                //                 line_height,
                //                 char_width,
                //                 &mut response,
                //             )
                //         },
                //     );
                // });
                // self.handle_input(ui, &mut response);
                // Some(response)
                todo!("Implement the editor rendering logic here");
            }

            fn render_line_numbers(
                &self,
                ui: &mut egui::Ui,
                text: &str,
                line_height: f32,
                width: f32,
            ) {
                todo!("Implement line number rendering");
            }

            fn render_text_content(
                &mut self,
                ui: &mut egui::Ui,
                text: &str,
                crsr_state: &led::led::cursor::State,
                line_height: f32,
                char_width: f32,
                response: &mut led::led::commands::editor::Response,
            ) {
                todo!("Implement text content rendering");
            }

            fn handle_input(
                &mut self,
                ui: &mut egui::Ui,
                response: &mut led::led::commands::editor::Response,
            ) {
                todo!("Implement input handling");
            }

            fn render_cursor(
                &mut self,
                ui: &mut egui::Ui,
                cursor_state: &State,
                line_height: f32,
                char_width: f32,
                theme: &Theme,
            ) {
                todo!("Implement cursor rendering");
            }

            fn render_selection(
                &self,
                ui: &mut egui::Ui,
                text: &str,
                selection: Range,
                line_height: f32,
                char_width: f32,
                theme: &Theme,
            ) {
                todo!("Implement selection rendering");
            }

            fn handle_key_event(
                &mut self,
                key: egui::Key,
                modifiers: egui::Modifiers,
                response: &mut Response,
            ) {
                todo!("Implement key input handling");
            }
        }
    }
}
