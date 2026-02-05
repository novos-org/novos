//! Configuration logic for the novos engine.
//! 
//! This module handles the deserialization of `novos.toml` and provides
//! sane defaults for any missing fields. It is structured into sub-modules
//! (Site, Build, and Social) to keep the configuration file organized.

use serde::Deserialize;
use std::path::PathBuf;

/// The root configuration schema for a novos project.
///
/// This struct represents the entirety of `novos.toml`. If a field is missing,
/// the engine provides defaults to ensures the build can proceed regardless.
#[derive(Deserialize, Clone)]
pub struct Config {
    // --- Directory Settings ---
    
    /// Directory containing blog post Markdown files. Default: `./posts`
    #[serde(default = "default_posts")] 
    pub posts_dir: PathBuf,

    /// Sub-directory within the output folder where posts will be rendered. Default: `""` (root)
    #[serde(default = "default_posts_outdir")]
    pub posts_outdir: String,

    /// Directory containing static page Markdown files. Default: `./pages`
    #[serde(default = "default_pages")] 
    pub pages_dir: PathBuf,

    /// Target directory for the generated static site. Default: `./.build`
    #[serde(default = "default_output")] 
    pub output_dir: PathBuf,

    /// Path to the primary HTML template file. Default: `./index.html`
    #[serde(default = "default_template")] 
    pub template_path: PathBuf,

    /// Directory for reusable HTML fragments and includes. Default: `./includes`
    #[serde(default = "default_includes")] 
    pub includes_dir: PathBuf,

    /// Directory for raw assets (images, fonts, etc.). Default: `./static`
    #[serde(default = "default_static")] 
    pub static_dir: PathBuf,

    /// Path to the specific template used for post/page views. Default: `./includes/view_template.html`
    #[serde(default = "default_view")] 
    pub view_template_path: PathBuf,

    // --- Core Metadata ---

    /// The base domain for absolute link generation (e.g., "https://example.com").
    #[serde(default = "default_url")] 
    pub base_url: String,

    /// The base sub-path if the site is not hosted at the root (e.g., "/blog").
    #[serde(default = "default_base")] 
    pub base: String,

    /// Metadata specific to the site identity (Title, Description, etc.).
    #[serde(default)]
    pub site: SiteMetadata,

    /// Settings that control the behavior of the build engine (Optimization, Minification).
    #[serde(default)]
    pub build: BuildSettings,
}

/// Metadata describing the website for SEO and RSS purposes.
#[derive(Deserialize, Clone, Default)]
pub struct SiteMetadata {
    /// The name of the website.
    #[serde(default = "default_title")]
    pub title: String,
    
    /// A short description of the site for meta tags.
    #[serde(default)]
    pub description: String,
    
    /// The default author name for posts.
    #[serde(default)]
    pub author: String,

    /// Whether to generate an `rss.xml` file.
    #[serde(default = "default_bool_true")]
    pub generate_rss: bool,

    /// Whether to generate a `search.json` index for client-side search.
    #[serde(default = "default_bool_true")]
    pub generate_search: bool,
}

/// Flags and options that tune the build process.
#[derive(Deserialize, Clone, Default)]
pub struct BuildSettings {
    /// If true, the output directory is deleted before every build.
    #[serde(default = "default_bool_true")]
    pub clean_output: bool,

    /// The output style for compiled Sass. Options: "expanded" (default) or "compressed".
    #[serde(default = "default_sass_style")]
    pub sass_style: String,

    /// Attempt to minify the final HTML output.
    #[serde(default = "default_bool_false")]
    pub minify_html: bool,

    /// Toggle for syntax highlighting.
    #[serde(default = "default_bool_true")]
    pub use_syntect: bool,

    /// Syntax highlighting theme for code blocks.
    /// Default: "base16-ocean.dark"
    #[serde(default = "default_theme")]
    pub syntax_theme: String,

    /// Optional directory containing custom .sublime-syntax files.
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

// Boolean helpers for serde defaults
fn default_bool_true() -> bool { true }
fn default_bool_false() -> bool { false }