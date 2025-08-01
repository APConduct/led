pub mod edtr {
    use super::super::lua::Runtime;
    use super::super::{
        super::led,
        buffer::editor::State,
        commands::editor::{self, Response},
        cursor,
        types::{Position, Range},
    };
    use egui::{Rect, Ui};
    use rfd::FileDialog;
    use saran::{context::Context as GuiContext, theme::Theme};
    use std::fs;

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

            // Ensure scroll area fills the central panel
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

                // Commands are now executed immediately in Widget::show, so do not execute them here.
            }
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
                        if let Some(path) = FileDialog::new().pick_file() {
                            match fs::read_to_string(&path) {
                                Ok(content) => {
                                    let buffer_id = self.edtr_state.create_buffer(content);
                                    // Store file path in buffer metadata
                                    if let Some(meta) =
                                        self.edtr_state.buffer_metadata.get_mut(&buffer_id)
                                    {
                                        meta.file_path = Some(path.to_string_lossy().to_string());
                                        meta.modified = false;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to open file: {}", e);
                                    // TODO: Display error in UI instead of just printing to console
                                }
                            }
                        }
                    }

                    if ui.button("Save").clicked() {
                        if let Some(buffer_id) = self.edtr_state.get_active_buffer() {
                            let file_path = self
                                .edtr_state
                                .buffer_metadata
                                .get(&buffer_id)
                                .and_then(|meta| meta.file_path.clone())
                                .or_else(|| {
                                    FileDialog::new()
                                        .save_file()
                                        .map(|p| p.to_string_lossy().to_string())
                                });

                            if let Some(path) = file_path {
                                if let Some(content) = self.edtr_state.get_buffer_text(buffer_id) {
                                    match fs::write(&path, content) {
                                        Ok(_) => {
                                            // Update buffer metadata
                                            if let Some(meta) =
                                                self.edtr_state.buffer_metadata.get_mut(&buffer_id)
                                            {
                                                meta.file_path = Some(path);
                                                meta.modified = false;
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to save file: {}", e);
                                            // TODO: Display error in UI instead of just printing to console
                                        }
                                    }
                                }
                            }
                        }
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
        buffer_id: led::buffer::ID,
        edtr_state: &'a mut led::buffer::editor::State,
        gui_ctx: &'a mut saran::context::Context,
        show_line_numbers: bool,

        font_size: f32,
        tab_size: usize,

        cursor_blink_time: f32,
        scroll_offset: egui::Vec2,
    }

    // Padding constants for editor layout
    const TOP_PADDING: f32 = 4.0;
    const LEFT_PADDING: f32 = 4.0;
    // Additional padding for buffer text area
    const TEXT_TOP_PADDING: f32 = 16.0;
    const TEXT_LEFT_PADDING: f32 = 32.0;

    impl<'a> Widget<'a> {
        pub fn new(
            buffer_id: led::buffer::ID,
            edtr_state: &'a mut led::buffer::editor::State,
            gui_ctx: &'a mut saran::context::Context,
        ) -> Self {
            // println!("[DEBUG] Widget::new called");
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
        ) -> Option<led::commands::editor::Response> {
            let mut response = Response {
                commands: Vec::new(),
                cursor_moved: false,
                text_changed: false,
            };

            // Get buffer text and cursor state
            let text = self.edtr_state.get_buffer_text(self.buffer_id)?.to_string();
            let mut crsr_state = self.edtr_state.get_cursor_state(self.buffer_id)?.clone();

            let font_id = egui::FontId::monospace(self.font_size);
            let line_height = ui.fonts(|f| f.row_height(&font_id));
            let char_width = ui.fonts(|f| f.glyph_width(&font_id, ' '));

            let line_count = text.lines().count();
            let max_line_length = text.lines().map(|l| l.len()).max().unwrap_or(0);

            // Calculate content size for scrolling
            // Fixed gutter width for up to 99,999 lines (5 digits)
            let max_digits = 5;
            let line_number_width = if self.show_line_numbers {
                (max_digits as f32 * char_width) + (char_width * 2.0)
            } else {
                0.0
            };
            let content_width = LEFT_PADDING
                + TEXT_LEFT_PADDING
                + line_number_width
                + (max_line_length as f32 * char_width)
                + 100.0;
            let content_height =
                TOP_PADDING + TEXT_TOP_PADDING + (line_count as f32 * line_height) + 100.0;

            // Scroll area for both axes
            // Calculate minimum allocation based on available viewport
            let min_width = ui.available_width();
            let min_height = ui.available_height();
            let alloc_width = content_width.max(min_width);
            let alloc_height = content_height.max(min_height);

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .stick_to_right(false)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    // Allocate the full content area (fixed for morphing/jank)
                    let (rect, _response) = ui.allocate_exact_size(
                        egui::vec2(alloc_width, alloc_height),
                        egui::Sense::hover(),
                    );

                    let theme = self.gui_ctx.style_system.get_active_theme().clone();
                    let origin = ui.min_rect().min;

                    // Local flag for auto-scroll
                    let mut should_scroll_to_cursor = false;

                    // Handle keyboard and text input
                    ui.input(|i| {
                        for event in &i.events {
                            match event {
                                egui::Event::Text(text) => {
                                    // Insert text at refreshed cursor position
                                    if let Some(cursor) =
                                        self.edtr_state.get_cursor_state(self.buffer_id)
                                    {
                                        let buffer =
                                            self.edtr_state.buffers().get(&self.buffer_id).unwrap();
                                        let offset = buffer.position_to_offset(cursor.position());

                                        response.commands.push(editor::Command::InsertText {
                                            buffer_id: self.buffer_id,
                                            offset,
                                            text: text.clone(),
                                        });

                                        response.text_changed = true;

                                        // Advance cursor right by one column after insert
                                        let mut new_pos = cursor.position();
                                        new_pos.column += text.chars().count(); // Usually 1, but supports paste
                                        response.commands.push(editor::Command::MoveCursor {
                                            buffer_id: self.buffer_id,
                                            position: new_pos,
                                        });
                                        response.cursor_moved = true;

                                        // Reset preferred_column on text input
                                        if let Some(cursor_mut) =
                                            self.edtr_state.cursors.get_mut(&self.buffer_id)
                                        {
                                            cursor_mut.preferred_column = None;
                                        }
                                        // Set flag to auto-scroll after text input
                                        should_scroll_to_cursor = true;
                                    }
                                }
                                egui::Event::Key {
                                    key,
                                    pressed: true,
                                    modifiers,
                                    ..
                                } => {
                                    self.handle_key_event(*key, *modifiers, &mut response);
                                    // Set flag to auto-scroll only if movement or edit occurred
                                    if response.cursor_moved || response.text_changed {
                                        should_scroll_to_cursor = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                    });

                    // Paint background
                    ui.painter()
                        .rect_filled(rect, egui::Rounding::ZERO, theme.background);

                    // Paint line numbers and text
                    let mut y = origin.y + TOP_PADDING + TEXT_TOP_PADDING;
                    for (line_num, line) in text.lines().enumerate() {
                        let mut x = origin.x + LEFT_PADDING;
                        if self.show_line_numbers {
                            // Pad line numbers to 5 digits, right-aligned
                            let line_text = format!("{:>width$}", line_num + 1, width = max_digits);
                            // The right edge of the gutter:
                            let gutter_right_x =
                                origin.x + LEFT_PADDING + line_number_width - char_width;
                            // Paint the line number so its right edge is at gutter_right_x
                            let pos = egui::pos2(gutter_right_x, y);
                            ui.painter().text(
                                pos,
                                egui::Align2::RIGHT_TOP,
                                line_text,
                                font_id.clone(),
                                theme.line_numbers,
                            );
                            x += line_number_width;
                        }
                        x += TEXT_LEFT_PADDING;
                        let color = if line.trim_start().starts_with("//") {
                            egui::Color32::from_rgb(128, 128, 128)
                        } else if line.contains("fn ") || line.contains("let ") {
                            egui::Color32::from_rgb(198, 120, 221)
                        } else {
                            theme.foreground
                        };
                        let pos = egui::pos2(x, y);
                        ui.painter().text(
                            pos,
                            egui::Align2::LEFT_TOP,
                            line,
                            font_id.clone(),
                            color,
                        );
                        y += line_height;
                    }

                    // Render selection and cursor after text
                    let selection = crsr_state.selection().unwrap_or_else(|| Range {
                        start: Position { line: 0, column: 0 },
                        end: Position { line: 0, column: 0 },
                    });
                    self.render_selection(
                        ui,
                        &text,
                        selection,
                        line_height,
                        char_width,
                        &theme,
                        line_number_width,
                    );
                    self.render_cursor(
                        ui,
                        &crsr_state,
                        line_height,
                        char_width,
                        &theme,
                        line_number_width,
                    );
                    // Always refetch the updated cursor state after executing commands
                    if let Some(cursor_state) = self.edtr_state.get_cursor_state(self.buffer_id) {
                        crsr_state = cursor_state.clone();
                    }

                    // Only auto-scroll if movement or edit occurred (fix phantom scrolling)
                    if should_scroll_to_cursor {
                        let cursor_x = crsr_state.position().column as f32 * char_width
                            + origin.x
                            + LEFT_PADDING
                            + line_number_width
                            + TEXT_LEFT_PADDING;
                        let cursor_y = crsr_state.position().line as f32 * line_height
                            + origin.y
                            + TOP_PADDING
                            + TEXT_TOP_PADDING;
                        let cursor_rect = egui::Rect::from_min_size(
                            egui::pos2(cursor_x, cursor_y),
                            egui::vec2(2.0, line_height),
                        );
                        // Add a 2-line scroll margin so the cursor can move closer to the top/bottom before triggering scroll
                        let margin_lines = 2.0;
                        let margin = line_height * margin_lines;
                        let clip_rect = ui.clip_rect();
                        let expanded_cursor_rect = cursor_rect.expand(margin);

                        if !clip_rect.contains_rect(expanded_cursor_rect) {
                            ui.scroll_to_rect(expanded_cursor_rect, None);
                        }
                        // Reset flag after scrolling
                        should_scroll_to_cursor = false;
                    }

                    // Handle input (mouse and keyboard) with scroll offset
                    // (removed call to handle_input_with_scroll; all input handling is now inside the scroll area closure)
                });

            // Immediately execute commands so state is up-to-date
            for command in &response.commands {
                let _ = self.edtr_state.execute_command(command.clone());
            }
            // Always refetch the updated cursor state after executing commands
            crsr_state = self.edtr_state.get_cursor_state(self.buffer_id)?.clone();

            Some(response)
        }

        fn render_line_numbers(&self, ui: &mut egui::Ui, text: &str, line_height: f32, width: f32) {
            let theme = self.gui_ctx.style_system.get_active_theme();
            let font_id = egui::FontId::monospace(self.font_size);

            let mut y = TOP_PADDING;
            for (line_num, _) in text.lines().enumerate() {
                let line_text = format!("{:>4}", line_num + 1);
                // No baseline adjustment; align exactly with buffer text
                let baseline_adjust = line_height * 0.8;
                let pos = egui::pos2(LEFT_PADDING, y + baseline_adjust);
                ui.painter().text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    line_text,
                    font_id.clone(),
                    theme.line_numbers,
                );
                y += line_height;
            }

            let sep_x = width - (self.font_size * 0.5) + LEFT_PADDING;
            ui.painter().line_segment(
                [
                    egui::pos2(sep_x, TOP_PADDING),
                    egui::pos2(sep_x, ui.available_height()),
                ],
                egui::Stroke::new(1.0, theme.line_numbers),
            );
        }

        fn render_text_content(
            &mut self,
            ui: &mut egui::Ui,
            text: &str,
            cursor_state: &cursor::State,
            line_height: f32,
            char_width: f32,
            _response: &mut editor::Response,
        ) {
            // Calculate line_number_width here for use in selection/cursor rendering
            let line_number_width = if self.show_line_numbers {
                let line_count = text.lines().count();
                let digits = line_count.to_string().len();
                (digits as f32 * char_width) + (char_width * 2.0)
            } else {
                0.0
            };
            // Clone theme before any mutable borrow of self
            let theme = self.gui_ctx.style_system.get_active_theme().clone();
            let font_id = egui::FontId::monospace(self.font_size);

            // Render background
            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                egui::Rounding::ZERO,
                theme.background,
            );

            // Render selection
            if let Some(selection) = cursor_state.selection() {
                self.render_selection(
                    ui,
                    text,
                    selection,
                    line_height,
                    char_width,
                    &theme,
                    line_number_width,
                );
            }

            // Render text
            let mut y = TOP_PADDING;
            for (_line_num, line) in text.lines().enumerate() {
                let pos = egui::pos2(LEFT_PADDING, y);

                // Simple syntax highlighting (can be expanded)
                let color = if line.trim_start().starts_with("//") {
                    egui::Color32::from_rgb(128, 128, 128) // Comments
                } else if line.contains("fn ") || line.contains("let ") {
                    egui::Color32::from_rgb(198, 120, 221) // Keywords
                } else {
                    theme.foreground
                };

                ui.painter()
                    .text(pos, egui::Align2::LEFT_TOP, line, font_id.clone(), color);

                y += line_height;
            }

            // Render cursor
            self.render_cursor(ui, cursor_state, line_height, char_width, &theme, 0.0);

            // Handle text input
            if ui.rect_contains_pointer(ui.available_rect_before_wrap()) {
                ui.memory_mut(|mem| mem.request_focus(ui.next_auto_id()));
            }
        }

        // Helper: handle input with scroll offset
        // (now unused, all input handling is inside the scroll area closure)

        fn render_cursor(
            &mut self,
            ui: &mut egui::Ui,
            cursor_state: &cursor::State,
            line_height: f32,
            char_width: f32,
            theme: &Theme,
            line_number_width: f32,
        ) {
            // Cursor blinking
            self.cursor_blink_time += ui.input(|i| i.unstable_dt);
            let cursor_visible = (self.cursor_blink_time * 2.0) % 2.0 < 1.0;

            if cursor_visible {
                let origin = ui.min_rect().min;
                let cursor_x = cursor_state.position().column as f32 * char_width
                    + origin.x
                    + LEFT_PADDING
                    + line_number_width
                    + TEXT_LEFT_PADDING;
                let cursor_y = cursor_state.position().line as f32 * line_height
                    + origin.y
                    + TOP_PADDING
                    + TEXT_TOP_PADDING;

                ui.painter().line_segment(
                    [
                        egui::pos2(cursor_x, cursor_y),
                        egui::pos2(cursor_x, cursor_y + line_height),
                    ],
                    egui::Stroke::new(2.0, theme.cursor),
                );
            }
        }

        fn render_selection(
            &self,
            ui: &mut egui::Ui,
            text: &str,
            selection: Range,
            line_height: f32,
            char_width: f32,
            theme: &Theme,
            line_number_width: f32,
        ) {
            // Simple selection rendering - can be optimized
            let start_y = selection.start.line as f32 * line_height + TOP_PADDING;
            let end_y = selection.end.line as f32 * line_height + TOP_PADDING;

            if selection.start.line == selection.end.line {
                // Single line selection
                let start_x =
                    selection.start.column as f32 * char_width + LEFT_PADDING + line_number_width;
                let end_x =
                    selection.end.column as f32 * char_width + LEFT_PADDING + line_number_width;

                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(start_x, start_y),
                        egui::vec2(end_x - start_x, line_height),
                    ),
                    egui::Rounding::ZERO,
                    theme.selection,
                );
            } else {
                // Multi-line selection (simplified)
                for line in selection.start.line..=selection.end.line {
                    let y = line as f32 * line_height + TOP_PADDING;
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(LEFT_PADDING + line_number_width, y),
                            egui::vec2(ui.available_width(), line_height),
                        ),
                        egui::Rounding::ZERO,
                        theme.selection,
                    );
                }
            }
        }

        fn handle_key_event(
            &mut self,
            key: egui::Key,
            modifiers: egui::Modifiers,
            response: &mut editor::Response,
        ) {
            use egui::Key;

            match key {
                Key::ArrowLeft => {
                    // Move cursor left
                    let text = self
                        .edtr_state
                        .get_buffer_text(self.buffer_id)
                        .unwrap_or_default();
                    let lines: Vec<&str> = text.lines().collect();
                    if let Some(cursor) = self.edtr_state.cursors.get_mut(&self.buffer_id) {
                        let mut new_pos = cursor.position;
                        if new_pos.column > 0 {
                            new_pos.column -= 1;
                        } else if new_pos.line > 0 {
                            new_pos.line -= 1;
                            // Move to end of previous line
                            if new_pos.line < lines.len() {
                                new_pos.column = lines[new_pos.line].len();
                            }
                        }
                        // Reset preferred column on horizontal movement
                        cursor.preferred_column = None;

                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });

                        response.cursor_moved = true;
                    }
                }

                Key::ArrowRight => {
                    // Move cursor right
                    let text = self
                        .edtr_state
                        .get_buffer_text(self.buffer_id)
                        .unwrap_or_default();
                    let lines: Vec<&str> = text.lines().collect();
                    if let Some(cursor) = self.edtr_state.cursors.get_mut(&self.buffer_id) {
                        let mut new_pos = cursor.position;

                        if new_pos.line < lines.len() {
                            let current_line = lines[new_pos.line];
                            if new_pos.column < current_line.len() {
                                new_pos.column += 1;
                            } else if new_pos.line + 1 < lines.len() {
                                new_pos.line += 1;
                                new_pos.column = 0;
                            }
                        }
                        // Reset preferred column on horizontal movement
                        cursor.preferred_column = None;

                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });

                        response.cursor_moved = true;
                    }
                }

                Key::ArrowUp => {
                    // Move cursor up with preferred column logic
                    let text = self
                        .edtr_state
                        .get_buffer_text(self.buffer_id)
                        .unwrap_or_default();
                    let lines: Vec<&str> = text.lines().collect();
                    if let Some(cursor) = self.edtr_state.cursors.get_mut(&self.buffer_id) {
                        let mut new_pos = cursor.position;

                        // Set preferred_column only if None (first vertical move after horizontal)
                        if cursor.preferred_column.is_none() {
                            cursor.preferred_column = Some(cursor.position.column);
                        }
                        // println!(
                        //     "[DEBUG][ArrowUp] preferred_column={:?}, before={:?}, moving to line={}, target_line_len={}",
                        //     cursor.preferred_column,
                        //     cursor.position,
                        //     if new_pos.line > 0 {
                        //         new_pos.line - 1
                        //     } else {
                        //         0
                        //     },
                        //     lines
                        //         .get(if new_pos.line > 0 {
                        //             new_pos.line - 1
                        //         } else {
                        //             0
                        //         })
                        //         .map(|l| l.len())
                        //         .unwrap_or(0)
                        // );

                        if new_pos.line > 0 {
                            new_pos.line -= 1;
                        }

                        // Always use preferred_column for vertical moves, clamped to line length
                        let target_line_len = lines.get(new_pos.line).map(|l| l.len()).unwrap_or(0);
                        new_pos.column = cursor.preferred_column.unwrap().min(target_line_len);

                        // println!(
                        //     "[DEBUG][ArrowUp] after move: new_pos={:?}, preferred_column={:?}",
                        //     new_pos, cursor.preferred_column
                        // );
                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });

                        response.cursor_moved = true;
                    }
                }

                Key::ArrowDown => {
                    // Move cursor down with preferred column logic
                    let text = self
                        .edtr_state
                        .get_buffer_text(self.buffer_id)
                        .unwrap_or_default();
                    let lines: Vec<&str> = text.lines().collect();
                    if let Some(cursor) = self.edtr_state.cursors.get_mut(&self.buffer_id) {
                        let mut new_pos = cursor.position;

                        // Set preferred_column only if None (first vertical move after horizontal)
                        if cursor.preferred_column.is_none() {
                            cursor.preferred_column = Some(cursor.position.column);
                        }
                        // println!(
                        //     "[DEBUG][ArrowDown] preferred_column={:?}, before={:?}, moving to line={}, target_line_len={}",
                        //     cursor.preferred_column,
                        //     cursor.position,
                        //     if new_pos.line + 1 < lines.len() {
                        //         new_pos.line + 1
                        //     } else {
                        //         new_pos.line
                        //     },
                        //     lines
                        //         .get(if new_pos.line + 1 < lines.len() {
                        //             new_pos.line + 1
                        //         } else {
                        //             new_pos.line
                        //         })
                        //         .map(|l| l.len())
                        //         .unwrap_or(0)
                        // );

                        if new_pos.line + 1 < lines.len() {
                            new_pos.line += 1;
                        }

                        // Always use preferred_column for vertical moves, clamped to line length
                        let target_line_len = lines.get(new_pos.line).map(|l| l.len()).unwrap_or(0);
                        new_pos.column = cursor.preferred_column.unwrap().min(target_line_len);

                        // println!(
                        //     "[DEBUG][ArrowDown] after move: new_pos={:?}, preferred_column={:?}",
                        //     new_pos, cursor.preferred_column
                        // );
                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });

                        response.cursor_moved = true;
                    }
                }

                Key::Backspace => {
                    // Delete character before cursor
                    if let Some(cursor) = self.edtr_state.get_cursor_state(self.buffer_id) {
                        if cursor.position().column > 0 || cursor.position().line > 0 {
                            let buffer = self.edtr_state.buffers().get(&self.buffer_id).unwrap();
                            let offset = buffer.position_to_offset(cursor.position());

                            if offset > 0 {
                                response.commands.push(editor::Command::DeleteText {
                                    buffer_id: self.buffer_id,
                                    start: offset - 1,
                                    length: 1,
                                });

                                response.text_changed = true;

                                // Move cursor left after deletion
                                let mut new_pos = cursor.position();
                                if new_pos.column > 0 {
                                    new_pos.column -= 1;
                                } else if new_pos.line > 0 {
                                    new_pos.line -= 1;
                                    // Move to end of previous line
                                    if let Some(text) =
                                        self.edtr_state.get_buffer_text(self.buffer_id)
                                    {
                                        let lines: Vec<&str> = text.lines().collect();
                                        if new_pos.line < lines.len() {
                                            new_pos.column = lines[new_pos.line].len();
                                        }
                                    }
                                }
                                response.commands.push(editor::Command::MoveCursor {
                                    buffer_id: self.buffer_id,
                                    position: new_pos,
                                });
                                response.cursor_moved = true;

                                // Reset preferred_column on deletion
                                if let Some(cursor_mut) =
                                    self.edtr_state.cursors.get_mut(&self.buffer_id)
                                {
                                    cursor_mut.preferred_column = None;
                                }
                                // Set flag to auto-scroll after deletion
                            }
                        }
                    }
                }

                Key::Delete => {
                    // Delete character after cursor
                    if let Some(cursor) = self.edtr_state.get_cursor_state(self.buffer_id) {
                        let buffer = self.edtr_state.buffers().get(&self.buffer_id).unwrap();
                        let offset = buffer.position_to_offset(cursor.position());

                        if offset < buffer.len() {
                            response.commands.push(editor::Command::DeleteText {
                                buffer_id: self.buffer_id,
                                start: offset,
                                length: 1,
                            });

                            response.text_changed = true;

                            // Reset preferred_column on deletion
                            if let Some(cursor_mut) =
                                self.edtr_state.cursors.get_mut(&self.buffer_id)
                            {
                                cursor_mut.preferred_column = None;
                            }
                            // Set flag to auto-scroll after deletion
                        }
                    }
                }

                Key::Tab => {
                    // Insert tab_size spaces
                    if let Some(cursor) = self.edtr_state.get_cursor_state(self.buffer_id) {
                        let buffer = self.edtr_state.buffers().get(&self.buffer_id).unwrap();
                        let offset = buffer.position_to_offset(cursor.position());

                        let tab_str = " ".repeat(self.tab_size);
                        response.commands.push(editor::Command::InsertText {
                            buffer_id: self.buffer_id,
                            offset,
                            text: tab_str.clone(),
                        });

                        response.text_changed = true;

                        // Advance cursor by tab_size columns
                        let mut new_pos = cursor.position();
                        new_pos.column += self.tab_size;
                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });
                        response.cursor_moved = true;
                    }
                }

                Key::Enter => {
                    // Insert newline
                    if let Some(cursor) = self.edtr_state.get_cursor_state(self.buffer_id) {
                        let buffer = self.edtr_state.buffers().get(&self.buffer_id).unwrap();
                        let offset = buffer.position_to_offset(cursor.position());

                        response.commands.push(editor::Command::InsertText {
                            buffer_id: self.buffer_id,
                            offset,
                            text: "\n".to_string(),
                        });

                        response.text_changed = true;

                        // Move cursor to start of next line
                        let mut new_pos = cursor.position();
                        new_pos.line += 1;
                        new_pos.column = 0;
                        response.commands.push(editor::Command::MoveCursor {
                            buffer_id: self.buffer_id,
                            position: new_pos,
                        });
                        response.cursor_moved = true;
                    }
                }

                _ => {}
            }
        }
    }
}
