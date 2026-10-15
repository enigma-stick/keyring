use std::path;

use rusqlite::{Connection, OptionalExtension};

use crate::secret::Secret;

pub(crate) struct DbStore {
    connection: Connection,
}

impl DbStore {
    pub(crate) fn open(path: &path::Path) -> rusqlite::Result<Self> {
        Ok(Self {
            connection: Connection::open(path)?,
        })
    }

    pub(crate) fn init(&self) -> Result<(), rusqlite::Error> {
        let table_exists = self.connection.query_row(
            "SELECT COUNT(name) AS count FROM sqlite_master WHERE type='table' AND name= ?1;",
            ["secrets"],
            |row| Ok(row.get::<&str, i64>("count")? == 1),
        )?;

        if table_exists {
            return Ok(());
        }

        let mut statement = self.connection.prepare(
            "CREATE TABLE secrets (
                label       TEXT PRIMARY KEY NOT NULL,
                encrypted   TEXT NOT NULL,
                created_at  INTEGER NOT NULL,
                updated_at  INTEGER NOT NULL
            );",
        )?;

        statement.execute([])?;
        Ok(())
    }

    pub(crate) fn get(&self, label: &str) -> rusqlite::Result<Option<Secret>> {
        self.connection
            .query_row(
                "SELECT label, encrypted, created_at, updated_at
                FROM secrets
                WHERE label = ?1",
                [label],
                Secret::from_row,
            )
            .optional()
    }

    pub(crate) fn list(&self) -> rusqlite::Result<Vec<Secret>> {
        let mut statement = self.connection.prepare(
            "SELECT label, encrypted, created_at, updated_at
            FROM secrets
            ORDER BY label",
        )?;

        let rows = statement.query_map([], Secret::from_row)?;
        rows.collect()
    }

    pub(crate) fn create(&self, secret: Secret) -> Result<(), rusqlite::Error> {
        let mut statement = self.connection.prepare(
            "INSERT INTO secrets (label, encrypted, created_at, updated_at)
            VALUES (?1, ?2, unixepoch('now'), unixepoch('now'));",
        )?;

        statement.execute([secret.label, secret.encrypted])?;
        Ok(())
    }

    pub(crate) fn delete(&self, label: String) -> Result<(), rusqlite::Error> {
        let mut statement = self
            .connection
            .prepare("DELETE FROM secrets WHERE label = ?1;")?;

        statement.execute([label])?;
        Ok(())
    }
}
