use std::time::SystemTime;

pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub raw_content: String,
    pub mtime: SystemTime,
}
