pub mod killmail;
pub mod history;
pub mod system;
pub mod names;



pub use killmail::Killmail;
pub use history::History;
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

pub fn link_killmail(id: &i32) -> String {
    format!("<a href=\"../killmail/{}\">{}</a>", id, id)
}