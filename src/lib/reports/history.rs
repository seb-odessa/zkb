use crate::models::*;
use crate::reports;
use crate::services::Context;
use crate::services::server::root;

#[derive(Debug, PartialEq)]
pub struct History;
impl History {
    pub fn report(system: &Integer, minutes: &Integer, ctx: &Context) -> String {
        use crate::services::*;

        let mut output = String::new();
        let root = root(&ctx);
        let msg_id = crate::create_id().to_simple();
        ctx.database.push(Message::Find((msg_id, Category::History((*system, *minutes)))));
        while let Some(msg) = ctx.responses.pop() {
            if let Message::Report((report_id, ref report)) = msg {
                if report_id == msg_id {
                    match report {
                        Report::History(history) => {
                            for killmail in history {
                                reports::Killmail::write(&mut output, killmail, &root);
                            }
                        },
                        report => {
                            warn!("Unexpected report {:?}", report);
                        }
                    }
                    break;
                } else {
                   ctx.responses.push(msg);
                }
            }
        }
        return output;
    }
}

