use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) vault_key_label: String,
    pub(crate) database_location: String,
}
