use crate::{METADATA, MIN_VALID_SERVER_VERSION, USERNAME_CLIENT_COUNT};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use valkey_module::logging::log_notice;
use valkey_module::{Context, ValkeyError, ValkeyValue, Version};

pub(crate) fn valid_server_version(version: Version) -> bool {
    let server_version = &[
        version.major.into(),
        version.minor.into(),
        version.patch.into(),
    ];
    server_version >= MIN_VALID_SERVER_VERSION
}

pub(crate) fn metadata_save(server_dir: String) -> Result<(), ValkeyError> {
    let map: HashMap<_, _> = METADATA
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect();
    let json = serde_json::to_string_pretty(&map)?;
    let mut file = File::create(&format!("{}/metadata.json", server_dir))?;
    file.write_all(json.as_bytes())?;
    log_notice(&format!("metadata_save: {:?} ", map));
    Ok(())
}

pub(crate) fn metadata_load(server_dir: String) -> Result<(), ValkeyError> {
    Ok(match File::open(&format!("{}/metadata.json", server_dir)) {
        Ok(file) => {
            let map: HashMap<String, String> = serde_json::from_reader(file)?;
            for (key, value) in map {
                METADATA.insert(key, value);
            }
            log_notice(&format!("metadata_load: {:?}", METADATA));
        }
        Err(err) => {
            log_notice(&format!("metadata_load error: {:?}", err));
        }
    })
}

pub(crate) fn config_get_dir(ctx: &Context) -> String {
    let mut output = "".to_string();
    //  Array([SimpleString("dir"), SimpleString("/path/here")])
    let conig_get_dir = ctx.call("config", &["get", "dir"]).unwrap();
    match conig_get_dir {
        ValkeyValue::Array(tmp) => {
            //  Some(SimpleString("/path/here"))
            let server_dir = tmp.get(1);
            match server_dir {
                Some(ValkeyValue::SimpleString(server_dir)) => {
                    output = server_dir.to_string();
                }
                _ => {}
            }
        }
        _ => {}
    }
    output
}

pub(crate) fn incr_username_client_count(username: &String) {
    USERNAME_CLIENT_COUNT
        .entry(username.into())
        .and_modify(|v| *v += 1)
        .or_insert(1); // Inserts 1 if key is missing
}

pub(crate) fn decr_username_client_count(username: &String) {
    USERNAME_CLIENT_COUNT
        .entry(username.into())
        .and_modify(|v| *v -= 1)
        .or_insert(0); // Inserts 0 if key is missing
}
