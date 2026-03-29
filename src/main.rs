mod app;
mod config;
mod event;
mod render;
mod scroll;
mod search;
mod selection;
mod setup;
mod theme;
mod ui;
mod watcher;
mod widgets;

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::Parser;

use config::Config;

#[derive(Parser)]
#[command(name = "mdx", version, about = "A beautiful terminal markdown viewer")]
struct Cli {
    /// Markdown file to view (reads from stdin if not specified)
    file: Option<PathBuf>,

    /// Theme: dracula, catppuccin, nord, tokyo-night, gruvbox, solarized, one-dark, monokai
    #[arg(long)]
    theme: Option<String>,

    /// Raw mode: render to stdout (for use as previewer in yazi/fzf/etc.)
    #[arg(long)]
    raw: bool,

    /// Render width in raw mode (defaults to terminal width or 80)
    #[arg(short, long)]
    width: Option<u16>,

    /// Set up mdx as yazi's markdown previewer
    #[arg(long)]
    setup_yazi: bool,

    /// Initialize default config file
    #[arg(long)]
    init_config: bool,

    /// Show config file path
    #[arg(long)]
    config_path: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.setup_yazi {
        if let Err(e) = setup::setup_yazi() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if cli.init_config {
        if let Err(e) = Config::init_config() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if cli.config_path {
        println!("{}", Config::config_path().display());
        return;
    }

    let (content, file_path) = if let Some(ref file) = cli.file {
        let c = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: Failed to read {}: {}", file.display(), e);
                std::process::exit(1);
            }
        };
        (c, file.clone())
    } else {
        // Try reading from stdin
        let is_tty = atty_check();
        if is_tty {
            eprintln!("Error: No file specified. Usage: mdx <file.md>");
            eprintln!("       Or pipe content: echo '# Hello' | mdx");
            std::process::exit(1);
        }
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_default();
        (buf, PathBuf::from("stdin"))
    };

    let config = Config::load();

    // Set nerd_font env for detection if configured explicitly
    match config.general.nerd_font.as_str() {
        "true" => unsafe { std::env::set_var("MDX_NERD_FONT", "1") },
        "false" => unsafe { std::env::set_var("MDX_NERD_FONT", "0") },
        _ => {} // "auto" - let has_nerd_font() detect
    }

    // Priority: CLI flag > last saved theme > config file
    let theme_name = cli.theme
        .or_else(Config::load_last_theme)
        .unwrap_or(config.general.theme.clone());
    let theme = theme::Theme::by_name(&theme_name);

    if cli.raw {
        let width = cli.width.unwrap_or_else(|| {
            crossterm::terminal::size().map(|(w, _)| w).unwrap_or(80)
        });
        let text = render::render_markdown(&content, width, &theme);
        print!("{}", render::text_to_ansi(&text));
        return;
    }

    if let Err(e) = app::run(content, file_path, theme) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn atty_check() -> bool {
    std::io::IsTerminal::is_terminal(&io::stdin())
}
