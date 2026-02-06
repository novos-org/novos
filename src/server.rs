use crate::build::perform_build;
use crate::config::Config;
use notify::{recommended_watcher, RecursiveMode, Result as NotifyResult};
use notify::Watcher;
use std::{fs, path::Path, sync::{Arc, Mutex}, time::SystemTime};
use tiny_http;
use anyhow::Result;

/// Serve the generated site and watch the working directory for changes.
pub fn serve(
    config: Config,
    last_run: Arc<Mutex<SystemTime>>,
    port: u16,
    verbose: bool,
) -> Result<()> {
    // initial build
    perform_build(&config, Arc::clone(&last_run), verbose)?;

    // watcher: rebuild on modify events
    let config_c = config.clone();
    let lr_c = Arc::clone(&last_run);
    let verb = verbose;

    // recommended_watcher returns Result<impl Watcher, notify::Error>
    let mut watcher = recommended_watcher(move |res: NotifyResult<notify::Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                // best-effort rebuild; ignore errors here
                let _ = perform_build(&config_c, Arc::clone(&lr_c), verb);
            }
        }
    })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    // server
    let addr = format!("0.0.0.0:{}", port);
    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    println!("\x1b[33m Thinking at http://0.0.0.0:{}\x1b[0m", port);

    for request in server.incoming_requests() {
        let url: &str = request.url().split('?').next().unwrap_or("/");
        let f_path = if url == "/" { "index.html".into() } else { url[1..].to_string() };
        let mut full = config.output_dir.join(&f_path);

        if !full.exists() && !f_path.contains('.') {
            let alt = config.output_dir.join(format!("{}.html", f_path));
            if alt.exists() { full = alt; }
        }

 let response = match fs::read(&full) {
    Ok(d) => {
        // Extract extension from the actual file path being served
        let extension = full.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let ct = match extension {
            "css"  => "text/css",
            "js"   => "application/javascript",
            "json" => "application/json",
            "xml"  => "application/xml",
            "png"  => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "svg"  => "image/svg+xml",
            "txt"  => "text/plain",
            _      => "text/html; charset=utf-8", // Default for .html or extensionless
        };

        tiny_http::Response::from_data(d).with_header(
            tiny_http::Header::from_bytes(&b"Content-Type"[..], ct.as_bytes()).unwrap()
        )
    },
    Err(_) => tiny_http::Response::from_string("404 - no philosophies found").with_status_code(404),
};
let _ = request.respond(response);
    }

    Ok(())
}
