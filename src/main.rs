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
use rust_embed::RustEmbed;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime};
use std::io::{self, Write, Cursor};
use syntect::highlighting::{Theme, ThemeSet};

/// Assets for the default site template, embedded into the binary.
#[derive(RustEmbed)]
#[folder = "assets/default_site/"]
struct Asset;

/// Assets for the bare/minimal site template, embedded into the binary.
#[derive(RustEmbed)]
#[folder = "assets/blank_site/"]
struct BlankAsset;

/// Load a custom .tmTheme file for syntect.
pub fn load_custom_theme(path: &std::path::Path) -> Theme {
    let theme_file = fs::read_to_string(path)
        .expect("Failed to read .tmTheme file");
    let mut cursor = Cursor::new(theme_file);
    ThemeSet::load_from_reader(&mut cursor)
        .expect("Failed to parse .tmTheme")
}

/// novos CLI - Build at the speed of thought.
#[derive(ClapParser)]
#[command(author, version, about = "novos - Build at the speed of thought. ")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Enable verbose logging output.
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Compiles the project into a static site.
    Build,
    /// Starts a local server with live-reloading.
    #[command(alias = "server")]
    Serve {
        /// Port to listen on.
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
     /// Scaffolds a new project.

     Init {
        /// Target directory (defaults to current)
        #[arg(default_value = ".")]
        directory: String,
        /// Skip prompts and use a minimal, blank template.
        #[arg(short, long)]
        bare: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { directory, bare } => {
            println!("novos init v{}", env!("CARGO_PKG_VERSION"));
            init_project(&directory, bare)?;
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
                    // is_dev is false for standard builds
                    build::perform_build(&config, last_run, cli.verbose, false)?;
                    println!("\x1b[32msuccess\x1b[0m Build complete in {:.2}s.", start.elapsed().as_secs_f32());
                }
                Commands::Serve { port } | Commands::Serve { port } => {
                    println!("novos serve v{}", env!("CARGO_PKG_VERSION"));
                    println!("\x1b[2m[1/1]\x1b[0m Starting server on port {}...", port);
                    
                    // We pass 'true' for is_dev to enable the live-reload script injection
                    server::serve(config, last_run, port, cli.verbose).await?;
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

/// Helper to read input with a default value.
fn prompt_input(message: &str, default: &str) -> io::Result<String> {
    print!("{message} ({default}): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// Helper to handle [Y/n] or [y/N] logic.
fn prompt_confirm(message: &str, default_yes: bool) -> io::Result<bool> {
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{message} {suffix}: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();

    if trimmed.is_empty() {
        return Ok(default_yes);
    }

    match trimmed.as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        _ => Ok(default_yes),
    }
}

/// Extracts embedded assets from the binary into the target directory.
fn extract_assets<E: RustEmbed>(base_path: &Path) -> anyhow::Result<()> {
    for file in E::iter() {
        let rel_path = Path::new(file.as_ref());
        let full_path = base_path.join(rel_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = E::get(file.as_ref()).expect("failed to read embedded asset");

        if !full_path.exists() {
            fs::write(full_path, content.data)?;
        }
    }
    Ok(())
}

/// Extracts embedded assets and gathers user configuration via raw stdout/stdin.
fn init_project(target_dir: &str, bare: bool) -> anyhow::Result<()> {
    let base_path = PathBuf::from(target_dir);

    // Initial Defaults
    let mut url = "https://example.com".to_string();
    let mut title = "novos site".to_string();
    let mut author = "admin".to_string();
    let mut use_sass = true;
    let mut use_syntect = true;
    let mut gen_search = true;
    let mut gen_rss = true;
    let mut clean_out = true;
    let mut convert_to_webp = false;
    let mut minify = false;

    // --- Interactive Prompts ---
    if !bare {
        url = prompt_input("What is the URL of your site?", &url)?;
        title = prompt_input("Site Title", &title)?;
        author = prompt_input("Author Name", &author)?;
        // FIXED: Correctly assign to convert_to_webp
        convert_to_webp = prompt_confirm("Do you want to convert image assets to WebP?", true)?;
        // FIXED: Renamed 'use_' to 'use_sass'
        use_sass = prompt_confirm("Do you want to enable Sass compilation?", true)?;
        use_syntect = prompt_confirm("Do you want to enable syntax highlighting?", true)?;
        gen_search = prompt_confirm("Do you want to build a search index?", true)?;
        gen_rss = prompt_confirm("Do you want to generate an RSS feed?", true)?;
        clean_out = prompt_confirm("Clean output directory before build?", true)?;
        minify = prompt_confirm("Minify HTML output?", true)?;
    } else {
        use_sass = false;
        use_syntect = false;
        gen_search = false;
        gen_rss = false;
        convert_to_webp = false;
    }

    println!("\n\x1b[2m[1/2]\x1b[0m Generating novos.toml...");

    // Optimization: If sass is disabled, we might want to default to expanded 
    // or just leave it, but 'compressed' is a good default for prod.
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
minify_html = {minify}
convert_to_webp = {convert_to_webp}
syntax_theme = "base16-ocean.dark"
use_syntect = {use_syntect}
sass_style = "{sass_style}"
"#
    );

    if !base_path.exists() {
        fs::create_dir_all(&base_path)?;
    }
    fs::write(base_path.join("novos.toml"), toml_content)?;

    println!("\x1b[2m[2/2]\x1b[0m Extracting assets...");

    if bare {
        extract_assets::<BlankAsset>(&base_path)?;
    } else {
        extract_assets::<Asset>(&base_path)?;
    }

    Ok(())
}