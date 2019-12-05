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
mod character;
mod corporation;
mod alliance;
mod faction;

use crate::services::{Context, Category, Message, Report, Area};
use std::fmt::Write;


pub use names::Names;
pub use killmail::Killmail;
pub use victim::Victim;
pub use attacker::Attacker;
pub use history::History;
pub use system::System;
pub use region::Region;
pub use constellation::Constellation;
pub use item::Item;
pub use character::Character;
pub use corporation::Corporation;
pub use alliance::Alliance;
pub use faction::Faction;


pub const FAIL: &'static str = "Error occurred while trying to write in String";

#[derive(Debug, PartialEq)]
pub enum ReportType{
    Full,
    Brief,
}

pub trait Reportable {
    fn report(category: &String, arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx)
        } else if let Some(ref id) = find_id(category, arg, ctx) {
            Self::report_by_id(id, ctx)
        } else {
            format!("<div>{} {} was not found</div>", category, arg)
        }
    }

    fn report_by_id(id: &i32, ctx: &Context) -> String;
}

pub trait ReportableEx {
    fn get_category() -> String;

    fn brief(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, ReportType::Brief)
    }

    fn report(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, ReportType::Full)
    }

    fn perform_report(arg: &String, ctx: &Context, report_type: ReportType) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx, report_type)
        } else if let Some(ref id) = find_id(&Self::get_category(), arg, ctx) {
            Self::report_by_id(id, ctx, report_type)
        } else {
            format!("<div>{} {} was not found</div>", Self::get_category(), arg)
        }
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: ReportType) -> String;
}

pub fn href<S: Into<String>>(url: S, name: S) -> String{
    format!(r#"<a href="{url}">{name}</a>"#, url = url.into(), name = name.into())
}

pub fn div<S: Into<String>>(output: &mut dyn Write, content: S) {
    std::fmt::write(output, format_args!("<div>{}</div>", content.into())).expect(FAIL);
}

pub fn span<S0: Into<String>, S1: Into<String>, S2: Into<String>>(title: S0, style: S1, content: S2) -> String{
    format!(r#"<span title="{}" style = "{}">{}</span>"#, title.into(), style.into(), content.into())
}


pub fn table_start<S0: Into<String>, S1: Into<String>, S2: Into<String>>(output: &mut dyn Write, title: S0, style: S1, caption: S2) {
    std::fmt::write(output,format_args!(r#"<table title="{}" style = "{}">"#, title.into(), style.into())).expect(FAIL);
    let caption_content = caption.into();
    if !caption_content.is_empty() {
        std::fmt::write(output,format_args!("<caption>{}</caption>", caption_content)).expect(FAIL);
    }
}

pub fn table_cell<S0: Into<String>, S1: Into<String>, S2: Into<String>>(output: &mut dyn Write, title: S0, style: S1, content: S2){
    std::fmt::write(output,format_args!(r#"<td title="{}" style = "{}">{}</td>"#, title.into(), style.into(), content.into())).expect(FAIL);
}

pub fn table_cell_head<S0: Into<String>, S1: Into<String>, S2: Into<String>>(output: &mut dyn Write, title: S0, style: S1, content: S2){
    std::fmt::write(output,format_args!(r#"<th title="{}" style = "{}">{}</th>"#, title.into(), style.into(), content.into())).expect(FAIL);
}

pub fn table_row_start<S0: Into<String>>(output: &mut dyn Write, style: S0) {
    std::fmt::write(output,format_args!(r#"<tr style = "{}">"#, style.into())).expect(FAIL);
}

pub fn table_row_end(output: &mut dyn Write) {
    std::fmt::write(output,format_args!("</tr>")).expect(FAIL);
}

pub fn table_end(output: &mut dyn Write, ) {
    std::fmt::write(output,format_args!("</table>")).expect(FAIL);
}

pub fn tip<S0: Into<String>, S1: Into<String>>(tip: S0, content: S1) -> String{
    format!(r#"<span title="{}">{}</span>"#, tip.into(), content.into())
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
        root=ctx.get_root(),
        api=url.into())
    ).expect(FAIL);
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
    use std::{thread, time};
    let msg_id = crate::create_id().to_simple();
    ctx.database.push(Message::Find((msg_id, category)));
    loop {
        while let Some(msg) = ctx.responses.pop() {
            if let Message::Report((id, content)) = msg {
                if id == msg_id {
                    return content;
                } else {
                    ctx.responses.push(Message::Report((id, content)));
                    thread::sleep(time::Duration::from_millis(20));
                }
            }
        }
    }
}

pub fn constellations(output: &mut dyn Write, region_id: &i32, ctx: &Context) {
        use std::collections::BTreeMap;
        if let Report::Constellations(constellations) = load(Category::Constellations(Area::Region(*region_id)), &ctx) {
            let mut map = BTreeMap::new();
            for constellation in &constellations {
                let id = constellation.get_id("constellation");
                let name = constellation.get_name("constellation");
                let url = span("Constellation", "", ctx.get_api_href("constellation", id, &name));
                map.insert(name, url);
            }
            let mut list = String::new();
            for (_, url) in &map {
                list += url;
                list += " ";
            }
            div(output, format!("Constellation in Region: {}", list));
        }
    }

pub fn systems(output: &mut dyn Write, constellation_id: &i32, ctx: &Context) {
        use crate::models::system::*;
        use std::collections::BTreeMap;
        if let Report::Systems(systems) = load(Category::Systems((Area::Constellation(*constellation_id), SystemFilter::Any)), &ctx) {
            let mut map = BTreeMap::new();
            for system in &systems {
                let id = system.get_id("system");
                let name = system.get_name("system");
                let url = span("Solar System", "", ctx.get_api_href("system", id, &name));
                map.insert(name, url);
            }
            let mut list = String::new();
            for (_, url) in & map {
                list += url;
                list += " ";
            }
            div(output, format!("Systems in constellation: {}", list));
        }
    }