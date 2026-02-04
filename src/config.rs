//! Configuration logic for the novos engine.
//! 
//! This module handles the deserialization of `novos.toml` and provides
//! sane defaults for any missing fields.

use serde::Deserialize;
use std::path::PathBuf;

/// The configuration schema for a novos project.
///
/// If a field is missing from `novos.toml`, the engine uses the defaults
/// listed in the table below.
///
/// | Field | Default Value | Description |
/// | :--- | :--- | :--- |
/// | `posts_dir` | `./posts` | Markdown files for blog entries |
/// | `pages_dir` | `./pages` | Markdown files for standalone pages |
/// | `output_dir` | `./.build` | Destination for the generated site |
/// | `template_path` | `./index.html` | The main layout template |
/// | `includes_dir` | `./includes` | Partial templates and shortcodes |
/// | `static_dir` | `./static` | Assets copied directly to output |
/// | `base_url` | `https://example.com` | The production URL for RSS/Sitemaps |
#[derive(Deserialize, Clone)]
pub struct Config {
    /// Directory containing blog post Markdown files.
    #[serde(default = "default_posts")] 
    pub posts_dir: PathBuf,

    /// Directory containing static page Markdown files.
    #[serde(default = "default_pages")] 
    pub pages_dir: PathBuf,

    /// Target directory for the generated static site.
    #[serde(default = "default_output")] 
    pub output_dir: PathBuf,

    /// Path to the primary HTML template file.
    #[serde(default = "default_template")] 
    pub template_path: PathBuf,

    /// Directory for reusable HTML fragments and includes.
    #[serde(default = "default_includes")] 
    pub includes_dir: PathBuf,

    /// Directory for raw assets (images, fonts, etc.).
    #[serde(default = "default_static")] 
    pub static_dir: PathBuf,

    /// Path to the specific template used for post/page views.
    #[serde(default = "default_view")] 
    pub view_template_path: PathBuf,

    /// The base domain for absolute link generation.
    #[serde(default = "default_url")] 
    pub base_url: String,

    /// The base sub-path if the site is not hosted at the root.
    #[serde(default = "default_base")] 
    pub base: String,
}

// --- Default value providers ---

fn default_posts() -> PathBuf { PathBuf::from("./posts") }
fn default_pages() -> PathBuf { PathBuf::from("./pages") }
fn default_output() -> PathBuf { PathBuf::from("./.build") }
fn default_template() -> PathBuf { PathBuf::from("./index.html") }
fn default_includes() -> PathBuf { PathBuf::from("./includes") }
fn default_static() -> PathBuf { PathBuf::from("./static") }
fn default_view() -> PathBuf { PathBuf::from("./includes/view_template.html") }
fn default_url() -> String { "https://example.com".to_string() }
fn default_base() -> String { "".to_string() }