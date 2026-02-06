//! High-performance development server for `novos`.
use crate::build::perform_build;
use crate::config::Config;
use anyhow::Result;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use notify::{PollWatcher, Config as WatcherConfig, RecursiveMode, Watcher};
use std::{path::Path, sync::{Arc, Mutex}, time::{SystemTime, Duration}};
use tokio::sync::{broadcast, mpsc};
use tower_http::services::ServeDir;

pub async fn serve(
    config: Config,
    last_run: Arc<Mutex<SystemTime>>,
    port: u16,
    verbose: bool,
) -> Result<()> {
    // 1. Build initial ignore list as owned Strings
    let mut ignore_list = vec![
        ".git".to_string(), 
        "target".to_string(), 
        "#".to_string(), 
        ".swp".to_string()
    ];
    
    if let Ok(gc) = tokio::fs::read_to_string(".gitignore").await {
        for line in gc.lines().map(|l| l.trim()).filter(|l| !l.is_empty() && !l.starts_with('#')) {
            ignore_list.push(line.to_string()); // Convert to owned String
        }
    }

    // 2. Initial build
    perform_build(&config, Arc::clone(&last_run), verbose, true)?;

    let (tx, _rx) = broadcast::channel::<()>(16);
    let (event_tx, mut event_rx) = mpsc::channel::<()>(100);

    // 3. Async Build Worker
    let tx_worker = tx.clone();
    let config_worker = config.clone();
    let lr_worker = Arc::clone(&last_run);
    tokio::spawn(async move {
        while let Some(_) = event_rx.recv().await {
            tokio::time::sleep(Duration::from_millis(150)).await;
            while event_rx.try_recv().is_ok() {}
            if verbose { println!("\x1b[32m[novos] Change detected, rebuilding...\x1b[0m"); }
            if perform_build(&config_worker, Arc::clone(&lr_worker), verbose, true).is_ok() {
                let _ = tx_worker.send(());
            }
        }
    });

    // 4. The PollWatcher (Panic-proof for FreeBSD)
    let watcher_tx = event_tx.clone();
    let watch_config = WatcherConfig::default().with_poll_interval(Duration::from_millis(200));
    
    let mut watcher = PollWatcher::new(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            let is_valid = event.paths.iter().any(|p| {
                let s = p.to_string_lossy();
                let name = p.file_name().unwrap_or_default().to_string_lossy();
                
                let is_ignored = ignore_list.iter().any(|ig| s.contains(ig)) 
                                || name.starts_with('.') 
                                || name.starts_with('#') 
                                || name.ends_with('~');
                !is_ignored
            });

            if is_valid && (event.kind.is_modify() || event.kind.is_create()) {
                let _ = watcher_tx.try_send(());
            }
        }
    }, watch_config)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    // 5. Axum Server
    let app = Router::new()
        .route("/novos/live", get(move |ws: WebSocketUpgrade| {
            let rx = tx.subscribe();
            async move { ws.on_upgrade(|socket| handle_socket(socket, rx)) }
        }))
        .fallback_service(ServeDir::new(&config.output_dir));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("\x1b[33m novos thinking at http://localhost:{}\x1b[0m", port);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<()>) {
    while let Ok(_) = rx.recv().await {
        if socket.send(Message::Text("reload".into())).await.is_err() { break; }
    }
}