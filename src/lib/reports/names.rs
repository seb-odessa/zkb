use crate::api;
use crate::services::Context;

use crate::reports::FAIL;

use std::fmt::Write;
use std::fmt::write;


#[derive(Debug, PartialEq)]
pub struct Names {
    name: String,
    names: api::names::Names,
}
impl Names {
    pub fn new(name: &String) -> Option<Self> {
        api::names::Names::new(name).map(|names| Self{ name: name.clone(), names: names })
    }

    fn fmt(output: &mut dyn Write, root: &String, api: &str, items: &Option<Vec<api::names::Item>>) {
        if let Some(items) = items {
            for item in items {
                if api.is_empty() {
                    write(
                        output,
                        format_args!("<div>{} =&gt {}",item.id,item.name)
                    )
                } else {
                    write(
                        output,
                        format_args!(
                            r##"
                            <div>&nbsp;{id}&nbsp;=&gt;&nbsp;[{api}]&nbsp;
                            <a href="{root}/api/{api}/{id}">{name}</a>&nbsp;
                            <a href="https://zkillboard.com/{api}/{id}/">zkb</a></div>"##,
                            root=root,
                            api=api,
                            id=item.id,
                            name=item.name
                        )
                    )
                }.expect(FAIL);
            }
        }
    }

    pub fn report(name: &String, ctx: &Context) -> String {
        let mut output = String::new();
        let root = ctx.get_root();
        write(&mut output, format_args!("<div>&lt{}&gt</div>", name)).expect(FAIL);
        if let Some(name) = Names::new(name) {
            Self::fmt(&mut output, &root, "", &name.names.agents);
            Self::fmt(&mut output, &root, "alliance", &name.names.alliances);
            Self::fmt(&mut output, &root, "character", &name.names.characters);
            Self::fmt(&mut output, &root, "", &name.names.constellations);
            Self::fmt(&mut output, &root, "corporation", &name.names.corporations);
            Self::fmt(&mut output, &root, "", &name.names.factions);
            Self::fmt(&mut output, &root, "item", &name.names.inventory_types);
            Self::fmt(&mut output, &root, "region", &name.names.regions);
            Self::fmt(&mut output, &root, "", &name.names.stations);
            Self::fmt(&mut output, &root, "system", &name.names.systems);
        } else {
            write(&mut output, format_args!("{} not found", name)).expect(FAIL);
        }
        return output;
    }
}
