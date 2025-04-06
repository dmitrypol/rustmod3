use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct EnvConfig {
    pub(crate) admin_users: Vec<String>,
}

impl EnvConfig {
    pub(crate) fn new(env: &str) -> Self {
        match env {
            "dev" => EnvConfig {
                admin_users: vec![],
            },
            _ => EnvConfig {
                admin_users: vec!["admin".to_string()],
            },
        }
    }
}
