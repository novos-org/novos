//! The core build engine for `novos`.
//!
//! This module orchestrates the entire static site generation (SSG) pipeline.
//! It leverages Rust's concurrency model via `Rayon` for high-performance 
//! transformation of Markdown and Sass into optimized, minified web assets.

use crate::{config::Config, parser, rss, models::Post};
use rayon::prelude::*;
use serde_json::{json, Value};
use minify_html::{minify, Cfg};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Instant, SystemTime},
};

// High-performance syntax highlighting
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

// Media processing and text manipulation
use webp::Encoder;
use regex::Regex;

/// Injected script for the `novos serve` command.
/// Provides a lightweight WebSocket client to trigger browser refreshes.
const LIVE_RELOAD_SCRIPT: &str = r#"
<script id="novos-live-reload">
    (function() {
        const socket = new WebSocket('ws://' + window.location.host + '/novos/live');
        socket.onmessage = (event) => {
            if (event.data === 'reload') {
                console.log('novos: Change detected, reloading...');
                window.location.reload();
            }
        };
        socket.onclose = () => console.log('novos: Live reload disconnected.');
    })();
</script>
"#;

/// Internal helper to render an index file.
/// This centralizes the logic for both the main homepage and paginated sub-pages.
fn render_index_file(
    tera: &tera::Tera,
    ctx: &tera::Context,
    dest: PathBuf,
    config: &Config,
    is_dev: bool
) -> io::Result<()> {
    // Attempt to render the 'index.html' template
    match tera.render("index.html", ctx) {
        Ok(rendered) => {
            // Apply minification and dev-scripts
            let final_html = process_html(rendered, config.build.minify_html, is_dev);
            fs::write(dest, final_html)
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("Tera Error (index.html): {}", e))),
    }
}

/// Resolves a path by checking the project root first, falling back to the active theme.
/// 
/// This enables "Theme Overriding": users can customize specific files in their project
/// without modifying the shared theme directory.
fn resolve_path(relative_path: &str, theme_dir: &Option<PathBuf>) -> PathBuf {
    let project_path = Path::new(relative_path);
    if project_path.exists() {
        return project_path.to_path_buf();
    }
    if let Some(t_dir) = theme_dir {
        let theme_path = t_dir.join(relative_path);
        if theme_path.exists() {
            return theme_path;
        }
    }
    project_path.to_path_buf()
}

/// Minifies HTML and optionally injects the live-reload WebSocket script.
/// 
/// Uses `minify-html` for extremely fast, spec-compliant minification.
fn process_html(mut html: String, should_minify: bool, is_dev: bool) -> String {
    if is_dev {
        // Inject script before the closing body tag or at the end
        if let Some(pos) = html.find("</body>") {
            html.insert_str(pos, LIVE_RELOAD_SCRIPT);
        } else {
            html.push_str(LIVE_RELOAD_SCRIPT);
        }
    }

    if !should_minify {
        return html;
    }

    let mut cfg = Cfg::new();
    cfg.minify_js = true;
    cfg.minify_css = true;
    cfg.keep_comments = false;
     
    let minified = minify(html.as_bytes(), &cfg);
    String::from_utf8(minified).unwrap_or(html)
}

/// Standard recursive directory copy.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Regex-based rewriter to swap image extensions in HTML/CSS for WebP.
/// 
/// Respects external URLs unless they match the local `base_url`.
fn rewrite_to_webp(content: String, base_url: &str) -> String {
    let file_re = Regex::new(
        r#"(?i)(["'\(\s])([^"'\)\s?#]+\.)(?:png|jpe?g)(["'\)\s])"#
    ).unwrap();

    let content = file_re.replace_all(&content, |caps: &regex::Captures| {
        let pre = &caps[1];
        let path = &caps[2];
        let post = &caps[3];

        let is_external = path.contains("://") || path.starts_with("//");
        let is_our_domain = path.starts_with(base_url);

        if !is_external || is_our_domain {
            format!("{}{}webp{}", pre, path, post)
        } else {
            caps[0].to_string()
        }
    }).into_owned();

    let mime_re = Regex::new(r#"(?i)image/(?:png|jpeg)"#).unwrap();
    mime_re.replace_all(&content, "image/webp").into_owned()
}

/// Scans the output directory and converts all PNG/JPG assets to WebP.
/// 
/// Runs in parallel to utilize multi-core systems.
fn process_images(config: &Config, verbose: bool) -> io::Result<()> {
    let output_dir = &config.output_dir;
    
    let mut image_paths = Vec::new();
    let walker = walkdir::WalkDir::new(output_dir);
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ["jpg", "jpeg", "png"].contains(&ext_lower.as_str()) {
                image_paths.push(path.to_path_buf());
            }
        }
    }

    image_paths.into_par_iter().for_each(|path| {
        if let Ok(img) = image::open(&path) {
            let encoder = Encoder::from_image(&img).unwrap();
            let webp_data = encoder.encode(75.0); 
            
            let mut webp_path = path.clone();
            webp_path.set_extension("webp");
            
            if fs::write(&webp_path, &*webp_data).is_ok() {
                if verbose {
                    println!("\x1b[2m  optimized\x1b[0m {}", path.file_name().unwrap().to_str().unwrap());
                }
                let _ = fs::remove_file(path);
            }
        }
    });

    Ok(())
}

