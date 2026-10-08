use std::time::Duration;

use itertools::Itertools;

use crate::misc::sql::BackendSchema;

use super::Message;

pub fn fetch_other_process() -> Vec<u32> {
    Message::send(Message::Ps);
    Message::recv_iter_timeout(Duration::from_millis(2000))
        .filter_map(|msg| {
            if let super::Message::PsReply { pid } = msg {
                Some(pid)
            } else {
                None
            }
        })
        .collect_vec()
}

pub fn fetch_sql_backend(pid: u32) -> Option<BackendSchema> {
    Message::send(Message::Schema { pid });
    Message::recv_iter_timeout(Duration::from_millis(2000)).find_map(|msg| {
        if let Message::SchemaReplay { pid: tpid, schema } = msg
            && pid == tpid
        {
            Some(schema.clone())
        } else {
            None
        }
    })
}

pub fn send_sql_query(pid: u32, query: String) -> Option<String> {
    Message::Sql { pid, query }.send();
    Message::recv_iter_timeout(Duration::from_millis(2000)).find_map(|msg| {
        if let Message::SqlReplay { pid: tpid, msg } = msg
            && pid == tpid
        {
            Some(msg)
        } else {
            None
        }
    })
}
