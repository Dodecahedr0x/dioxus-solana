#![cfg(target_arch = "wasm32")]
#[path = "mock.rs"]
mod mock;

use dioxus_solana_core::Cluster;
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn discover_returns_boxed_trait_objects() {
    let wallet = mock::make_mock_wallet();
    let cb = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |e: web_sys::CustomEvent| {
        let api = e.detail();
        let register: Function = Reflect::get(&api, &"register".into())
            .unwrap()
            .dyn_into()
            .unwrap();
        register.call1(&api, &wallet).unwrap();
    });
    let win = web_sys::window().unwrap();
    win.add_event_listener_with_callback("wallet-standard:app-ready", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();

    let wallets = dioxus_solana_web::discover(Cluster::Devnet);
    assert!(wallets.iter().any(|w| w.info().name == "MockWallet"));
}
