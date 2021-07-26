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

    /// whether or not the data store should be readonly
    #[argh(switch)]
    readonly: bool,
}

impl AppConfig {
    pub fn load() -> Self {
        let config: Self = argh::from_env();
        if config.readonly && config.db.is_none() {
            eprintln!(
                "{}",
                "Configuration error: --readonly requires --db to also be defined",
            );
            std::process::exit(1);
        }

        config
    }

    pub fn db(&self) -> &Option<String> {
        &self.db
    }

    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
