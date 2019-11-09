pub mod killmail;
pub mod history;
pub mod system;
pub mod region;
pub mod names;
pub mod constellation;

use crate::services::Context;
use std::fmt::Write;

pub use names::Names;
pub use killmail::Killmail;
pub use history::History;
pub use system::System;
pub use region::Region;
pub use constellation::Constellation;

pub const FAIL: &'static str = "Error occurred while trying to write in String";

pub fn get_root(context: &Context) -> String {
    format!("http://{}/navigator", &context.server)
}

pub fn load<S: Into<String>>(url: S, context: &Context) -> String {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    format!(r##"
        <div id ="{id}"/>
        <script>
            document.getElementById("{id}").innerHTML='<object type="text/html" data="{root}/{api}"/>';
        </script>"##,
        id=id,
        root=get_root(&context),
        api=url.into())
}

pub fn href<S: Into<String>>(url: S, name: S) -> String{
    format!(r#"<a href="{url}">{name}</a>"#, url = url.into(), name = name.into())
}

pub fn div<S: Into<String>>(output: &mut dyn Write, name: S, value: S) {
    std::fmt::write(
        output,
        format_args!("<div>{name}: {value}</div>", name = name.into(), value = value.into())
    ).expect(FAIL);
}

pub fn lazy<S: Into<String>>(output: &mut dyn Write, url: S, ctx: &Context) {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    std::fmt::write(
        output,
        format_args!(r##"
        <div id ="{id}"/>
        <script>
            document.getElementById("{id}").innerHTML='<object type="text/html" data="{root}/{api}"/>';
        </script>"##,
        id=id,
        root=get_root(ctx),
        api=url.into())
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