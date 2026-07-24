#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, Wallet};
use dioxus_solana_web::wallet::StandardWallet;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn info_reads_name_and_features() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    assert_eq!(w.info().name, "MockWallet");
    assert!(w.info().features.iter().any(|f| f == "solana:signTransaction"));
}

#[wasm_bindgen_test]
async fn connect_returns_account_with_pubkey() {
    let w = StandardWallet::new(mock::make_mock_wallet(), Cluster::Devnet);
    let account = w.connect().await.unwrap();
    assert_eq!(account.wallet_name, "MockWallet");
    assert_eq!(account.pubkey().to_bytes(), [9u8; 32]);
}
