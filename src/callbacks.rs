use crate::utils::{decr_username_client_count, incr_username_client_count};
use crate::{CLIENT_ID_USERNAME, ENV_CONFIG};
use std::ffi::c_int;
use valkey_module::{
    AUTH_HANDLED, AUTH_NOT_HANDLED, Context, RedisModule_DeauthenticateAndCloseClient, Status,
    ValkeyError, ValkeyString,
};

pub(crate) fn auth_callback(
    ctx: &Context,
    username: ValkeyString,
    _password: ValkeyString,
) -> Result<c_int, ValkeyError> {
    // ip based auth for special users
    let admin_users = ENV_CONFIG.get().unwrap().admin_users.clone();
    for admin_user in admin_users {
        if username.to_string() == admin_user && get_client_ip(ctx) == "127.0.0.1" {
            unsafe {
                if let Some(tmp_fn) = RedisModule_DeauthenticateAndCloseClient {
                    tmp_fn(ctx.get_raw(), ctx.get_client_id());
                }
            }
            ctx.log_notice(&format!("closed client for user {}", admin_user));
            return Ok(AUTH_NOT_HANDLED);
        }
    }
    // regular auth
    let current_username = ctx.get_client_username();
    if ctx.authenticate_client_with_acl_user(&username) == Status::Ok {
        let client_id = ctx.get_client_id();
        decr_username_client_count(&current_username.to_string());
        incr_username_client_count(&username.to_string());
        CLIENT_ID_USERNAME.insert(client_id, username.to_string());
        return Ok(AUTH_HANDLED);
    }
    Ok(AUTH_NOT_HANDLED)
}

fn get_client_ip(ctx: &Context) -> String {
    let client_info = ctx.get_client_info();
    let addr_u8: Vec<u8> = client_info.addr.iter().map(|&x| x as u8).collect();
    let ip_addr_as_string = String::from_utf8_lossy(&addr_u8)
        .trim_matches(char::from(0))
        .to_string();
    ip_addr_as_string
}
