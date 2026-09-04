//! Host-testable Phantom deeplink helpers for native iOS.
//!
//! Phantom holds the user key. This crate encrypts the request, opens the
//! wallet, and decrypts the callback. It does not Ed25519-sign the
//! transaction and it does not submit.

use crate::native_mwa::error_from_status;
use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    PublicKey, SalsaBox, SecretKey,
};
use dioxus_solana_core::WalletError;
use serde_json::{json, Value};

pub const PHANTOM_CONNECT_URL: &str = "https://phantom.app/ul/v1/connect";
pub const PHANTOM_SIGN_MESSAGE_URL: &str = "https://phantom.app/ul/v1/signMessage";
pub const PHANTOM_SIGN_TRANSACTION_URL: &str = "https://phantom.app/ul/v1/signTransaction";
pub const PHANTOM_REDIRECT: &str = "dioxussolana://phantom-callback";

const NONCE_LEN: usize = 24;

#[derive(Clone)]
pub struct DappKeys {
    pub secret: SecretKey,
    pub public_b58: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhantomCallback {
    pub phantom_encryption_public_key: String,
    pub nonce: String,
    pub data: String,
}

pub fn generate_dapp_keys() -> DappKeys {
    let secret = SecretKey::generate(&mut OsRng);
    let public_b58 = bs58::encode(secret.public_key().as_bytes()).into_string();
    DappKeys { secret, public_b58 }
}

pub fn connect_url(app_url: &str, dapp_pk_b58: &str, redirect: &str, cluster: &str) -> String {
    format!(
        "{}?app_url={}&dapp_encryption_public_key={}&redirect_link={}&cluster={}",
        PHANTOM_CONNECT_URL,
        qe(app_url),
        qe(dapp_pk_b58),
        qe(redirect),
        qe(cluster)
    )
}

pub fn encrypted_method_url(
    base: &str,
    dapp_pk_b58: &str,
    nonce_b58: &str,
    redirect: &str,
    payload_b58: &str,
) -> String {
    format!(
        "{}?dapp_encryption_public_key={}&nonce={}&redirect_link={}&payload={}",
        base,
        qe(dapp_pk_b58),
        qe(nonce_b58),
        qe(redirect),
        qe(payload_b58)
    )
}

pub fn sign_message_json(message: &[u8], session: &str) -> Value {
    json!({
        "message": bs58::encode(message).into_string(),
        "session": session,
        "display": "utf8",
    })
}

pub fn sign_transaction_json(tx: &[u8], session: &str) -> Value {
    json!({
        "transaction": bs58::encode(tx).into_string(),
        "session": session,
    })
}

pub fn encrypt_payload(
    secret: &SecretKey,
    phantom_pk_b58: &str,
    plaintext: &[u8],
) -> Result<(String, String), WalletError> {
    let boxed = salsa_box(secret, phantom_pk_b58)?;
    let nonce = SalsaBox::generate_nonce(&mut OsRng);
    let ciphertext = boxed
        .encrypt(&nonce, plaintext)
        .map_err(|e| WalletError::Codec(e.to_string()))?;
    Ok((
        bs58::encode(nonce.as_slice()).into_string(),
        bs58::encode(ciphertext).into_string(),
    ))
}

pub fn decrypt_payload(
    secret: &SecretKey,
    phantom_pk_b58: &str,
    nonce_b58: &str,
    data_b58: &str,
) -> Result<Vec<u8>, WalletError> {
    let boxed = salsa_box(secret, phantom_pk_b58)?;
    let nonce_bytes = decode_b58(nonce_b58)?;
    if nonce_bytes.len() != NONCE_LEN {
        return Err(WalletError::Codec("phantom nonce must be 24 bytes".into()));
    }
    let mut nonce = crypto_box::Nonce::default();
    nonce.copy_from_slice(&nonce_bytes);
    let data = decode_b58(data_b58)?;
    boxed
        .decrypt(&nonce, data.as_ref())
        .map_err(|e| WalletError::Codec(e.to_string()))
}

pub fn parse_callback(url: &str) -> Result<PhantomCallback, WalletError> {
    let pairs = query_pairs(url);
    if let Some(code) = query_value(&pairs, "errorCode") {
        let message = query_value(&pairs, "errorMessage").unwrap_or_default();
        let parsed = code.parse::<f64>().ok();
        return Err(error_from_status(
            status_from_error_code(parsed, &message),
            &message,
        ));
    }
    if let Some(message) = query_value(&pairs, "errorMessage") {
        return Err(WalletError::from_js(None, &message));
    }
    let data =
        query_value(&pairs, "data").ok_or_else(|| WalletError::Codec("missing data".into()))?;
    let nonce =
        query_value(&pairs, "nonce").ok_or_else(|| WalletError::Codec("missing nonce".into()))?;
    let phantom_encryption_public_key =
        query_value(&pairs, "phantom_encryption_public_key").unwrap_or_default();
    Ok(PhantomCallback {
        phantom_encryption_public_key,
        nonce,
        data,
    })
}

pub fn connect_account(plain: &[u8]) -> Result<(String, String), WalletError> {
    let v: Value = serde_json::from_slice(plain).map_err(|e| WalletError::Codec(e.to_string()))?;
    let public_key = v
        .get("public_key")
        .and_then(Value::as_str)
        .ok_or_else(|| WalletError::Codec("missing public_key".into()))?;
    let session = v
        .get("session")
        .and_then(Value::as_str)
        .ok_or_else(|| WalletError::Codec("missing session".into()))?;
    Ok((public_key.to_string(), session.to_string()))
}

pub fn signature_from_json(plain: &[u8]) -> Result<Vec<u8>, WalletError> {
    let v: Value = serde_json::from_slice(plain).map_err(|e| WalletError::Codec(e.to_string()))?;
    let sig = v
        .get("signature")
        .and_then(Value::as_str)
        .ok_or_else(|| WalletError::Codec("missing signature".into()))?;
    decode_b58(sig)
}

pub fn transaction_from_json(plain: &[u8]) -> Result<Vec<u8>, WalletError> {
    let v: Value = serde_json::from_slice(plain).map_err(|e| WalletError::Codec(e.to_string()))?;
    let tx = v
        .get("transaction")
        .and_then(Value::as_str)
        .ok_or_else(|| WalletError::Codec("missing transaction".into()))?;
    decode_b58(tx)
}

pub fn pubkey_from_b58(b58: &str) -> Result<solana_pubkey::Pubkey, WalletError> {
    Ok(solana_pubkey::Pubkey::try_from(
        decode_b58(b58)?.as_slice(),
    )?)
}

pub fn signature_from_b58(b58: &str) -> Result<solana_signature::Signature, WalletError> {
    Ok(solana_signature::Signature::try_from(
        decode_b58(b58)?.as_slice(),
    )?)
}

fn salsa_box(secret: &SecretKey, phantom_pk_b58: &str) -> Result<SalsaBox, WalletError> {
    let pk_bytes = decode_b58(phantom_pk_b58)?;
    let pk = PublicKey::from(<[u8; 32]>::try_from(pk_bytes.as_slice()).map_err(|_| {
        WalletError::Codec("phantom encryption public key must be 32 bytes".into())
    })?);
    Ok(SalsaBox::new(&pk, secret))
}

fn decode_b58(s: &str) -> Result<Vec<u8>, WalletError> {
    bs58::decode(s.trim())
        .into_vec()
        .map_err(|e| WalletError::Codec(e.to_string()))
}

fn status_from_error_code(code: Option<f64>, message: &str) -> &'static str {
    if code == Some(4001.0) {
        return "rejected";
    }
    match WalletError::from_js(code, message) {
        WalletError::UserRejected => "rejected",
        WalletError::NotInstalled => "not_installed",
        WalletError::Disconnected => "disconnected",
        _ => "err",
    }
}

