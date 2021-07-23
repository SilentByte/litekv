////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use std::path::Path;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Repo {
    connection: Mutex<rusqlite::Connection>,
}

impl Repo {
    pub fn create_with_file<P: AsRef<Path>>(filename: P) -> anyhow::Result<Self> {
        Ok(Repo {
            connection: Mutex::new(rusqlite::Connection::open(filename)?),
        })
    }

    pub fn create_in_memory() -> anyhow::Result<Self> {
        Ok(Repo {
            connection: Mutex::new(rusqlite::Connection::open_in_memory()?),
        })
    }
}
