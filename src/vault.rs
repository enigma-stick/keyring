use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use enigma::EnigmaError;

use crate::config::Config;

pub(crate) fn ensure_authenticated(device: &mut enigma::Device) -> Result<(), EnigmaError> {
    let status = device.status()?;

    if status.session == enigma::SessionState::Authenticated {
        return Ok(());
    }

    let pin = rpassword::prompt_password("Enigma PIN: ").unwrap();
    device.authenticate(pin.as_str())
}

pub(crate) fn ensure_vault_key_exists(
    device: &mut enigma::Device,
    key_label: String,
) -> Result<u32, EnigmaError> {
    let filter = enigma::KeyListFilter {
        key_label: Some(key_label.as_str()),
        ..Default::default()
    };

    let keys = device.list_keys(filter)?;

    if keys.keys.len() == 1 {
        return Ok(keys.keys[0].id);
    }

    let request = enigma::GenerateKeyRequest {
        label: key_label.as_str(),
        key_type: enigma::KeyType::Aes,
        key_usage: enigma::KeyUsage::Encrypt,
        key_size: 256,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    };

    ensure_authenticated(device)?;
    device.generate_key(request).map(|key| key.key_id)
}

pub(crate) fn encrypt_secret(
    device: &mut enigma::Device,
    config: &Config,
    secret: String,
) -> Result<String, EnigmaError> {
    let key_id = ensure_vault_key_exists(device, config.vault_key_label.clone())?;

    let request = enigma::EncryptRequest {
        key_id,
        plaintext: secret.as_bytes(),
    };
    let response = device.encrypt(request)?;

    use base64::engine::general_purpose::STANDARD;
    let encoded = STANDARD.encode(response.data);

    Ok(encoded)
}

pub(crate) fn decrypt_secret(
    device: &mut enigma::Device,
    config: &Config,
    ciphertext: String,
) -> Result<String, EnigmaError> {
    let key_id = ensure_vault_key_exists(device, config.vault_key_label.clone())?;

    use base64::engine::general_purpose::STANDARD;
    let raw_ciphertext = STANDARD.decode(ciphertext).unwrap();

    let request = enigma::DecryptRequest {
        key_id,
        ciphertext: raw_ciphertext.as_slice(),
    };
    let response = device.decrypt(request)?;

    String::from_utf8(response.data.to_vec()).map_err(|_| EnigmaError::InvalidArgument)
}