fn query_pairs(url: &str) -> Vec<(String, String)> {
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|pair| {
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            (qd(k), qd(v))
        })
        .collect()
}

fn query_value(pairs: &[(String, String)], key: &str) -> Option<String> {
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

fn qe(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => {
                out.push('%');
                out.push(char::from(b"0123456789ABCDEF"[(b >> 4) as usize]));
                out.push(char::from(b"0123456789ABCDEF"[(b & 0xf) as usize]));
            }
        }
    }
    out
}

fn qd(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = from_hex(bytes[i + 1]);
            let lo = from_hex(bytes[i + 2]);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_solana_core::WalletError;

    #[test]
    fn connect_url_has_cluster_and_redirect() {
        let url = connect_url(
            "http://192.168.1.10:4050",
            "DappPk",
            PHANTOM_REDIRECT,
            "devnet",
        );
        assert!(url.starts_with("https://phantom.app/ul/v1/connect?"));
        assert!(url.contains("cluster=devnet"));
        assert!(url.contains("dapp_encryption_public_key=DappPk"));
        assert!(url.contains("redirect_link=dioxussolana%3A%2F%2Fphantom-callback"));
        assert!(url.contains("app_url=http%3A%2F%2F192.168.1.10%3A4050"));
    }

    #[test]
    fn sign_message_payload_is_base58_utf8() {
        let v = sign_message_json(b"hello", "sess");
        assert_eq!(v["session"], "sess");
        assert_eq!(v["display"], "utf8");
        assert_eq!(v["message"], bs58::encode(b"hello").into_string());
    }

    #[test]
    fn sign_transaction_payload_is_base58_bytes() {
        let v = sign_transaction_json(&[1, 2, 3], "sess");
        assert_eq!(v["transaction"], bs58::encode([1, 2, 3]).into_string());
        assert!(!v.to_string().contains("send"));
    }

    #[test]
    fn box_round_trips_json() {
        let dapp = generate_dapp_keys();
        let phantom = SecretKey::generate(&mut OsRng);
        let phantom_b58 = bs58::encode(phantom.public_key().as_bytes()).into_string();
        let plain = br#"{"public_key":"11111111111111111111111111111111","session":"tok"}"#;
        let (nonce, data) = encrypt_payload(&dapp.secret, &phantom_b58, plain).unwrap();
        let opened = decrypt_payload(&dapp.secret, &phantom_b58, &nonce, &data).unwrap();
        assert_eq!(opened, plain);
        let (pk, session) = connect_account(&opened).unwrap();
        assert_eq!(pk, "11111111111111111111111111111111");
        assert_eq!(session, "tok");
    }

    #[test]
    fn callback_maps_user_reject() {
        let err = parse_callback(
            "dioxussolana://phantom-callback?errorCode=4001&errorMessage=User%20rejected%20the%20request",
        )
        .unwrap_err();
        assert_eq!(err, WalletError::UserRejected);
    }

    #[test]
    fn callback_reads_encrypted_fields() {
        let cb = parse_callback(
            "dioxussolana://phantom-callback?phantom_encryption_public_key=Pk&nonce=Nn&data=Dt",
        )
        .unwrap();
        assert_eq!(cb.phantom_encryption_public_key, "Pk");
        assert_eq!(cb.nonce, "Nn");
        assert_eq!(cb.data, "Dt");
    }

    #[test]
    fn signature_json_is_base58() {
        let raw = [9u8; 64];
        let json = format!("{{\"signature\":\"{}\"}}", bs58::encode(raw).into_string());
        assert_eq!(signature_from_json(json.as_bytes()).unwrap(), raw);
    }

    #[test]
    fn pubkey_from_base58_solana_bytes() {
        let pk = solana_pubkey::Pubkey::from([7u8; 32]);
        assert_eq!(pubkey_from_b58(&pk.to_string()).unwrap(), pk);
    }

    #[test]
    fn signature_from_base58_64_bytes() {
        let raw = [9u8; 64];
        let sig = signature_from_b58(&bs58::encode(raw).into_string()).unwrap();
        assert_eq!(sig.as_ref(), &raw);
    }
}
