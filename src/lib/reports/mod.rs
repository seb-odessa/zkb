pub mod names;
pub mod killmail;
pub mod victim;
pub mod attacker;
pub mod history;
pub mod system;
pub mod region;
pub mod stargate;
pub mod constellation;
mod item;

use crate::services::{Context, Category, Message, Report};
use std::fmt::Write;


pub use names::Names;
pub use killmail::Killmail;
pub use victim::Victim;
pub use attacker::Attacker;
pub use history::History;
pub use system::System;
pub use region::Region;
pub use stargate::Stargate;
pub use constellation::Constellation;
pub use item::Item;

pub const FAIL: &'static str = "Error occurred while trying to write in String";

#[derive(Debug, PartialEq)]
pub enum ReportType{
    Full,
    Brief,
}

pub fn root(context: &Context) -> String {
    format!("http://{}/navigator", &context.server)
}

pub fn href<S: Into<String>>(url: S, name: S) -> String{
    format!(r#"<a href="{url}">{name}</a>"#, url = url.into(), name = name.into())
}

pub fn div<S: Into<String>>(output: &mut dyn Write, content: S) {
    std::fmt::write(output, format_args!("<div>{}</div>", content.into())).expect(FAIL);
}

pub fn tip<S0: Into<String>, S1: Into<String>>(tip: S0, text: S1) -> String{
    format!(r#"<span title="{}">{}</span>"#, tip.into(), text.into())
}

pub fn jovian_buttons(output: &mut dyn Write, id: &i32, name: &String) {
    std::fmt::write(
        output,
        format_args!(r###"
                <span id="JovianButtons" data-id="{id}" data-name="{name}">
                <span> Jovian Observatory </span>
                <button onclick="registerJovianObservatory()">Register</button>
                <button onclick="unregisterJovianObservatory()">Unregister</button>
                </span>
                <script>
                    function registerJovianObservatory() {{
                        document.getElementById("{id}").style.color = "red";
                    }}

                    function unregisterJovianObservatory() {{
                        document.getElementById("{id}").style.color = "green";
                    }}
                </script>
            "###,
            id=id,
            name=name)
    ).expect(FAIL);
}

pub fn lazy<S: Into<String>>(output: &mut dyn Write, url: S, ctx: &Context) {
    std::fmt::write(
        output,
        format_args!(r##"
        <div id = "{id}">...</div>
        <script>
            fetch("{root}/{api}")
               .then(response => response.text())
               .then(html => document.getElementById("{id}").innerHTML = html)
               .catch((err) => console.log("Can’t access " + "{root}/{api}" + ": " + err));
        </script>"##,
        id=crate::create_id(),
        root=root(ctx),
        api=url.into())
    ).expect(FAIL);
}

pub fn load_later<S: Into<String>>(output: &mut dyn Write, id: &String, api: S, root: &String) {
    std::fmt::write(
        output,
        format_args!(r##"

        <script>
            fetch("{root}/{api}")
               .then(response => response.text())
               .then(text => document.getElementById("{id}").innerText = text)
               .catch((err) => console.log("Can’t access " + "{root}/{api}" + ": " + err));
        </script>"##,
        id=id,
        api=api.into(),
        root=root)
    ).expect(FAIL);
}

pub fn zkb_href(category: &'static str, id: &Option<i32>, name: &Option<String>) -> String {
    format!("<a href=\"https://zkillboard.com/{}/{}/\">{}</a>",
        category,
        id.as_ref().cloned().unwrap_or(0),
        name.as_ref().cloned().unwrap_or_default())
}

pub fn self_href(api: &str, id: &i32, name: &String) -> String {
    format!("<a href=\"{}/{}\">{}</a>", api, id, name)
}



pub fn link_system(id: &i32, name: &String) -> String {
    format!("<a href=\"../system/{}\">{}</a>", id, name)
}

pub fn link_killmail(id: &i32) -> String {
    format!("<a href=\"../killmail/{}\">{}</a>", id, id)
}

pub fn find_id<S: Into<String>>(category: S, name: S, ctx: &Context) -> Option<i32> {
    use crate::services::*;

    let description = (category.into(), name.into());
    if let Report::Id(id) = load(Category::ObjectDesc(description), &ctx) {
        Some(id)
    } else {
        None
    }
}

pub fn load(category: Category, ctx: &Context) -> Report {
    let msg_id = crate::create_id().to_simple();
    ctx.database.push(Message::Find((msg_id, category)));
    while let Some(msg) = ctx.responses.pop() {
        if let Message::Report((id, content)) = msg {
            if id == msg_id {
                return content;
            } else {
                ctx.responses.push(Message::Report((id, content)));
                // Need some sleep here?
            }
        }
    }
    return Report::Fail;
}


