use std::fs;
use std::path::PathBuf;

const YAZI_PLUGIN_LUA: &str = r#"--- mdx previewer plugin for yazi
--- Renders markdown files with full styling in the preview pane

local M = {}

function M:peek(job)
    local child = Command("mdx")
        :args({ "--raw", "-w", tostring(job.area.w), tostring(job.file.url) })
        :stdout(Command.PIPED)
        :stderr(Command.PIPED)
        :spawn()

    if not child then
        return
    end

    local output = child:wait_with_output()
    if not output or not output.status or not output.status.success then
        return
    end

    local lines = {}
    for line in output.stdout:gmatch("[^\n]*") do
        table.insert(lines, ui.Line.parse(line))
    end

    local offset = job.skip or 0
    local visible = {}
    for i = offset + 1, math.min(#lines, offset + job.area.h) do
        table.insert(visible, lines[i])
    end

    ya.preview_widgets(job, { ui.Text(visible):area(job.area) })
end

function M:seek(job)
    local h = cx.active.current.hovered
    if h then
        local step = job.units > 0 and 1 or -1
        ya.manager_emit("peek", {
            math.max(0, cx.active.preview.skip + step),
            only_if = h.url,
        })
    end
end

return M
"#;

fn yazi_config_dir() -> PathBuf {
    // Yazi follows XDG convention on all platforms (including macOS)
    std::env::var("YAZI_CONFIG_HOME")
        .or_else(|_| std::env::var("XDG_CONFIG_HOME").map(|p| format!("{}/yazi", p)))
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".config")
                .join("yazi")
        })
}

pub fn setup_yazi() -> Result<(), Box<dyn std::error::Error>> {
    // Check if yazi is installed
    let yazi_found = std::process::Command::new("which")
        .arg("yazi")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !yazi_found {
        eprintln!("\x1b[33m⚠\x1b[0m  yazi not found in PATH.");
        eprintln!("   Install yazi first: https://yazi-rs.github.io/docs/installation");
        std::process::exit(1);
    }

    let yazi_dir = yazi_config_dir();
    let plugin_dir = yazi_dir.join("plugins").join("mdx.yazi");
    let yazi_toml = yazi_dir.join("yazi.toml");

    // 1. Create plugin directory and write init.lua
    fs::create_dir_all(&plugin_dir)?;
    let lua_path = plugin_dir.join("init.lua");
    fs::write(&lua_path, YAZI_PLUGIN_LUA)?;
    println!("\x1b[32m✓\x1b[0m  Plugin written to {}", lua_path.display());

    // 2. Update yazi.toml — add previewer config
    let toml_content = if yazi_toml.exists() {
        fs::read_to_string(&yazi_toml)?
    } else {
        fs::create_dir_all(&yazi_dir)?;
        String::new()
    };

    let marker = r#"{ mime = "text/markdown", run = "mdx" }"#;

    if toml_content.contains(marker) {
        println!("\x1b[32m✓\x1b[0m  yazi.toml already configured, skipping.");
    } else {
        let snippet = format!(
            "\n[plugin]\nprepend_previewers = [\n    {{ mime = \"text/markdown\", run = \"mdx\" }},\n]\n"
        );

        if toml_content.contains("[plugin]") {
            // [plugin] section exists — need to inject prepend_previewers
            if toml_content.contains("prepend_previewers") {
                // Already has prepend_previewers array — insert our entry
                let new_content = toml_content.replace(
                    "prepend_previewers = [",
                    &format!("prepend_previewers = [\n    {{ mime = \"text/markdown\", run = \"mdx\" }},"),
                );
                fs::write(&yazi_toml, new_content)?;
            } else {
                // Has [plugin] but no prepend_previewers — add after [plugin]
                let new_content = toml_content.replace(
                    "[plugin]",
                    "[plugin]\nprepend_previewers = [\n    { mime = \"text/markdown\", run = \"mdx\" },\n]",
                );
                fs::write(&yazi_toml, new_content)?;
            }
        } else {
            // No [plugin] section — append
            let mut new_content = toml_content;
            new_content.push_str(&snippet);
            fs::write(&yazi_toml, new_content)?;
        }

        println!("\x1b[32m✓\x1b[0m  Updated {}", yazi_toml.display());
    }

    println!();
    println!("  \x1b[1mYazi integration complete!\x1b[0m");
    println!("  Markdown files will now preview with mdx in yazi's preview pane.");
    println!("  Open yazi and navigate to any .md file to try it out.");

    Ok(())
}
