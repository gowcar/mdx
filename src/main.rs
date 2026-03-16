mod app;
mod event;
mod render;
mod scroll;
mod theme;
mod ui;
mod widgets;

use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "mdx", version, about = "A beautiful terminal markdown viewer")]
struct Cli {
    /// Markdown file to view
    file: PathBuf,

    /// Theme name (catppuccin, dracula, nord, tokyo-night)
    #[arg(long, default_value = "catppuccin")]
    theme: String,
}

fn main() {
    let cli = Cli::parse();

    let content = match fs::read_to_string(&cli.file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read {}: {}", cli.file.display(), e);
            std::process::exit(1);
        }
    };

    let theme = theme::Theme::by_name(&cli.theme);
    if let Err(e) = app::run(content, cli.file, theme) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
