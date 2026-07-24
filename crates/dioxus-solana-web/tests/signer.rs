#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, WalletSigner};
use dioxus_solana_web::signer::StandardSigner;
use solana_transaction::versioned::VersionedTransaction;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn signer() -> StandardSigner {
    let wallet = mock::make_mock_wallet();
    let account = js_sys::Reflect::get(&wallet, &"accounts".into()).unwrap();
    let account = js_sys::Array::from(&account).get(0);
    StandardSigner::new(wallet, account, Cluster::Devnet).unwrap()
}

#[wasm_bindgen_test]
async fn pubkey_reads_account_bytes() {
    assert_eq!(signer().pubkey().to_bytes(), [9u8; 32]);
}

#[wasm_bindgen_test]
async fn sign_message_returns_signature() {
    let sig = signer().sign_message(b"hello").await.unwrap();
    assert_eq!(sig.as_ref(), &[1u8; 64]);
}

#[wasm_bindgen_test]
async fn sign_transaction_round_trips() {
    let tx = VersionedTransaction::default();
    let signed = signer().sign_transaction(tx.clone()).await.unwrap();
    assert_eq!(signed, tx);
}

#[wasm_bindgen_test]
async fn sign_and_send_returns_signature() {
    let sig = signer().sign_and_send_transaction(VersionedTransaction::default()).await.unwrap();
    assert_eq!(sig.as_ref(), &[2u8; 64]);
}
