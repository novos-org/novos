use serde::Deserialize;
use std::collections::HashMap;
use std::time::SystemTime;
use crate::models::Post;

#[derive(Debug, Deserialize, Default)]
struct Metadata {
    title: Option<String>,
    date: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    // This "flattens" any other keys into a map
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>, 
}


pub fn parse_frontmatter(raw: &str, slug: &str, mtime: SystemTime) -> Post {
    let mut title = slug.to_string();
    let mut date = String::new();
    let mut tags = Vec::new();
    let mut extra = HashMap::new();
    let mut content = raw.to_string();

    let (delimiter, is_toml) = if raw.starts_with("+++") {
        ("+++", true)
    } else if raw.starts_with("---") {
        ("---", false)
    } else {
        ("", false)
    };

    if !delimiter.is_empty() {
        let parts: Vec<&str> = raw.splitn(3, delimiter).collect();
        if parts.len() == 3 {
            let fm_str = parts[1];
            content = parts[2].trim().to_string();

            let res: Result<Metadata, _> = if is_toml {
                // We use toml -> yaml value conversion or just use a generic Value type
                toml::from_str(fm_str).map_err(|_| ())
            } else {
                serde_yaml::from_str(fm_str).map_err(|_| ())
            };

            if let Ok(m) = res {
                if let Some(t) = m.title { title = t; }
                if let Some(d) = m.date { date = d; }
                tags = m.tags;
                extra = m.extra;
            }
        }
    }

    Post {
        slug: slug.to_string(),
        title,
        date,
        tags,
        extra,
        raw_content: content,
        mtime,
    }
}