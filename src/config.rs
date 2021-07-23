////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use argh::FromArgs;

/// A tiny key-value store with a simple REST API backed by SQLite.
#[derive(Debug, Clone, FromArgs)]
pub struct AppConfig {
    /// host name or address for which LiteKV is listening
    #[argh(option, default = "\"127.0.0.1\".to_string()")]
    host: String,

    /// port on which LiteKV is listening
    #[argh(option, default = "8088")]
    port: usize,

    /// path to the database file
    #[argh(option)]
    db: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        argh::from_env()
    }

    pub fn db(&self) -> &Option<String> {
        &self.db
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
