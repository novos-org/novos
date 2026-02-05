//! <div align="center">
//!
//! # novos
//!  Build at the speed of thought.
//!
//! </div>
//!
//! ## Features
//! - **Sass transpilation** via native `grass`
//! - **Fast Parallelism** utilizing `Rayon`
//! - **Self-Contained** binary with embedded assets
//! - **RSS & Atom** generation baked-in
//!
//! ## Engine
//! - **Language:** Rust (2024 Edition)
//! - **Markdown:** `pulldown-cmark` (CommonMark compliant, yay!)
//! - **License:** 3-Clause BSD

mod config;
mod models;
mod parser;
mod rss;
mod build;
mod server;

use clap::{Parser as ClapParser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input, Confirm};
use rust_embed::RustEmbed;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime};

/// Assets for the default site template, embedded into the binary.
#[derive(RustEmbed)]
#[folder = "assets/default_site/"]
struct Asset;

/// novos CLI - Build at the speed of thought.
#[derive(ClapParser)]
#[command(author, version, about = "novos - Build at the speed of thought. ")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Compiles the project into a static site.
    Build,
    /// Starts a local server on the specified port.
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Scaffolds a new project.
    Init {
        /// Target directory (defaults to current)
        #[arg(default_value = ".")]
        directory: String,
    },
}

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory } => {
            println!("novos init v{}", env!("CARGO_PKG_VERSION"));
            init_project(&directory)?;
            println!("\n\x1b[36msuccess\x1b[0m Project initialized in '{}'.", directory);
            println!("Done in {:.2}s.", start.elapsed().as_secs_f32());
        }
        _ => {
            // Commands that require novos.toml
            let cfg_str = fs::read_to_string("novos.toml").map_err(|_| {
                anyhow::anyhow!(
                    "\x1b[31mError: novos.toml not found. Run 'novos init' to begin.\x1b[0m"
                )
            })?;

            let config: config::Config = toml::from_str(&cfg_str)?;
            let last_run = Arc::new(Mutex::new(SystemTime::UNIX_EPOCH));

            match cli.command {
                Commands::Build => {
                    build::perform_build(&config, last_run, cli.verbose)?;
                }
                Commands::Serve { port } => {
                    println!("novos serve v{}", env!("CARGO_PKG_VERSION"));
                    println!("\x1b[2m[1/1]\x1b[0m Starting server on port {}...", port);
                    server::serve(config, last_run, port, cli.verbose)?;
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

/// Extracts embedded assets and gathers user configuration via interactive prompts.
fn init_project(target_dir: &str) -> anyhow::Result<()> {
    let base_path = PathBuf::from(target_dir);
    let theme = ColorfulTheme::default();

    // --- Interactive Prompts ---
    let url: String = Input::with_theme(&theme)
        .with_prompt("What is the URL of your site?")
        .default("https://example.net".into())
        .interact_text()?;

    let title: String = Input::with_theme(&theme)
        .with_prompt("Site Title")
        .default("novos example".into())
        .interact_text()?;

    let author: String = Input::with_theme(&theme)
        .with_prompt("Author Name")
        .default("Your Name".into())
        .interact_text()?;

    let gen_rss = Confirm::with_theme(&theme)
        .with_prompt("Enable RSS generation?")
        .default(true)
        .interact()?;

    let gen_search = Confirm::with_theme(&theme)
        .with_prompt("Build search index?")
        .default(true)
        .interact()?;

    let use_sass = Confirm::with_theme(&theme)
        .with_prompt("Enable Sass compilation?")
        .default(true)
        .interact()?;

    let use_syntect = Confirm::with_theme(&theme)
        .with_prompt("Enable syntax highlighting?")
        .default(false)
        .interact()?;

    let clean_out = Confirm::with_theme(&theme)
        .with_prompt("Wipe output folder on every build?")
        .default(true)
        .interact()?;

    println!("\n\x1b[2m[1/2]\x1b[0m Generating novos.toml...");

    let sass_style = if use_sass { "compressed" } else { "expanded" };

    let toml_content = format!(
        r#"base_url = "{url}"
base = ""

posts_dir    = "./posts"
pages_dir    = "./pages"
static_dir   = "./static"
output_dir   = "./.build"
posts_outdir = "posts/"

template_path      = "./index.html"
includes_dir       = "./includes"
view_template_path = "./includes/view_template.html"

[site]
title = "{title}"
description = "A fast, minimal static site generated with Rust."
author = "{author}"
generate_rss = {gen_rss}
generate_search = {gen_search}

[build]
clean_output = {clean_out}
syntax_theme = "InspiredGitHub"
use_syntect = {use_syntect}
sass_style = "{sass_style}"
"#
    );

    if !base_path.exists() {
        fs::create_dir_all(&base_path)?;
    }
    fs::write(base_path.join("novos.toml"), toml_content)?;

    // --- Asset Extraction ---
    println!("\x1b[2m[2/2]\x1b[0m Extracting default assets...");

    for file in Asset::iter() {
        let rel_path = Path::new(file.as_ref());
        let full_path = base_path.join(rel_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = Asset::get(file.as_ref()).expect("failed to read embedded asset");

        if !full_path.exists() {
            fs::write(full_path, content.data)?;
        }
    }

    Ok(())
}