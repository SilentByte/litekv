////!
////! LiteKV -- A tiny key-value store with a simple REST API backed by SQLite.
////! Copyright (c) 2021 SilentByte <https://silentbyte.com/>
////!

use std::path::Path;
use std::sync::Mutex;

use chrono::{
    DateTime,
    Utc,
};
use indoc::indoc;
use rusqlite::{
    params,
    Connection,
    Transaction,
};

#[derive(Debug)]
pub struct Repo {
    connection: Mutex<Connection>,
}

impl Repo {
    pub fn create_with_file<P: AsRef<Path>>(filename: P) -> anyhow::Result<Self> {
        let repo = Repo {
            connection: Mutex::new(Connection::open(filename)?),
        };

        repo.init_db()?;
        Ok(repo)
    }

    pub fn create_in_memory() -> anyhow::Result<Self> {
        let repo = Repo {
            connection: Mutex::new(Connection::open_in_memory()?),
        };

        repo.init_db()?;
        Ok(repo)
    }

    fn init_db(&self) -> anyhow::Result<()> {
        self.connection.lock().unwrap().execute_batch(indoc! {r#"
            CREATE TABLE IF NOT EXISTS store (
                id              INTEGER PRIMARY KEY,
                scope           TEXT NOT NULL,
                key             TEXT NOT NULL,
                value           TEXT NOT NULL,
                created_on      TEXT NOT NULL,
                expires_on      TEXT
            );

            CREATE INDEX IF NOT EXISTS store_scope_key_index ON store (scope, key);

            CREATE INDEX IF NOT EXISTS store_created_on_index ON store (created_on DESC);

            CREATE INDEX IF NOT EXISTS store_expires_on_index ON store (expires_on DESC);
        "#})?;

        Ok(())
    }

    pub fn commit_data(
        &self,
        scope: &str,
        key: &str,
        value: &str,
        created_on: DateTime<Utc>,
        expires_on: Option<DateTime<Utc>>,
    ) -> anyhow::Result<()> {
        self.connection.lock().unwrap().execute(
            indoc! {r#"
                INSERT INTO store (scope, key, value, created_on, expires_on)
                VALUES (?1, ?2, ?3, ?4, ?5);
            "#},
            params![scope, key, value, created_on, expires_on],
        )?;

        Ok(())
    }
}
