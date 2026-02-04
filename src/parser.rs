use crate::{config::Config, models::Post};
use pulldown_cmark::{Parser, Options, html};
use std::{collections::HashMap, fs, time::SystemTime};

/// Parses frontmatter from a file and returns a Post struct.
pub fn parse_frontmatter(raw: &str, slug: &str, mtime: SystemTime) -> Post {
let (mut title, mut date, mut tags) = (slug.to_string(), String::new(), Vec::new());
let mut content = raw.to_string();

if raw.starts_with("---") {
let parts: Vec<&str> = raw.splitn(3, "---").collect();
if parts.len() == 3 {
for line in parts[1].lines() {
if let Some((k, v)) = line.split_once(':') {
match k.trim() {
"title" => title = v.trim().trim_matches('"').to_string(),
"date" => date = v.trim().to_string(),
"tags" => {
tags = v
.split(',')
.map(|s| s.trim().trim_matches('"').to_string())
.filter(|s| !s.is_empty())
.collect();
}
_ => {}
}
}
}
content = parts[2].trim().to_string();
}
}

Post {
slug: slug.to_string(),
title,
date,
tags,
raw_content: content,
mtime,
}
}

/// Renders Markdown string to HTML using pulldown-cmark.
pub fn render_markdown(md: &str) -> String {
let mut body = String::new();
html::push_html(&mut body, Parser::new_ext(md, Options::all()));
body
}

/// The core engine: recursively resolves <% tags %>, handles variables, 
/// and processes includes/shortcodes.
pub fn resolve_tags(
    content: &str,
    config: &Config,
    posts_html: &str,
    post: &Post,
    body: Option<&str>,
    depth: u32,
    vars: &mut HashMap<String, String>,
) -> String {
    if depth > 10 {
        return content.to_string();
    }

    let mut output = String::new();
    let mut curr = content;

    while let Some(start) = curr.find("<%") {
        output.push_str(&curr[..start]);
        let rem = &curr[start..];

        if let Some(end) = rem.find("%>") {
            let tag = rem[2..end].trim();

            if tag.starts_with("set ") {
                if let Some((key_part, val_part)) = tag[4..].split_once('=') {
                    vars.insert(key_part.trim().to_string(), val_part.trim().to_string());
                }
            } else if tag.starts_with("print ") {
                let key = tag[6..].trim();
                if let Some(val) = vars.get(key) {
                    output.push_str(val);
                }
            } else {
                match tag {
                    // --- NEW CONFIG TAGS ---
                    "base" => output.push_str(&config.base),
                    "base_url" => output.push_str(&config.base_url),
                    
                    // --- EXISTING TAGS ---
                    "posts" => output.push_str(posts_html),
                    "title" => output.push_str(&post.title),
                    "date" => output.push_str(&post.date),
                    "tags" => {
                        let tags_html = post.tags.iter()
                            .map(|t| format!("<span class=\"tag\">{}</span>", t))
                            .collect::<Vec<_>>().join(" ");
                        output.push_str(&tags_html);
                    }
                    "content" => output.push_str(body.unwrap_or(&post.raw_content)),
                    
                    _ if tag.starts_with("include ") => {
                        let filename = tag[8..].trim();
                        let path = config.includes_dir.join(filename);
                        if let Ok(data) = fs::read_to_string(path) {
                            output.push_str(&resolve_tags(&data, config, posts_html, post, body, depth + 1, vars));
                        }
                    }
                    
                    _ if tag.starts_with('.') => {
                        let mut parts = tag[1..].split_whitespace();
                        if let Some(name) = parts.next() {
                            let args: Vec<String> = parts.map(|s| s.to_string()).collect();
                            let path = config.includes_dir.join("shortcodes").join(format!("{}.html", name));
                            if let Ok(template) = fs::read_to_string(path) {
                                output.push_str(&render_shortcode(&template, &args));
                            }
                        }
                    }

                    _ => {
                        if let Some(val) = vars.get(tag) {
                            output.push_str(val);
                        } else {
                            output.push_str(&rem[..end + 2]);
                        }
                    }
                }
            }
            curr = &rem[end + 2..];
        } else {
            break;
        }
    }
    output.push_str(curr);
    output
}

/// Replaces placeholders like <%= a1 =%> with positional arguments.
fn render_shortcode(template: &str, args: &[String]) -> String {
    let mut rendered = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("<%= a{} =%>", i + 1);
        rendered = rendered.replace(&placeholder, arg);
    }
    rendered
}
