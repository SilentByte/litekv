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
    OpenFlags,
};

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub value: serde_json::Value,
    pub created_on: DateTime<Utc>,
    pub expires_on: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Repo {
    connection: Mutex<Connection>,
}

impl Repo {
    pub fn create_with_file<P: AsRef<Path>>(filename: P, readonly: bool) -> anyhow::Result<Self> {
        let flags = if readonly {
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
        } else {
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
        };

        let repo = Repo {
            connection: Mutex::new(Connection::open_with_flags(filename, flags)?),
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
        // language=sql
        self.connection.lock().unwrap().execute_batch(indoc! {r#"
            CREATE TABLE IF NOT EXISTS store (
                id              INTEGER PRIMARY KEY,
                scope           TEXT NOT NULL,
                key             TEXT NOT NULL,
                value           BLOB NOT NULL,
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
        value: &serde_json::Value,
        created_on: DateTime<Utc>,
        expires_on: Option<DateTime<Utc>>,
    ) -> anyhow::Result<()> {
        // language=sql
        self.connection.lock().unwrap().execute(
            indoc! {r#"
                INSERT INTO store (scope, key, value, created_on, expires_on)
                VALUES (?1, ?2, ?3, ?4, ?5);
            "#},
            params![scope, key, value, created_on, expires_on],
        )?;

        Ok(())
    }

    pub fn query_data(
        &self,
        scope: &str,
        key: &str,
        start_on: Option<DateTime<Utc>>,
        end_on: Option<DateTime<Utc>>,
        limit: Option<u64>,
    ) -> anyhow::Result<Vec<QueryResult>> {
        // language=sql
        let result = self
            .connection
            .lock()
            .unwrap()
            .prepare_cached(indoc! {r#"
                SELECT value, created_on, expires_on
                FROM store
                WHERE scope = ?1
                    AND key = ?2
                    AND (?3 IS NULL OR expires_on IS NULL OR expires_on < ?3)
                    AND (?4 IS NULL OR created_on >= ?4)
                    AND (?5 IS NULL OR created_on <= ?5)
                ORDER BY created_on DESC
                LIMIT coalesce(?6, -1);
            "#})?
            .query_map(
                params![scope, key, Utc::now(), start_on, end_on, limit],
                |row| {
                    Ok(QueryResult {
                        value: row.get(0)?,
                        created_on: row.get(1)?,
                        expires_on: row.get(2)?,
                    })
                },
            )?
            .map(|row| row.unwrap())
            .collect();

        Ok(result)
    }
}
