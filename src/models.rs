use std::time::SystemTime;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub raw_content: String,
    pub mtime: SystemTime,
}

#[derive(Debug, serde::Deserialize)]
pub struct ThemeConfig {
    pub theme: ThemeMetadata,
}

#[derive(Debug, serde::Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub syntax_theme: Option<String>,
}

pub struct Theme {
    pub root: PathBuf,
    pub config: ThemeMetadata,
}