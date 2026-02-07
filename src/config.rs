//! Configuration logic for the novos engine.
//! 
//! This module handles the deserialization of `novos.toml` and provides
//! sane defaults for any missing fields. It is structured into sub-modules
//! (Site, Build, and Social) to keep the configuration file organized.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The root configuration schema for a novos project.
///
/// This struct represents the top-level mapping of the `novos.toml` file.
/// It combines directory paths, site-wide metadata, and engine-specific
/// build settings.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    // --- Directory Settings ---

    /// The name of the theme to use. If [None], the engine assumes a flat 
    /// directory structure or a custom local layout.
    pub theme: Option<String>,

    /// Directory containing blog post Markdown files. 
    /// Defaults to `./posts`.
    #[serde(default = "default_posts")] 
    pub posts_dir: PathBuf,

    /// Sub-directory within the output folder where posts will be rendered. 
    /// Set to an empty string `""` to render posts at the root of the output.
    #[serde(default = "default_posts_outdir")]
    pub posts_outdir: String,

    /// Directory containing static page Markdown files (e.g., About, Contact). 
    /// Defaults to `./pages`.
    #[serde(default = "default_pages")] 
    pub pages_dir: PathBuf,

    /// Target directory for the generated static site. 
    /// Defaults to `./.build`.
    #[serde(default = "default_output")] 
    pub output_dir: PathBuf,

    /// Path to the primary HTML template file (usually the home page). 
    /// Defaults to `./index.html`.
    #[serde(default = "default_template")] 
    pub template_path: PathBuf,

    /// Directory for reusable HTML fragments and includes (e.g., nav, footer). 
    /// Defaults to `./includes`.
    #[serde(default = "default_includes")] 
    pub includes_dir: PathBuf,

    /// Directory for raw assets like images, fonts, and scripts. 
    /// Everything in here is copied directly to the output.
    #[serde(default = "default_static")] 
    pub static_dir: PathBuf,

    /// Path to the specific template used for individual post/page views. 
    /// Defaults to `./includes/view_template.html`.
    #[serde(default = "default_view")] 
    pub view_template_path: PathBuf,

    // --- Core Metadata ---

    /// The base domain for absolute link generation (e.g., "https://example.com").
    /// Essential for RSS feeds and SEO tags.
    #[serde(default = "default_url")] 
    pub base_url: String,

    /// The base sub-path if the site is not hosted at the root (e.g., "/blog").
    /// Useful for GitHub Pages or project sub-directories.
    #[serde(default = "default_base")] 
    pub base: String,

    /// Metadata specific to the site identity (Title, Description, Author, etc.).
    #[serde(default)]
    pub site: SiteMetadata,

    /// Settings that control the behavior of the build engine (Sass, Minification, etc.).
    #[serde(default)]
    pub build: BuildSettings,
}

/// Metadata describing the website for SEO and RSS purposes.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SiteMetadata {
    /// The name of the website, used in `<title>` tags and RSS headers.
    #[serde(default = "default_title")]
    pub title: String,
    
    /// A short description of the site for meta tags and social sharing.
    #[serde(default)]
    pub description: String,
    
    /// The default author name used for posts if not specified in front matter.
    #[serde(default)]
    pub author: String,

    /// Whether to generate an `rss.xml` file in the output directory.
    #[serde(default = "default_bool_true")]
    pub generate_rss: bool,

    /// Whether to generate a `search.json` index for client-side search logic.
    #[serde(default = "default_bool_true")]
    pub generate_search: bool,

    // --- Pagination ---

    /// Toggle to enable or disable pagination for the main post list.
    #[serde(default = "default_bool_false")]
    pub paginate: bool,

    /// Number of posts to show per page if pagination is enabled.
    #[serde(default = "default_posts_per_page")]
    pub posts_per_page: usize,
}

/// Flags and options that tune the build process.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct BuildSettings {
    /// If true, the output directory is wiped before every build to ensure a clean state.
    #[serde(default = "default_bool_true")]
    pub clean_output: bool,

    /// The output style for compiled Sass. 
    /// Options: "expanded" (readable) or "compressed" (optimized).
    #[serde(default = "default_sass_style")]
    pub sass_style: String,

    /// Attempt to minify the final HTML output to save bandwidth.
    #[serde(default = "default_bool_false")]
    pub minify_html: bool,

    /// Toggle for syntax highlighting in code blocks via Syntect.
    #[serde(default = "default_bool_true")]
    pub use_syntect: bool,

    /// Name of the syntax highlighting theme (e.g., "base16-ocean.dark").
    #[serde(default = "default_theme")]
    pub syntax_theme: String,

    /// Path to a custom `.tmTheme` file if a built-in theme isn't used.
    pub syntax_theme_path: Option<PathBuf>,

    /// If enabled, the engine will attempt to convert source images to WebP format.
    #[serde(default = "default_bool_false")]
    pub convert_to_webp: bool,

    /// Optional directory containing custom `.sublime-syntax` files for additional language support.
    pub custom_syntax_dir: Option<PathBuf>,
}

// --- Default value providers ---

fn default_posts() -> PathBuf { PathBuf::from("./posts") }
fn default_posts_outdir() -> String { "".to_string() }
fn default_pages() -> PathBuf { PathBuf::from("./pages") }
fn default_output() -> PathBuf { PathBuf::from("./.build") }
fn default_template() -> PathBuf { PathBuf::from("./index.html") }
fn default_includes() -> PathBuf { PathBuf::from("./includes") }
fn default_static() -> PathBuf { PathBuf::from("./static") }
fn default_view() -> PathBuf { PathBuf::from("./includes/view_template.html") }
fn default_url() -> String { "https://example.com".to_string() }
fn default_base() -> String { "".to_string() }
fn default_title() -> String { "a novos site".to_string() }
fn default_sass_style() -> String { "expanded".to_string() }
fn default_theme() -> String { "base16-ocean.dark".to_string() }
fn default_bool_true() -> bool { true }
fn default_bool_false() -> bool { false }
fn default_posts_per_page() -> usize { 10 }