/// Compiles SCSS/SASS to CSS using the pure-Rust `grass` compiler.
pub fn compile_sass(config: &Config, theme_dir: &Option<PathBuf>, verbose: bool) -> io::Result<()> {
    let sass_dir = resolve_path("sass", theme_dir);
    if !sass_dir.exists() {
        return Ok(());
    }

    let css_dir = config.output_dir.join("css");
    fs::create_dir_all(&css_dir)?;

    let style = match config.build.sass_style.as_str() {
        "compressed" => grass::OutputStyle::Compressed,
        _ => grass::OutputStyle::Expanded,
    };
    
    let mut options = grass::Options::default().style(style);
    if let Some(td) = theme_dir {
        options = options.load_path(td.join("sass"));
    }

    for entry in fs::read_dir(sass_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "scss" || ext == "sass") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            
            // Skip partials (files starting with underscore)
            if file_name.starts_with('_') {
                continue;
            }

            if verbose {
                println!("\x1b[2m  compiling\x1b[0m {}", file_name);
            }

            match grass::from_path(&path, &options) {
                Ok(mut css) => {
                    let mut out_path = css_dir.join(file_name);
                    out_path.set_extension("css");
                    
                    if config.build.convert_to_webp {
                        css = rewrite_to_webp(css, &config.base_url);
                    }

                    fs::write(out_path, css)?;
                }
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("Sass Error: {}", e)));
                }
            }
        }
    }
    Ok(())
}

/// Loads all TOML/JSON files from `data/` and `themes/<name>/data/`.
fn load_data_dir(theme_dir: &Option<PathBuf>) -> Value {
    let mut data_map = serde_json::Map::new();
    
    let data_sources = [
        theme_dir.as_ref().map(|d| d.join("data")),
        Some(PathBuf::from("data")),
    ];

    for path in data_sources.into_iter().flatten() {
        if path.exists() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let p = entry.path();
                    if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                        let stem = p.file_stem().unwrap().to_str().unwrap().to_string();
                        let content = fs::read_to_string(&p).unwrap_or_default();

                        let val: Value = match ext {
                            "toml" => toml::from_str(&content).unwrap_or(json!({})),
                            "json" => serde_json::from_str(&content).unwrap_or(json!({})),
                            _ => continue,
                        };
                        data_map.insert(stem, val);
                    }
                }
            }
        }
    }
    Value::Object(data_map)
}

