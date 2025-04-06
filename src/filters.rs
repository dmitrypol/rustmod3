use crate::{CLIENT_ID_USERNAME, USERNAME_CMD_COUNT};
use std::ops::Deref;
use valkey_module::{CommandFilterCtx, RedisModuleCommandFilterCtx};

pub(crate) fn cmd_filter_fn(ctx: *mut RedisModuleCommandFilterCtx) {
    let cf_ctx = CommandFilterCtx::new(ctx);
    let client_id = cf_ctx.get_client_id();
    // lookup username by client_id
    let username = match CLIENT_ID_USERNAME.get(&client_id) {
        Some(tmp) => tmp.deref().clone(),
        None => "default".to_string(),
    };
    USERNAME_CMD_COUNT
        .entry(username)
        .and_modify(|v| *v += 1)
        .or_insert(1); // Inserts 1 if key is missing
}
