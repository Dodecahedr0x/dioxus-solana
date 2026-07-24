#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::{Cluster, Wallet};
use dioxus_solana_web::discovery::WalletRegistry;
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn app_ready_dispatch_registers_a_listening_wallet() {
    // Simulate a wallet: on `wallet-standard:app-ready`, call detail.register(mockWallet).
    let wallet = mock::make_mock_wallet();
    let cb = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |e: web_sys::CustomEvent| {
        let api = e.detail();
        let register: Function = Reflect::get(&api, &"register".into()).unwrap().dyn_into().unwrap();
        register.call1(&api, &wallet).unwrap();
    });
    let win = web_sys::window().unwrap();
    win.add_event_listener_with_callback("wallet-standard:app-ready", cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    let registry = WalletRegistry::new(Cluster::Devnet);
    registry.discover(); // dispatches app-ready; the wallet registers synchronously
    let wallets = registry.wallets();
    assert!(wallets.iter().any(|w| w.info().name == "MockWallet"));
}
