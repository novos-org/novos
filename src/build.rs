use crate::{config::Config, parser, rss, models::Post};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
    time::{Instant, SystemTime},
};

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

pub fn compile_sass(config: &Config, verbose: bool) -> io::Result<()> {
    let sass_dir = Path::new("sass");
    if !sass_dir.exists() {
        return Ok(());
    }

    let css_dir = config.output_dir.join("css");
    fs::create_dir_all(&css_dir)?;

    for entry in fs::read_dir(sass_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "scss" || ext == "sass") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            
            if file_name.starts_with('_') {
                continue;
            }

            if verbose {
                println!("\x1b[2m  compiling\x1b[0m {}", file_name);
            }

            match grass::from_path(&path, &grass::Options::default()) {
                Ok(css) => {
                    let mut out_path = css_dir.join(file_name);
                    out_path.set_extension("css");
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

pub fn perform_build(
    config: &Config,
    last_run_mu: Arc<Mutex<SystemTime>>,
    verbose: bool,
) -> io::Result<()> {
    let start = Instant::now();
    let lr = *last_run_mu.lock().unwrap();

    // Yarn Header
    println!("Novos build v{}", env!("CARGO_PKG_VERSION"));

    // Step 1
    println!("\x1b[2m[1/4]\x1b[0m Cleaning output directory...");
    if config.output_dir.exists() {
        let _ = fs::remove_dir_all(&config.output_dir);
    }
    fs::create_dir_all(&config.output_dir)?;

    if config.static_dir.exists() {
        if verbose {
            println!("\x1b[2m  copying assets\x1b[0m");
        }
        copy_dir_all(&config.static_dir, &config.output_dir)?;
    }

    // Step 2
    println!("\x1b[2m[2/4]\x1b[0m Compiling stylesheets...");
    compile_sass(config, verbose)?;

    // Step 3
    println!("\x1b[2m[3/4]\x1b[0m Processing content...");
    
    let main_tpl = fs::read_to_string(&config.template_path)?;
    let view_tpl = fs::read_to_string(&config.view_template_path).unwrap_or_else(|_| "<% content %>".to_string());

    let mut post_paths = Vec::new();
    if config.posts_dir.exists() {
        for e in fs::read_dir(&config.posts_dir)? {
            let p = e?.path();
            if p.extension().map(|s| s == "md").unwrap_or(false) {
                post_paths.push(p);
            }
        }
    }

    let mut posts: Vec<Post> = post_paths
        .into_par_iter()
        .map(|p| {
            let mt = fs::metadata(&p).and_then(|m| m.modified()).unwrap_or(lr);
            let raw = fs::read_to_string(&p).unwrap_or_default();
            parser::parse_frontmatter(&raw, p.file_stem().unwrap().to_str().unwrap(), mt)
        })
        .collect();

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let mut posts_html = String::from("<ul class='post-list'>\n");
    for p in &posts {
        posts_html.push_str(&format!(
	"  <li>{} - <a href='{}{}'>{}</a></li>\n",
       p.date, config.base, p.slug, p.title
       ));
    }
    posts_html.push_str("</ul>");

    // Step 4
    println!("\x1b[2m[4/4]\x1b[0m Rendering pages...");

    posts.par_iter().for_each(|p| {
        let dest = config.output_dir.join(format!("{}.html", p.slug));
        if p.mtime > lr || !dest.exists() {
            let mut vars = HashMap::new();
            let body = parser::render_markdown(&p.raw_content);
            let layout = parser::resolve_tags(&view_tpl, config, &posts_html, p, Some(&body), 0, &mut vars);
            let final_h = parser::resolve_tags(&main_tpl, config, &posts_html, p, Some(&layout), 0, &mut vars);
            fs::write(dest, final_h).ok();
        }
    });

    let rss_xml = rss::generate_rss(&posts, config);
    fs::write(config.output_dir.join("rss.xml"), rss_xml)?;

    if config.pages_dir.exists() {
        let mut page_paths = Vec::new();
        for e in fs::read_dir(&config.pages_dir)? {
            let p = e?.path();
            if p.extension().map(|s| s == "html").unwrap_or(false) {
                page_paths.push(p);
            }
        }
        page_paths.into_par_iter().for_each(|p| {
            let slug = p.file_stem().unwrap().to_str().unwrap();
            let raw = fs::read_to_string(&p).unwrap_or_default();
            let mt = fs::metadata(&p).and_then(|m| m.modified()).unwrap_or(lr);
            let pd = parser::parse_frontmatter(&raw, slug, mt);
            
            let mut vars = HashMap::new();
            let pb = parser::resolve_tags(&pd.raw_content, config, &posts_html, &pd, None, 0, &mut vars);
            let fh = parser::resolve_tags(&main_tpl, config, &posts_html, &pd, Some(&pb), 0, &mut vars);
            
            fs::write(config.output_dir.join(format!("{}.html", slug)), fh).ok();
        });
    }

    // Home
    let index_meta = Post {
        slug: "index".to_string(),
        title: "home".to_string(),
        date: "".to_string(),
        tags: vec![],
        raw_content: "<% include home.html %>".to_string(),
        mtime: SystemTime::now(),
    };
    
    let mut index_vars = HashMap::new();
    let index_body = parser::resolve_tags(&index_meta.raw_content, config, &posts_html, &index_meta, None, 0, &mut index_vars);
    let index_page = parser::resolve_tags(&main_tpl, config, &posts_html, &index_meta, Some(&index_body), 0, &mut index_vars);
    fs::write(config.output_dir.join("index.html"), index_page)?;

    *last_run_mu.lock().unwrap() = SystemTime::now();
    
    println!("\x1b[36msuccess\x1b[0m Build complete.");
    println!("Done in {:.2}s.", start.elapsed().as_secs_f32());
    Ok(())
}