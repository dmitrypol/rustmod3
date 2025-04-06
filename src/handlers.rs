use crate::utils::{decr_username_client_count, incr_username_client_count};
use crate::{CLIENT_ID_USERNAME, USERNAME_CLIENT_COUNT, USERNAME_CMD_COUNT};
use std::collections::BTreeMap;
use valkey_module::server_events::{ClientChangeSubevent, KeyChangeSubevent};
use valkey_module::{Context, InfoContext, ValkeyResult};
use valkey_module_macros::{
    InfoSection, client_changed_event_handler, info_command_handler, key_event_handler,
};

#[derive(Debug, Clone, InfoSection)]
#[allow(non_camel_case_types)]
struct stats {
    commands: BTreeMap<String, u64>,
    clients: BTreeMap<String, u64>,
}

impl stats {
    fn new(commands: BTreeMap<String, u64>, clients: BTreeMap<String, u64>) -> Self {
        Self { commands, clients }
    }
}

#[info_command_handler]
fn add_info(ctx: &InfoContext, _for_crash_report: bool) -> ValkeyResult<()> {
    let commands = USERNAME_CMD_COUNT
        .iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    let clients = USERNAME_CLIENT_COUNT
        .iter()
        .map(|entry| (entry.key().clone(), *entry.value()))
        .collect();
    ctx.build_one_section(stats::new(commands, clients))
}

#[client_changed_event_handler]
fn client_changed_event_handler(ctx: &Context, client_event: ClientChangeSubevent) {
    match client_event {
        ClientChangeSubevent::Connected => {
            let username = "default".to_string();
            let client_id = ctx.get_client_id();
            incr_username_client_count(&username.to_string());
            CLIENT_ID_USERNAME.insert(client_id, username);
        }
        ClientChangeSubevent::Disconnected => {
            let username = ctx.get_client_username().to_string();
            let client_id = ctx.get_client_id();
            decr_username_client_count(&username.to_string());
            CLIENT_ID_USERNAME.remove(&client_id);
        }
    }
}

#[key_event_handler]
fn key_event_handler(ctx: &Context, key_event: KeyChangeSubevent) {
    match key_event {
        KeyChangeSubevent::Deleted => {
            ctx.log_notice("Key deleted");
        }
        KeyChangeSubevent::Evicted => {
            ctx.log_notice("Key evicted");
        }
        KeyChangeSubevent::Overwritten => {
            ctx.log_notice("Key overwritten");
        }
        KeyChangeSubevent::Expired => {
            ctx.log_notice("Key expired");
        }
    }
}
