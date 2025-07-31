use anyhow::Result as AnyResult;

use super::commands::editor::Command;
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

    pub fn proccess_frame_commands(&mut self) -> AnyResult<Vec<super::commands::editor::Command>> {
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
