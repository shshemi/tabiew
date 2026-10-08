use crate::{app::App, ctl::message::Message, handler, misc::sql::sql};

pub struct Reply<'a> {
    msg: Message,
    app: Option<&'a App>,
}

impl<'a> Reply<'a> {
    pub fn new(msg: Message) -> Self {
        Self { msg, app: None }
    }
    pub fn with_app(self, app: &'a App) -> Self {
        Self {
            app: Some(app),
            ..self
        }
    }

    pub fn send(self) {
        if let Some(rp) = handle(self.msg, self.app) {
            rp.send();
        }
    }
}

fn handle(msg: Message, _app: Option<&App>) -> Option<Message> {
    match msg {
        Message::Ps => Some(Message::PsReply {
            pid: std::process::id(),
        }),

        Message::Schema { pid } if is_for_me(pid) => Some(Message::SchemaReplay {
            pid,
            schema: sql().schema().clone(),
        }),

        Message::Sql { pid, query } if is_for_me(pid) => match sql().execute(&query, None) {
            Ok(df) => {
                let msg = format!("Query {} run successfully", query);
                handler::message::Message::TabsAddQueryPane(df, query).enqueue();
                Some(Message::sql_replay(msg))
            }
            Err(err) => {
                //
                Some(Message::sql_replay(format!(
                    "Query {} failed: {}",
                    query, err
                )))
            }
        },

        _ => None,
    }
}

fn is_for_me(pid: u32) -> bool {
    std::process::id() == pid
}
