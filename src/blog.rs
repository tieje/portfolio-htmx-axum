use askama::Template;

pub async fn blog_dir_handler() -> BlogDirTemplate {
    
}

#[derive(Debug, Default, Clone)]
pub struct BlogDir {
    entries: Vec<Entry>,
}
#[derive(Debug, Default, Clone)]
pub struct Entry {
    title: String,
    time: String,
    tags: Option<Vec<String>>,
    link: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/blog/blog_dir.html")]
pub struct BlogDirTemplate {
    entries: Vec<String>,
}


#[derive(Template)]
#[template(path = "components/blog/entry.html")]
pub struct EntryTemplate {
    title: String,
    time: String,
    tags: Vec<String>,
    link: String,
}
#[derive(Template)]
#[template(path = "components/blog/footer.html")]
pub struct EntryFooterTemplate {
    link: String,
}
