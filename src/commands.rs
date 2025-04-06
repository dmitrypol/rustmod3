use crate::utils::{config_get_dir, metadata_save};
use crate::{CLIENT_ID_USERNAME, METADATA};
use std::collections::BTreeMap;
use valkey_module::{Context, NextArg, ValkeyError, ValkeyResult, ValkeyString};

pub(crate) fn client_id_username(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    let mut output = BTreeMap::new();
    for entry in CLIENT_ID_USERNAME.iter() {
        output.insert(entry.key().clone() as i64, entry.value().clone());
    }
    Ok(output.into())
}

pub(crate) fn metadata(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    let mut args = args.into_iter().skip(1);
    let subcommand = args.next_string()?.to_string().to_lowercase();
    let args = args.collect::<Vec<ValkeyString>>();
    match subcommand.as_str() {
        "set" => metadata_set(ctx, args),
        "get" => metadata_get(ctx, args),
        "del" => metadata_del(ctx, args),
        "flush" => metadata_flush(ctx, args),
        _ => Err(ValkeyError::Str(
            "Unknown subcommand for METADATA GET|SET|DEL|FLUSH",
        )),
    }
}

fn metadata_set(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    let mut args = args.into_iter();
    let key = args.next_string()?;
    let value = args.next_string()?;
    METADATA.insert(key, value);
    let server_dir = config_get_dir(ctx);
    metadata_save(server_dir)?;
    Ok("OK".into())
}

fn metadata_get(_ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() == 0 {
        // return all METADATA
        let mut output = BTreeMap::new();
        for entry in METADATA.iter() {
            output.insert(entry.key().clone(), entry.value().clone());
        }
        Ok(output.into())
    } else {
        // grab the key, then return METADATA value if found
        let mut args = args.into_iter();
        let key = args.next_string()?;
        match METADATA.get(&key) {
            Some(value) => Ok(value.clone().into()),
            None => Ok("metadata not found".into()),
        }
    }
}

fn metadata_del(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() == 1 {
        let mut args = args.into_iter();
        let key = args.next_string()?;
        let resp = METADATA.remove(&key);
        match resp {
            Some(tmp) => {
                let server_dir = config_get_dir(ctx);
                metadata_save(server_dir)?;
                Ok(format!("metadata deleted {:?}", tmp).into())
            }
            None => Ok("metadata not found".into()),
        }
    } else {
        Err(ValkeyError::WrongArity)
    }
}

fn metadata_flush(ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    let tmp = METADATA.clear();
    let server_dir = config_get_dir(ctx);
    metadata_save(server_dir)?;
    Ok((format!("metadata flushed {:?}", tmp)).into())
}
