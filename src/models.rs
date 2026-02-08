use std::time::SystemTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub raw_content: String,
    pub mtime: SystemTime,
}
