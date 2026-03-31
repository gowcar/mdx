use std::fs;
use std::path::PathBuf;

const YAZI_PLUGIN_LUA: &str = r#"--- mdx previewer plugin for yazi
--- Renders markdown files with full styling in the preview pane

local M = {}

function M:peek(job)
    local child, err = Command("mdx")
        :arg({ "--raw", "-w", tostring(job.area.w), tostring(job.file.url) })
        :stdout(Command.PIPED)
        :stderr(Command.PIPED)
        :spawn()

    if not child then
        return
    end

    local limit = job.area.h
    local i, lines = 0, ""
    repeat
        local next, event = child:read_line()
        if event ~= 0 then
            break
        end

        i = i + 1
        if i > job.skip then
            lines = lines .. next
        end
    until i >= job.skip + limit

    child:start_kill()

    if job.skip > 0 and i < job.skip + limit then
        ya.emit("peek", { math.max(0, i - limit), only_if = job.file.url, upper_bound = true })
    else
        ya.preview_widget(
            { area = job.area, file = job.file, skip = job.skip },
            ui.Text.parse(lines):area(job.area)
        )
    end
end

function M:seek(job)
    require("code"):seek(job)
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

    // 1. Create plugin directory and write main.lua
    fs::create_dir_all(&plugin_dir)?;
    let lua_path = plugin_dir.join("main.lua");
    fs::write(&lua_path, YAZI_PLUGIN_LUA)?;
    println!("\x1b[32m✓\x1b[0m  Plugin written to {}", lua_path.display());

    // 2. Update yazi.toml — add previewer config
    let toml_content = if yazi_toml.exists() {
        fs::read_to_string(&yazi_toml)?
    } else {
        fs::create_dir_all(&yazi_dir)?;
        String::new()
    };

    let marker = r#"run = "mdx""#;

    if toml_content.contains(marker) {
        println!("\x1b[32m✓\x1b[0m  yazi.toml already configured, skipping.");
    } else {
        // Use both name and mime matching — many systems detect .md as text/plain or text/html
        let previewer_entries = r#"    { url = "*.md", run = "mdx" },
    { mime = "text/markdown", run = "mdx" },"#;

        let snippet = format!(
            "\n[plugin]\nprepend_previewers = [\n{}\n]\n",
            previewer_entries
        );

        if toml_content.contains("[plugin]") {
            // [plugin] section exists — need to inject prepend_previewers
            if toml_content.contains("prepend_previewers") {
                // Already has prepend_previewers array — insert our entries
                let new_content = toml_content.replace(
                    "prepend_previewers = [",
                    &format!("prepend_previewers = [\n{}", previewer_entries),
                );
                fs::write(&yazi_toml, new_content)?;
            } else {
                // Has [plugin] but no prepend_previewers — add after [plugin]
                let new_content = toml_content.replace(
                    "[plugin]",
                    &format!("[plugin]\nprepend_previewers = [\n{}\n]", previewer_entries),
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
