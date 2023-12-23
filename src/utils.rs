pub fn to_link(title: String, workplace: String) -> String {
    let title = title.replace([' ', '-'], "");
    let workplace = workplace.replace([' ', '-'], "");
    match workplace.is_empty() {
        true => title,
        false => format!("{}-{}", title, workplace),
    }
}
