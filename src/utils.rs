use convert_case::{Case, Casing};

pub fn to_portfolio_link(title: String, workplace: String) -> String {
    let title = title.as_str().to_case(Case::Kebab);
    let workplace = workplace.as_str().to_case(Case::Kebab);
    match workplace.is_empty() {
        true => title,
        false => format!("{}-{}", title, workplace),
    }
}
