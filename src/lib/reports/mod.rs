pub mod killmail;
pub mod system;
pub mod names;

pub use killmail::Killmail;
pub use system::System;
pub use names::Names;


pub fn zkb_href(category: &'static str, id: &Option<i32>, name: &Option<String>) -> String {
    format!("<a href=\"https://zkillboard.com/{}/{}/\">{}</a>", 
        category, 
        id.as_ref().cloned().unwrap_or(0), 
        name.as_ref().cloned().unwrap_or_default())
}

pub fn link_system(id: &i32, name: &String) -> String {
    format!("<a href=\"../system/{}\">{}</a>", id, name)
}