use rusqlite::Row;
use tabled::Tabled;

#[derive(Tabled)]
pub(crate) struct Secret {
    pub(crate) label: String,

    #[tabled(skip)]
    pub(crate) encrypted: String,

    #[tabled(display = "format_epoch")]
    pub(crate) created_at: i64,

    #[tabled(display = "format_epoch")]
    pub(crate) updated_at: i64,
}

impl Secret {
    pub(crate) fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            label: row.get("label")?,
            encrypted: row.get("encrypted")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

fn format_epoch(seconds: &i64) -> String {
    chrono::DateTime::from_timestamp(*seconds, 0)
        .map(|date| {
            date.with_timezone(&chrono::Local)
                .format("%Y-%m-%d %H:%M:%S %:z")
                .to_string()
        })
        .unwrap_or_else(|| "Invalid timestamp".to_string())
}
