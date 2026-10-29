# Enigma Stick Keyring

A small Rust CLI for managing secrets with an Enigma device. Encryption and
decryption happen on Enigma; encrypted values, labels, and creation/update
timestamps are stored in a local SQLite database. Labels and timestamps are
stored unencrypted.

## Setup

Requires Rust/Cargo, a connected Enigma device accessible to your user, and the
`enigma-host` library checked out alongside this directory:

```text
enigma/
├── enigma-host/
└── enigma-keyring/
```

Build the application:

```sh
cargo build --release
```

Create `config.toml` in your working directory:

```toml
vault_key_label = "keyring-vault"
database_location = "keyring.db"
```

The database and table are created automatically. The vault key is generated on
Enigma when needed. Relative paths are resolved from your working directory.

## Usage

```sh
# Create a secret; prompts for the Enigma PIN when needed and the secret value
enigma-keyring create github/personal

# Decrypt and print a secret
enigma-keyring get github/personal

# List labels and timestamps
enigma-keyring list

# Delete a stored secret
enigma-keyring delete github/personal

# Use a different configuration file
enigma-keyring --config /path/to/config.toml list

# Show help
enigma-keyring --help
```

Keep Enigma connected for all commands. Retrieving secrets requires the original
vault key on Enigma as well as the local database.

## License

Enigma Stick Host Library is licensed under the GNU General Public License,
version 3 only (`GPL-3.0-only`). See [LICENSE.md](LICENSE.md) for the full
license text.

Third-party components retain their respective licenses and copyright notices.
