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

#[derive(RustEmbed)]
#[folder = "assets/default_site/"]
struct Asset;

#[derive(ClapParser)]
#[command(author, version, about = "Novos - Build at the speed of thought. ")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Build,
    Serve { #[arg(short, long, default_value_t = 8080)] port: u16 },
    Init,
}

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
            // All step logic and "Done" messaging now happens inside perform_build
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