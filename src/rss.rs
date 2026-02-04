use crate::models::Post;
use crate::config::Config;
use rss::{ChannelBuilder, ItemBuilder, GuidBuilder};
use chrono::{Utc, NaiveDate, TimeZone};

fn format_rss_date(date_str: &str) -> String {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|d| Utc.from_utc_datetime(&d.and_hms_opt(0,0,0).unwrap()).to_rfc2822())
        .unwrap_or_else(|_| Utc::now().to_rfc2822())
}

pub fn generate_rss(posts: &[Post], config: &Config) -> String {
    let items: Vec<_> = posts.iter().take(15).map(|p| {
        let link = format!("{}/{}.html", config.base_url, p.slug);
        ItemBuilder::default()
            .title(Some(p.title.clone()))
            .link(Some(link.clone()))
            .guid(Some(GuidBuilder::default().value(link).build()))
            .description(Some(p.raw_content.chars().take(500).collect()))
            .pub_date(Some(format_rss_date(&p.date)))
            .build()
    }).collect();

    ChannelBuilder::default()
        .title("Novos")
        .link(config.base_url.clone())
        .description("Build at the speed of thought.")
        .items(items)
        .build()
        .to_string()
}
