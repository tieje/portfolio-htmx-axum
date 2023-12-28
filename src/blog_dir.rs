use askama::Template;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

pub async fn blog_dir_handler() -> BlogDirTemplate {
    BlogDir::new("./data/blog.json").serialize()
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct BlogDir {
    pub entries: Vec<Entry>,
}
impl BlogDir {
    pub fn new(file_path: &str) -> Self {
        let mut json_data = String::new();
        let _ = File::open(file_path)
            .expect("could not open file")
            .read_to_string(&mut json_data)
            .expect("could not parse json");
        let res: Self = serde_json::from_str(&json_data).expect("could not parse json");
        res
    }
    fn serialize(&self) -> BlogDirTemplate {
        let mut res = BlogDirTemplate {
            entries: Vec::new(),
        };
        for entry in self.entries.iter() {
            let new_entry = BlogDirEntryTemplate {
                title: entry.title.clone(),
                time: entry.time.clone(),
                tags: entry.tags.clone().unwrap_or_default(),
                link: format!("entries/{}", entry.file),
            };
            res.entries.push(new_entry);
        }
        res
    }
}

#[derive(Template, Serialize, Default, Clone, Debug)]
#[template(path = "pages/blog_dir.html")]
pub struct BlogDirTemplate {
    entries: Vec<BlogDirEntryTemplate>,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Entry {
    pub title: String,
    pub time: String,
    pub tags: Option<Vec<String>>,
    pub file: String,
}
#[derive(Template, Serialize, Default, Clone, Debug)]
#[template(path = "components/blog_dir/entry.html")]
pub struct BlogDirEntryTemplate {
    title: String,
    time: String,
    tags: Vec<String>,
    link: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "output is too long"]
    fn parse_local_json() {
        let blog = BlogDir::new("./data/blog.json");
        dbg!(blog);
    }
}
