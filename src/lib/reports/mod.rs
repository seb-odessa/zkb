pub mod names;
pub mod killmail;
pub mod victim;
pub mod attacker;
pub mod history;
pub mod system;
pub mod region;
pub mod stargate;
pub mod constellation;
pub mod network;

mod item;
mod character;
mod corporation;
mod alliance;
mod faction;

use crate::services::{Context, Category, Message, Report, Area};
use crate::models;
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
pub use network::{Node, Edge};


pub const FAIL: &'static str = "Error occurred while trying to write in String";


#[derive(Debug, PartialEq)]
pub enum ReportType{
    Full,
    Brief,
    Hint,
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

    fn hint(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, ReportType::Hint)
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
    let element = format!("<div>{}</div>\n", content.into());
    std::fmt::write(output, format_args!("{}", element)).expect(FAIL);
}

pub fn span<S0: Into<String>, S1: Into<String>, S2: Into<String>>(title: S0, style: S1, content: S2) -> String{
    format!(r#"<span title="{}" style = "{}">{}</span>"#, title.into(), style.into(), content.into())
}

pub fn hidden<S0: Into<String>, S1: Into<String>>(output: &mut dyn Write, id: S0, value: S1) {
    let content = format!("<div hidden id=\"{}\" data-values=\"{}\"></div>\n", id.into(), value.into());
    std::fmt::write(output, format_args!("{}", content)).expect(FAIL);
}

pub fn canvas<S: Into<String>>(output: &mut dyn Write, id: S) {
    std::fmt::write(output,
        format_args!("<div style='display: inline-block;'><canvas id=\"{}\" width='200' height='200'></canvas></div>\n", id.into())
    ).expect(FAIL);
}

pub fn script<S: Into<String>>(output: &mut dyn Write, src: S) {
    std::fmt::write(output, format_args!(r#"<script src="{}"></script>"#, src.into())).expect(FAIL);
}

pub fn write<S: Into<String>>(output: &mut dyn Write, content: S) {
    std::fmt::write(output, format_args!("{}\n", content.into())).expect(FAIL);
}


pub fn table_start<S0: Into<String>, S1: Into<String>, S2: Into<String>>(output: &mut dyn Write, title: S0, style: S1, caption: S2) {
    std::fmt::write(output,format_args!(r#"<table title="{}" style = "{}">"#, title.into(), style.into())).expect(FAIL);
    let caption_content = caption.into();
    if !caption_content.is_empty() {
        std::fmt::write(output,format_args!("<caption>{}</caption>", caption_content)).expect(FAIL);
    }
}

pub fn caption<S: Into<String>>(output: &mut dyn Write, content: S){
    std::fmt::write(output,format_args!(r#"<caption>{}</caption>"#, content.into())).expect(FAIL);
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
            let name = constellation.get_name("constellation");
            let url = span("Constellation", "", ctx.get_api_link("constellation", &name));
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

pub fn get_systems(constellation_id: &i32, ctx: &Context) -> Vec<models::system::SystemNamed> {
    let mut result = Vec::new();
    if let Report::Systems(systems) = load(Category::Systems((Area::Constellation(*constellation_id), models::system::SystemFilter::Any)), &ctx) {
        result = systems;
    }
    return result;
}

pub fn systems(output: &mut dyn Write, constellation_id: &i32, ctx: &Context) {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    let systems = get_systems(constellation_id, ctx);
    for system in &systems {
        let name = system.get_name("system");
        let url = span("Solar System", "", ctx.get_api_link("system", &name));
        map.insert(name, url);
    }
    let mut list = String::new();
    for (_, url) in & map {
        list += url;
        list += " ";
    }
    div(output, format!("Systems in constellation: {}", list));
}

pub fn get_security_status_color(rew_status: f32) -> String {
    let status = (10.0 * rew_status).round() / 10.0;
    // http://web.archive.org/web/20120219150840/http://blog.evepanel.net/eve-online/igb/colors-of-the-security-status.html
    if status >= 1.0      {"#2FEFEF"}
    else if status >= 0.9 {"#48F0C0"}
    else if status >= 0.8 {"#00EF47"}
    else if status >= 0.7 {"#00F000"}
    else if status >= 0.6 {"#8FEF2F"}
    else if status >= 0.5 {"#EFEF00"}
    else if status >= 0.4 {"#D77700"}
    else if status >= 0.3 {"#F06000"}
    else if status >= 0.2 {"#F04800"}
    else if status >= 0.1 {"#D73000"}
    else                  {"#F00000"}
    .to_string()
}

pub fn map<S: Into<String>>(output: &mut dyn Write, id: &i32, deep: u32, uri: S, ctx: &Context) {
    std::fmt::write(
        output,
        format_args!(r##"
            <script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
            <style type="text/css"> #network {{ width: 90%; height: 70%; border: 1px solid lightgray; }} </style>
            <div><span><br/></span></div>
            <div id = "network">...</div>
            <div id = "extinfo"></div>
            <br/>
            <script type="text/javascript">

                const start = async function() {{
                    var nodes = await fetch("{root}/{nodes}").then(response => response.json());
                    var edges = await fetch("{root}/{edges}").then(response => response.json());
                    var nodes_ds = new vis.DataSet(nodes);
                    var edges_ds = new vis.DataSet(edges);
                    var container = document.getElementById('network');
                    var data = {{ nodes: nodes_ds, edges: edges_ds }};
                    var options = {{ clickToUse: true }};
                    var network = new vis.Network(container, data, options);
                    network.on("doubleClick", function(params) {{
                        var url = "{root}/{uri}/" + params.nodes;
                        window.open(url, "_self");
                    }} );
                    network.on("select", function(params) {{
                        var url = "{root}/{uri}_hint/" + params.nodes;
                        fetch(url)
                            .then(response => response.text())
                            .then(html => document.getElementById("extinfo").innerHTML = html)
                            .catch((err) => console.log("Can’t access " + url + ": " + err));
                    }} );
                }}

                start();
            </script>
        "##,
        root=ctx.get_root(),
        nodes=format!("json/nodes/system/{}/{}", id, deep),
        edges=format!("json/edges/system/{}/{}", id, deep),
        uri=uri.into(),
    )).expect(FAIL);
}

pub fn radar(output: &mut dyn Write, ctx: &Context) {
    script(output, ctx.get_js_url("Chart.bundle.min.js")); // https://www.chartjs.org/
    let labels: Vec<String> = (0..24).map(|x| format!("'{}'", x)).collect();
    let script = format!(r#"
        <script>
            function drawRadar(day) {{
                var id = day + "_Radar";
                var id_data = id + "_Values";
                var canvas = document.getElementById(id);
                var values = document.getElementById(id_data).getAttribute('data-values').split(',');
                console.log("id: " + id+ " values: " + values);
                var max = Math.max(...values) + 1;
                var chart = new Chart(canvas.getContext('2d'), {{
                    type: 'radar',
                    data: {{ labels: [{}], datasets: [{{ label: day, data: values }}] }},
                    options: {{ scale: {{ ticks: {{ suggestedMin: 0, suggestedMax: max }} }} }},
                }});
//                chart.canvas.parentNode.style.width = 400;
//                chart.canvas.parentNode.style.height = 400;
            }}

        </script>"#, labels.join(","));
    std::fmt::write(output, format_args!("{}", script)).expect(FAIL);
}

pub fn observer(output: &mut dyn Write, ids: Vec<&str>) {
    let script = format!(r#"
    <script>
        var ids = [{ids}];
        var observer = new MutationObserver(function(mutationsList, me) {{
            mutationsList.forEach((mutation) => {{
                if (mutation.type = 'childList') {{
                    var id = ids.shift();
                    var canvas_id = id + '_Radar';
                    var canvas = window.document.getElementById(canvas_id);
                    if (canvas) {{
                        drawRadar(id);
                    }} else {{
                        ids.push(id);
                    }}
                    console.log("mutation.type:" + mutation.type + " ids: " + ids.toString());
                }}
            }});
            if (0 == ids.length) {{
                me.disconnect();
                return;
            }}
        }});

        observer.observe( document, {{ childList: true, subtree: true }});

    </script>"#, ids = ids.join(","));
    std::fmt::write(output, format_args!("{}", script)).expect(FAIL);
}