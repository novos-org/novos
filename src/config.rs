use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_posts")] pub posts_dir: PathBuf,
    #[serde(default = "default_pages")] pub pages_dir: PathBuf,
    #[serde(default = "default_output")] pub output_dir: PathBuf,
    #[serde(default = "default_template")] pub template_path: PathBuf,
    #[serde(default = "default_includes")] pub includes_dir: PathBuf,
    #[serde(default = "default_static")] pub static_dir: PathBuf,
    #[serde(default = "default_view")] pub view_template_path: PathBuf,
    #[serde(default = "default_url")] pub base_url: String,
    #[serde(default = "default_base")] pub base: String,
}

fn default_posts() -> PathBuf { PathBuf::from("./posts") }
fn default_pages() -> PathBuf { PathBuf::from("./pages") }
fn default_output() -> PathBuf { PathBuf::from("./.build") }
fn default_template() -> PathBuf { PathBuf::from("./index.html") }
fn default_includes() -> PathBuf { PathBuf::from("./includes") }
fn default_static() -> PathBuf { PathBuf::from("./static") }
fn default_view() -> PathBuf { PathBuf::from("./includes/view_template.html") }
fn default_url() -> String { "https://example.com".to_string() }
fn default_base() -> String { "".to_string() }
