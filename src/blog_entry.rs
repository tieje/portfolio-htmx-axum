use crate::blog_dir::BlogDir;
use askama::Template;
use axum::extract::Path;
use std::fs;

pub async fn entry_handler(Path(key): Path<String>) -> EntryTemplate {
    EntryTemplate::new(key)
}

#[allow(dead_code)]
#[derive(Template, Default, Clone, Debug)]
#[template(path = "pages/blog_entry.html")]
pub struct EntryTemplate {
    title: String,
    time: String,
    tags: String,
    content: String,
}

impl EntryTemplate {
    pub fn new(link: String) -> Self {
        let blog = BlogDir::new("./data/blog.json");
        let blog_entry = blog.entries.iter().find(|entry| entry.file == link);
        match blog_entry {
            Some(r) => EntryTemplate {
                title: r.title.clone(),
                time: r.time.clone(),
                tags: r.tags.clone().unwrap_or_default().join(", "),
                content: Self::get_content(r.file.as_str()),
            },
            None => EntryTemplate {
                title: "Page not found".to_string(),
                ..Default::default()
            },
        }
    }

    fn get_content(link: &str) -> String {
        let file_path = format!("./data/content/{}.html", link);
        fs::read_to_string(file_path).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "for manual testing only"]
    fn get_content() {
        let test = "building_my_blog_with_htmx_and_axum".to_string();
        let res = EntryTemplate::new(test);
        dbg!(res);
    }
}
