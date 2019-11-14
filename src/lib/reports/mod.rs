pub mod names;
pub mod killmail;
pub mod history;
pub mod system;
pub mod region;
pub mod stargate;
pub mod constellation;

use crate::services::Context;
use std::fmt::Write;

pub use names::Names;
pub use killmail::Killmail;
pub use history::History;
pub use system::System;
pub use region::Region;
pub use stargate::Stargate;
pub use constellation::Constellation;

pub const FAIL: &'static str = "Error occurred while trying to write in String";

pub fn root(context: &Context) -> String {
    format!("http://{}/navigator", &context.server)
}

pub fn load<S: Into<String>>(url: S, context: &Context) -> String {
    use uuid::Uuid;
    let id = Uuid::new_v4();
    format!(r##"
        <div id="{id}">
        <script>
            document.getElementById("{id}").innerHTML='<object type="text/html" data="{root}/{api}"/>';
        </script>
        </div>"##,
        id=id,
        root=root(&context),
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
    use uuid::Uuid;
    let id = Uuid::new_v4();
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
        id=id,
        root=root(ctx),
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