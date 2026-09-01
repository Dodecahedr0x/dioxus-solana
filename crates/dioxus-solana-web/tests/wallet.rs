#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, Wallet, WalletError};
use dioxus_solana_web::wallet::StandardWallet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn info_reads_name_and_features() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    assert_eq!(w.info().name, "MockWallet");
    assert!(w
        .info()
        .features
        .iter()
        .any(|f| f == "solana:signTransaction"));
}

#[wasm_bindgen_test]
async fn connect_returns_account_with_pubkey() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    let account = w.connect().await.unwrap();
    assert_eq!(account.wallet_name, "MockWallet");
    assert_eq!(account.pubkey().to_bytes(), [9u8; 32]);
}

#[wasm_bindgen_test]
async fn connect_without_feature_errors() {
    let w = StandardWallet::new(mock::make_featureless_wallet(), Cluster::Devnet);
    let err = w.connect().await.unwrap_err();
    assert!(matches!(err, WalletError::Feature(_)));
}

#[wasm_bindgen_test]
async fn connect_passes_silent_false() {
    let raw = mock::make_recording_wallet();
    let w = StandardWallet::new(raw.clone(), Cluster::Devnet);
    w.connect().await.unwrap();
    let input = js_sys::Reflect::get(&raw, &"lastConnectInput".into()).unwrap();
    let silent = js_sys::Reflect::get(&input, &"silent".into()).unwrap();
    assert_eq!(silent.as_bool(), Some(false));
}

#[wasm_bindgen_test]
async fn connect_silent_passes_silent_true() {
    let raw = mock::make_recording_wallet();
    let w = StandardWallet::new(raw.clone(), Cluster::Devnet);
    w.connect_silent().await.unwrap();
    let input = js_sys::Reflect::get(&raw, &"lastConnectInput".into()).unwrap();
    let silent = js_sys::Reflect::get(&input, &"silent".into()).unwrap();
    assert_eq!(silent.as_bool(), Some(true));
}

#[wasm_bindgen_test]
async fn connect_silent_empty_accounts_is_disconnected() {
    let w = StandardWallet::new(mock::make_empty_connect_wallet(), Cluster::Devnet);
    let err = w.connect_silent().await.unwrap_err();
    assert!(matches!(err, WalletError::Disconnected));
}
