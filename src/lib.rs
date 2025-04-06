mod callbacks;
mod commands;
mod env_config;
mod filters;
mod handlers;
mod utils;

use crate::callbacks::auth_callback;
use crate::commands::{client_id_username, metadata};
use crate::env_config::EnvConfig;
use crate::filters::cmd_filter_fn;
use crate::utils::{config_get_dir, metadata_load, valid_server_version};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::sync::LazyLock;
use valkey_module::{
    Context, Status, VALKEYMODULE_CMDFILTER_NOSELF, ValkeyString, alloc::ValkeyAlloc, valkey_module,
};

static MIN_VALID_SERVER_VERSION: &[i64; 3] = &[7, 2, 8];
static CLIENT_ID_USERNAME: LazyLock<DashMap<u64, String>> = LazyLock::new(|| DashMap::new());
static USERNAME_CMD_COUNT: LazyLock<DashMap<String, u64>> = LazyLock::new(|| DashMap::new());
static USERNAME_CLIENT_COUNT: LazyLock<DashMap<String, u64>> = LazyLock::new(|| DashMap::new());
static ENV_CONFIG: OnceCell<EnvConfig> = OnceCell::new();
static METADATA: LazyLock<DashMap<String, String>> = LazyLock::new(|| DashMap::new());

fn preload(ctx: &Context, _args: &[ValkeyString]) -> Status {
    let ver = ctx.get_server_version().expect("can't get_server_version");
    if !valid_server_version(ver) {
        ctx.log_notice(format!("min valid server version {:?}", MIN_VALID_SERVER_VERSION).as_str());
        Status::Err
    } else {
        Status::Ok
    }
}

fn init(ctx: &Context, args: &[ValkeyString]) -> Status {
    let env_name = match args.get(0) {
        Some(tmp) => tmp.to_string(),
        None => "".to_string(),
    };
    let _ = ENV_CONFIG.set(EnvConfig::new(env_name.as_str()));
    let server_dir = config_get_dir(ctx);
    let _ = metadata_load(server_dir);
    Status::Ok
}

fn deinit(_ctx: &Context) -> Status {
    Status::Ok
}

valkey_module! {
    name: "rustmod3",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    preload: preload,
    init: init,
    deinit: deinit,
    auth: [auth_callback],
    commands: [
        ["metadata", metadata, "", 0, 0, 0],
        ["client_id_username", client_id_username, "", 0, 0, 0],
    ],
    filters: [
        [cmd_filter_fn, VALKEYMODULE_CMDFILTER_NOSELF]
    ]
}