/// The orchestrator of the `novos` build.
pub fn perform_build(
    config: &Config,
    last_run_mu: Arc<Mutex<SystemTime>>,
    verbose: bool,
    is_dev: bool,
) -> io::Result<()> {
    let start = Instant::now();
    let lr = *last_run_mu.lock().unwrap();

    let theme_dir = config.theme.as_ref().map(|t| PathBuf::from("themes").join(t));

    // --- STEP 0: TEMPLATE INITIALIZATION ---
    let mut tera = parser::init_tera("templates"); 
    if let Some(td) = &theme_dir {
        let theme_templates_glob = td.join("templates/**/*");
        if let Some(glob_str) = theme_templates_glob.to_str() {
            if let Ok(theme_tera) = tera::Tera::new(glob_str) {
                // Merge theme templates into our Tera instance
                tera.extend(&theme_tera).ok();
            }
        }
    }

    // --- STEP 1: ASSET PIPELINE ---
    if config.build.clean_output {
        if verbose { println!("\x1b[2m[1/5]\x1b[0m Cleaning output directory..."); }
        if config.output_dir.exists() {
            let _ = fs::remove_dir_all(&config.output_dir);
        }
    }
    
    fs::create_dir_all(&config.output_dir)?;
    let posts_out_path = config.output_dir.join(&config.posts_outdir);
    fs::create_dir_all(&posts_out_path)?;

    // Copy static assets: Theme first, then Project (Project overrides Theme)
    if let Some(td) = &theme_dir {
        let theme_static = td.join("static");
        if theme_static.exists() {
            copy_dir_all(theme_static, &config.output_dir)?;
        }
    }
    if config.static_dir.exists() {
        copy_dir_all(&config.static_dir, &config.output_dir)?;
    }

    if config.build.convert_to_webp {
        if verbose { println!("\x1b[2m[1.5/5]\x1b[0m Optimizing images..."); }
        process_images(config, verbose)?;
    }

    // --- STEP 2: STYLESHEETS ---
    if verbose { println!("\x1b[2m[2/5]\x1b[0m Compiling stylesheets..."); }
    compile_sass(config, &theme_dir, verbose)?;

    // --- STEP 3: CONTENT INGESTION ---
    if verbose { println!("\x1b[2m[3/5]\x1b[0m Processing content..."); }
    
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax_theme = ts.themes.get(&config.build.syntax_theme)
        .cloned()
        .unwrap_or_else(|| ts.themes.get("base16-ocean.dark").unwrap().clone());

    let global_data = load_data_dir(&theme_dir);

    let mut post_paths = Vec::new();
    if config.posts_dir.exists() {
        for e in fs::read_dir(&config.posts_dir)? {
            let p = e?.path();
            if p.extension().map(|s| s == "md").unwrap_or(false) {
                post_paths.push(p);
            }
        }
    }

    // Parallel parsing of Markdown and Frontmatter
    let mut posts: Vec<Post> = post_paths
        .into_par_iter()
        .map(|p| {
            let mt = fs::metadata(&p).and_then(|m| m.modified()).unwrap_or(lr);
            let raw = fs::read_to_string(&p).unwrap_or_default();
            parser::parse_frontmatter(&raw, p.file_stem().unwrap().to_str().unwrap(), mt)
        })
        .collect();

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    // --- STEP 4: TAXONOMY AGGREGATION ---
    let mut taxonomies: HashMap<String, Vec<&Post>> = HashMap::new();
    for post in &posts {
        for tag in &post.tags {
            taxonomies.entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(post);
        }
    }
// --- STEP 5: RENDERING ---
    if verbose { println!("\x1b[2m[4/5]\x1b[0m Rendering posts and taxonomies..."); }

    // [Detailed Comment: Post Rendering]
    // We use .par_iter() to render every markdown post into an HTML file.
    // Each post gets the full 'posts' list and 'taxonomies' map in its context,
    // allowing for "Related Posts" or "Tag Cloud" features inside post.html.
    posts.par_iter().for_each(|p| {
        let dest = posts_out_path.join(format!("{}.html", p.slug));
        if p.mtime > lr || !dest.exists() {
            let body = parser::render_markdown(&p.raw_content, config.build.use_syntect, &ps, &syntax_theme);
            
            let mut context = tera::Context::new();
            context.insert("post", p);
            context.insert("posts", &posts); // All posts for navigation
            context.insert("taxonomies", &taxonomies);
            context.insert("data", &global_data);
            context.insert("config", config);
            context.insert("content", &body);

            if let Ok(rendered) = tera.render("post.html", &context) {
                let mut final_html = process_html(rendered, config.build.minify_html, is_dev);
                if config.build.convert_to_webp {
                    final_html = rewrite_to_webp(final_html, &config.base_url);
                }
                fs::write(dest, final_html).ok();
            }
        }
    });

    // [Detailed Comment: Taxonomy Rendering]
    // Here we create the tags pages. If you don't have a 'tag.html', we use 'index.html'.
    // CRITICAL FIX: We pass the 'tagged_posts' as 'posts' so that the index template
    // loops over the filtered list instead of the global post list.
    if !taxonomies.is_empty() {
        let tax_out_dir = config.output_dir.join("tags");
        fs::create_dir_all(&tax_out_dir)?;

        taxonomies.par_iter().for_each(|(tag, tagged_posts)| {
            let mut tax_ctx = tera::Context::new();
            // We provide 'tag' so the template can display "Posts tagged with: Rust"
            tax_ctx.insert("tag", tag); 
            // We overwrite 'posts' with ONLY the posts containing this tag
            tax_ctx.insert("posts", tagged_posts); 
            tax_ctx.insert("config", config);
            tax_ctx.insert("data", &global_data);

            // Determine which template to use. Tera registry check ensures no crash.
            let template = if tera.get_template_names().any(|t| t == "tag.html") { 
                "tag.html" 
            } else { 
                "index.html" 
            };
            
            if let Ok(rendered) = tera.render(template, &tax_ctx) {
                let mut final_html = process_html(rendered, config.build.minify_html, is_dev);
                // Filenames are lowercased for URL consistency (e.g., tags/rust.html)
                let dest = tax_out_dir.join(format!("{}.html", tag.to_lowercase()));
                fs::write(dest, final_html).ok();
            }
        });
    }

    // --- STEP 6: INDEX & PAGINATION ---
    if verbose { println!("\x1b[2m[5/5]\x1b[0m Finalizing indices and metadata..."); }

    // [Detailed Comment: The Pagination Engine]
    // If pagination is enabled, we chunk the 'posts' vector.
    // Each chunk becomes a separate index.html file in a numbered directory.
    if config.site.paginate && config.site.posts_per_page > 0 {
        let chunks: Vec<_> = posts.chunks(config.site.posts_per_page).collect();
        let total_pages = chunks.len();

        for (i, chunk) in chunks.iter().enumerate() {
            let current_page = i + 1;
            let mut idx_ctx = tera::Context::new();
            
            // Context variables for the index template to handle "Next/Prev" links
            idx_ctx.insert("posts", chunk);
            idx_ctx.insert("total_pages", &total_pages);
            idx_ctx.insert("current_page", &current_page);
            idx_ctx.insert("has_prev", &(current_page > 1));
            idx_ctx.insert("has_next", &(current_page < total_pages));
            idx_ctx.insert("data", &global_data);
            idx_ctx.insert("config", config);
            
            // Map the output path:
            // Page 1 -> /index.html
            // Page 2 -> /page/2/index.html
            let out_path = if current_page == 1 {
                config.output_dir.join("index.html")
            } else {
                let page_dir = config.output_dir.join("page").join(current_page.to_string());
                fs::create_dir_all(&page_dir)?;
                page_dir.join("index.html")
            };
            
            render_index_file(&tera, &idx_ctx, out_path, config, is_dev)?;
        }
    } else {
        // [Detailed Comment: Single Index Fallback]
        // If pagination is OFF, we still need a homepage! 
        // We render all posts to the root index.html.
        let mut idx_ctx = tera::Context::new();
        idx_ctx.insert("posts", &posts);
        idx_ctx.insert("data", &global_data);
        idx_ctx.insert("config", config);
        
        let out_path = config.output_dir.join("index.html");
        render_index_file(&tera, &idx_ctx, out_path, config, is_dev)?;
    }
    // --- STEP 7: EXTERNAL DISTRIBUTIONS ---
    if config.site.generate_rss {
        let rss_xml = rss::generate_rss(&posts, config);
        fs::write(config.output_dir.join("rss.xml"), rss_xml)?;
    }

    if config.site.generate_search {
        let search_index: Vec<serde_json::Value> = posts.iter().map(|p| {
            let clean_text = parser::strip_markdown(&p.raw_content);
            let snippet: String = clean_text.chars().take(140).collect();
            json!({ 
                "title": p.title, 
                "slug": p.slug, 
                "date": p.date, 
                "tags": p.tags, 
                "snippet": snippet 
            })
        }).collect();
        fs::write(config.output_dir.join("search.json"), serde_json::to_string(&search_index)?)?;
    }

    // Update global build timestamp for the next run's differential check
    if let Ok(mut lr_lock) = last_run_mu.lock() {
        *lr_lock = SystemTime::now();
    }
    
    if verbose {
        println!("\x1b[36msuccess\x1b[0m build complete in {:.2}s.", start.elapsed().as_secs_f32());
    }
    Ok(())
}