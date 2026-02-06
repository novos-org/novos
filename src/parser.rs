use crate::{config::Config, models::Post};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::{collections::HashMap, fs, time::SystemTime};

// Syntect imports
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

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

/// Renders Markdown string to HTML using pulldown-cmark and syntect for code highlighting.
pub fn render_markdown(
    md: &str, 
    use_syntect: bool, 
    ps: &SyntaxSet, 
    theme: &Theme
) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(md, options);

    let mut events = Vec::new();
    let mut temp_code = String::new();
    let mut in_code_block = false;
    let mut current_lang = String::new();

    for event in parser {
        match event {
            // Identify the start of a fenced code block
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(label))) if use_syntect => {
                in_code_block = true;
                current_lang = label.to_string();
                temp_code.clear();
            }
            // Identify the end of the code block (Fixed for pulldown-cmark 0.10+)
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                in_code_block = false;
                
                let syntax = ps
                    .find_syntax_by_token(&current_lang)
                    .unwrap_or_else(|| ps.find_syntax_plain_text());

                let highlighted = highlighted_html_for_string(&temp_code, ps, syntax, theme)
                    .unwrap_or_else(|_| {
                        format!("<pre><code>{}</code></pre>", temp_code)
                    });

                events.push(Event::Html(highlighted.into()));
            }
            // Collect text if inside a code block
            Event::Text(text) => {
                if in_code_block {
                    temp_code.push_str(&text);
                } else {
                    events.push(Event::Text(text));
                }
            }
            // Pass all other events through normally
            _ => {
                if !in_code_block {
                    events.push(event);
                }
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

/// The core engine: recursively resolves {% tags %}, handles variables, 
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

    while let Some(start) = curr.find("{%") {
        output.push_str(&curr[..start]);
        let rem = &curr[start..];

        if let Some(end) = rem.find("%}") {
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
                    "base" => output.push_str(&config.base),
                    "base_url" => output.push_str(&config.base_url),
                    "site_title" => output.push_str(&config.site.title),
                    "site_description" => output.push_str(&config.site.description),
                    "site_author" => output.push_str(&config.site.author),
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

/// Replaces placeholders like {%% 1 %%} with positional arguments.
fn render_shortcode(template: &str, args: &[String]) -> String {
    let mut rendered = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{%% {} %%}}", i + 1);
        rendered = rendered.replace(&placeholder, arg);
    }
    rendered
}

/// Strips Markdown syntax to produce clean plain text for search indexing.
pub fn strip_markdown(md: &str) -> String {
    let parser = Parser::new(md);
    let mut plain_text = String::new();

    for event in parser {
        match event {
            Event::Text(text) | Event::Code(text) => {
                plain_text.push_str(&text);
                plain_text.push(' ');
            }
            _ => {}
        }
    }
    plain_text.trim().to_string()
}