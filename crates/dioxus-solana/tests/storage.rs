#![cfg(target_arch = "wasm32")]
use dioxus_solana::storage;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn remembers_and_clears_last_wallet() {
    storage::set_last_wallet("Phantom");
    assert_eq!(storage::last_wallet().as_deref(), Some("Phantom"));
    storage::clear_last_wallet();
    assert_eq!(storage::last_wallet(), None);
}
