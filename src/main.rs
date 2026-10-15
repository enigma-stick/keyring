mod cli;
mod config;
mod secret;
mod store;
mod vault;

use std::{error::Error, fs, path};

use clap::Parser;
use tabled::{Table, settings::Style};

use crate::{
    cli::{Args, Command},
    config::Config,
    secret::Secret,
    store::DbStore,
    vault::{decrypt_secret, encrypt_secret, ensure_authenticated, ensure_vault_key_exists},
};

fn main() -> Result<(), Box<dyn Error>> {
    // args
    let args = Args::parse();

    // config
    let config_contents = fs::read_to_string(&args.config)?;
    let config: Config = toml::from_str(&config_contents)?;

    // db
    let db_path = path::Path::new(config.database_location.as_str());
    let store = DbStore::open(&db_path)?;
    store.init()?;

    // enigma device
    let mut device = enigma::Device::open()?;

    // command
    match args.command {
        Command::Create { label } => {
            ensure_vault_key_exists(&mut device, config.vault_key_label.clone())?;
            ensure_authenticated(&mut device)?;
            let secret_value = rpassword::prompt_password("Secret: ")?;
            let encrypted = encrypt_secret(&mut device, &config, secret_value)?;

            let secret = Secret {
                label: label.clone(),
                encrypted,
                created_at: 0,
                updated_at: 0,
            };
            store.create(secret)?;
            println!("Secret created as \"{}\"", label);
        }
        Command::Get { label } => {
            ensure_vault_key_exists(&mut device, config.vault_key_label.clone())?;
            ensure_authenticated(&mut device)?;
            match store.get(label.as_str()).unwrap() {
                Some(secret) => {
                    let value = decrypt_secret(&mut device, &config, secret.encrypted)?;
                    print!("{}", value);
                }
                None => println!("No secret with label \"{}\"", label.clone()),
            }
        }
        Command::List => {
            let secrets = store.list()?;
            let mut table = Table::new(&secrets);
            table.with(Style::rounded());
            println!("{table}");
        }
        Command::Delete { label } => store.delete(label)?,
    }

    Ok(())
}
