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
//! - **Live Reloading** with non-blocking `notify` event monitoring
//! - **Self-Contained** binary with embedded assets
//! - **RSS & Atom** generation baked-in
//!
//! ## Engine
//! - **Language:** Rust (2024 Edition)
//! - **Markdown:** `pulldown-cmark`
//! - **License:** 3-Clause BSD

mod config;
mod models;
mod parser;
mod rss;
mod build;
mod server;

use clap::{Parser as ClapParser, Subcommand};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Instant};
use std::fs;
use std::path::Path;
use rust_embed::RustEmbed;

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
        port: u16 
    },
    /// Scaffolds a new project in the current directory.
    Init,
}

/// Entry point for the novos engine.
///
/// This function handles the CLI command routing and ensures a `novos.toml`
/// is present for build and serve operations.
fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let cli = Cli::parse();

    if let Commands::Init = cli.command {
        println!("novos init v{}", env!("CARGO_PKG_VERSION"));
        init_project()?;
        println!("\x1b[36msuccess\x1b[0m Project initialized.");
        println!("Done in {:.2}s.", start.elapsed().as_secs_f32());
        return Ok(());
    }

    let cfg_str = fs::read_to_string("novos.toml").map_err(|_| {
        anyhow::anyhow!("\x1b[31mError: novos.toml not found. Run 'novos init' to begin.\x1b[0m")
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
    Ok(())
}

/// Extracts embedded assets to initialize a novos workspace.
///
/// # Errors
///
/// Returns an error if the application lacks write permissions in the current
/// directory or if the embedded asset data is corrupted.
fn init_project() -> anyhow::Result<()> {
    println!("\x1b[2m[1/1]\x1b[0m Extracting default assets...");

    for file in Asset::iter() {
        let path = Path::new(file.as_ref());
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = Asset::get(file.as_ref()).expect("failed to read embedded asset");
        
        if !path.exists() {
            fs::write(path, content.data)?;
        }
    }
    Ok(())
}