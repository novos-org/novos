use crate::config::Config;
use crate::models::Post;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::{time::SystemTime};
use tera::{Context, Tera};

// Syntect imports
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Initializes the Tera engine. 
/// It's best to call this once at the start of your program.
pub fn init_tera(template_dir: &str) -> Tera {
    let mut tera = match Tera::new(&format!("{}/**/*", template_dir)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Tera parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    // We disable auto-escaping because we are injecting pre-rendered 
    // HTML from pulldown-cmark and syntect.
    tera.autoescape_on(vec![]);
    tera
}

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
    theme: &Theme,
) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(md, options);

    let mut events = Vec::new();
    let mut temp_code = String::new();
    let mut in_code_block = false;
    let mut current_lang = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(label))) if use_syntect => {
                in_code_block = true;
                current_lang = label.to_string();
                temp_code.clear();
            }
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
            Event::Text(text) => {
                if in_code_block {
                    temp_code.push_str(&text);
                } else {
                    events.push(Event::Text(text));
                }
            }
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

/// The core engine: uses Tera to render the final HTML.
/// Replaces the old recursive 'resolve_tags' logic.
pub fn render_template(
    tera: &Tera,
    template_name: &str,
    post: &Post,
    config: &Config,
    posts_list_html: &str,
    rendered_content: &str,
) -> String {
    let mut context = Context::new();
    
    // Inject data into the template context
    context.insert("post", post);
    context.insert("config", config);
    context.insert("content", rendered_content);
    context.insert("posts", posts_list_html);
    
    // Helper variables for cleaner template access
    context.insert("site_title", &config.site.title);
    context.insert("base_url", &config.base_url);

    match tera.render(template_name, &context) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error rendering {}: {}", template_name, e);
            format!("Rendering Error: {}", e)
        }
    }
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