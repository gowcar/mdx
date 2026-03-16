use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Default)]
#[allow(dead_code)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub status_bar: StatusBarConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GeneralConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub mouse: bool,
    #[serde(default = "default_true")]
    pub wrap: bool,
    #[serde(default = "default_max_width")]
    pub max_width: u16,
    #[serde(default = "default_padding")]
    pub padding: u16,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DisplayConfig {
    #[serde(default)]
    pub line_numbers_in_code: bool,
    #[serde(default = "default_true")]
    pub show_language_label: bool,
    #[serde(default = "default_true")]
    pub rounded_code_blocks: bool,
    #[serde(default = "default_true")]
    pub table_zebra_stripes: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct StatusBarConfig {
    #[serde(default = "default_true")]
    pub show_time: bool,
    #[serde(default = "default_true")]
    pub show_percentage: bool,
    #[serde(default = "default_true")]
    pub show_keyhints: bool,
    #[serde(default = "default_true")]
    pub show_theme_name: bool,
    #[serde(default = "default_time_format")]
    pub time_format: String,
    #[serde(default = "default_progress_style")]
    pub progress_style: String,
}

#[derive(Deserialize, Default)]
#[allow(dead_code)]
pub struct KeybindingsConfig {
    #[serde(default)]
    pub quit: Option<Vec<String>>,
    #[serde(default)]
    pub scroll_up: Option<Vec<String>>,
    #[serde(default)]
    pub scroll_down: Option<Vec<String>>,
    #[serde(default)]
    pub page_up: Option<Vec<String>>,
    #[serde(default)]
    pub page_down: Option<Vec<String>>,
    #[serde(default)]
    pub full_page_up: Option<Vec<String>>,
    #[serde(default)]
    pub full_page_down: Option<Vec<String>>,
    #[serde(default)]
    pub top: Option<Vec<String>>,
    #[serde(default)]
    pub bottom: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<Vec<String>>,
    #[serde(default)]
    pub search_next: Option<Vec<String>>,
    #[serde(default)]
    pub search_prev: Option<Vec<String>>,
    #[serde(default)]
    pub help: Option<Vec<String>>,
}

fn default_theme() -> String {
    "catppuccin".to_string()
}
fn default_true() -> bool {
    true
}
fn default_max_width() -> u16 {
    120
}
fn default_padding() -> u16 {
    2
}
fn default_time_format() -> String {
    "%H:%M".to_string()
}
fn default_progress_style() -> String {
    "both".to_string()
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            mouse: true,
            wrap: true,
            max_width: default_max_width(),
            padding: default_padding(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            line_numbers_in_code: false,
            show_language_label: true,
            rounded_code_blocks: true,
            table_zebra_stripes: true,
        }
    }
}

impl Default for StatusBarConfig {
    fn default() -> Self {
        Self {
            show_time: true,
            show_percentage: true,
            show_keyhints: true,
            show_theme_name: true,
            time_format: default_time_format(),
            progress_style: default_progress_style(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Warning: Failed to parse config: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to read config: {}", e);
                }
            }
        }
        Self::default()
    }

    pub fn config_dir() -> PathBuf {
        // Prefer ~/.config/mdx on all platforms for simplicity
        if let Some(home) = dirs::home_dir() {
            home.join(".config").join("mdx")
        } else {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("mdx")
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn init_config() -> std::io::Result<()> {
        let dir = Self::config_dir();
        fs::create_dir_all(&dir)?;
        let path = Self::config_path();
        if path.exists() {
            eprintln!("Config already exists at: {}", path.display());
            return Ok(());
        }
        fs::write(&path, DEFAULT_CONFIG)?;
        println!("Config created at: {}", path.display());
        Ok(())
    }
}

const DEFAULT_CONFIG: &str = r#"[general]
theme = "catppuccin"
mouse = true
wrap = true
max_width = 120
padding = 2

[display]
line_numbers_in_code = false
show_language_label = true
rounded_code_blocks = true
table_zebra_stripes = true

[status_bar]
show_time = true
show_percentage = true
show_keyhints = true
show_theme_name = true
time_format = "%H:%M"
progress_style = "both"   # "bar" | "percentage" | "both"

[keybindings]
# Each action maps to an array of key strings
# quit = ["q", "Ctrl-c"]
# scroll_up = ["k", "Up"]
# scroll_down = ["j", "Down"]
# page_up = ["Ctrl-u", "PageUp"]
# page_down = ["Ctrl-d", "PageDown"]
# full_page_up = ["b", "Ctrl-b"]
# full_page_down = ["f", "Ctrl-f", "Space"]
# top = ["g g", "Home"]
# bottom = ["G", "End"]
# search = ["/"]
# search_next = ["n"]
# search_prev = ["N"]
# help = ["?", "F1"]
"#;
