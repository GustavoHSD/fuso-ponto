//! Shared format for signed payment files ("comprovantes de pagamento").
//!
//! The admin tool signs a file saying "device X is paid until date Y";
//! the app verifies it against a public key baked into the binary.
//! Payload is the canonical string `device|paid_up_to|issued_at`, signed
//! with ed25519. The file is signed, not encrypted: contents are not secret.

use chrono::{DateTime, Local, NaiveDate};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaidFile {
    pub device: String,
    pub paid_up_to: String,
    pub issued_at: String,
    /// ed25519 signature over `device|paid_up_to|issued_at`, hex encoded.
    pub signature: String,
}

fn canonical(device: &str, paid_up_to: &str, issued_at: &str) -> Vec<u8> {
    format!("{device}|{paid_up_to}|{issued_at}").into_bytes()
}

/// Generate a keypair; returns (secret_hex, public_hex), each hex encoded.
pub fn keygen() -> Result<(String, String), String> {
    let mut secret = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut secret);
    let signing = SigningKey::from_bytes(&secret);
    Ok((
        hex::encode(signing.to_bytes()),
        hex::encode(signing.verifying_key().to_bytes()),
    ))
}

/// Public key matching a stored secret, hex encoded.
pub fn public_from_secret(secret_hex: &str) -> Result<String, String> {
    let secret_bytes = hex::decode(secret_hex).map_err(|_| "chave privada inválida".to_string())?;
    let secret: [u8; 32] = secret_bytes
        .try_into()
        .map_err(|_| "chave privada inválida".to_string())?;
    Ok(hex::encode(SigningKey::from_bytes(&secret).verifying_key().to_bytes()))
}

/// Sign "device is paid until paid_up_to" (YYYY-MM-DD).
pub fn sign(secret_hex: &str, device: &str, paid_up_to: &str) -> Result<PaidFile, String> {
    validate_date(paid_up_to)?;
    let secret_bytes = hex::decode(secret_hex).map_err(|_| "chave privada inválida".to_string())?;
    let secret: [u8; 32] = secret_bytes
        .try_into()
        .map_err(|_| "chave privada inválida".to_string())?;
    let signing = SigningKey::from_bytes(&secret);
    let issued_at = DateTime::<Local>::from(std::time::SystemTime::now()).to_rfc3339();
    let sig = signing.sign(&canonical(device, paid_up_to, &issued_at));
    Ok(PaidFile {
        device: device.to_string(),
        paid_up_to: paid_up_to.to_string(),
        issued_at,
        signature: hex::encode(sig.to_bytes()),
    })
}

/// Verify a file against the public key. Returns () or a user-facing error (PT-BR).
pub fn verify(public_hex: &str, file: &PaidFile) -> Result<(), String> {
    validate_date(&file.paid_up_to)?;
    DateTime::parse_from_rfc3339(&file.issued_at)
        .map_err(|_| "arquivo inválido: data de emissão".to_string())?;
    let public_bytes: [u8; 32] = hex::decode(public_hex)
        .ok()
        .and_then(|b| b.try_into().ok())
        .ok_or_else(|| "chave pública inválida".to_string())?;
    let verifying = VerifyingKey::from_bytes(&public_bytes)
        .map_err(|_| "chave pública inválida".to_string())?;
    let sig_bytes: [u8; 64] = hex::decode(&file.signature)
        .ok()
        .and_then(|b| b.try_into().ok())
        .ok_or_else(|| "arquivo inválido: assinatura".to_string())?;
    let sig = Signature::from_bytes(&sig_bytes);
    verifying
        .verify_strict(&canonical(&file.device, &file.paid_up_to, &file.issued_at), &sig)
        .map_err(|_| "arquivo inválido ou adulterado".to_string())
}

pub fn validate_date(s: &str) -> Result<(), String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| "data inválida (use AAAA-MM-DD)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip() {
        let (secret, public) = keygen().unwrap();
        let file = sign(&secret, "device-1", "2026-11-30").unwrap();
        assert_eq!(file.device, "device-1");
        verify(&public, &file).unwrap();
    }

    #[test]
    fn tampered_file_rejected() {
        let (secret, public) = keygen().unwrap();
        let mut file = sign(&secret, "device-1", "2026-11-30").unwrap();
        file.paid_up_to = "2030-01-01".into();
        assert!(verify(&public, &file).is_err());
    }

    #[test]
    fn wrong_key_rejected() {
        let (secret, _) = keygen().unwrap();
        let (_, other_public) = keygen().unwrap();
        let file = sign(&secret, "device-1", "2026-11-30").unwrap();
        assert!(verify(&other_public, &file).is_err());
    }

    #[test]
    fn bad_date_rejected() {
        let (secret, public) = keygen().unwrap();
        let mut file = sign(&secret, "device-1", "2026-11-30").unwrap();
        file.paid_up_to = "30/11/2026".into();
        assert!(verify(&public, &file).is_err());
        // malformed signature is rejected too, not a panic
        file.paid_up_to = "2026-12-01".into();
        assert!(verify(&public, &file).is_err());
    }
}
