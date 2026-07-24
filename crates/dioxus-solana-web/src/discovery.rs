use gloo_events::EventListener;
use js_sys::{Function, Object, Reflect};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

use dioxus_solana_core::Cluster;

use crate::wallet::StandardWallet;

/// Collects wallets registered through the Wallet Standard window handshake.
pub struct WalletRegistry {
    cluster: Cluster,
    raw: Rc<RefCell<Vec<JsValue>>>,
    _listener: Rc<RefCell<Option<EventListener>>>,
    #[allow(clippy::type_complexity)]
    _register_cb: RefCell<Option<Closure<dyn FnMut(JsValue)>>>,
}

impl WalletRegistry {
    pub fn new(cluster: Cluster) -> Self {
        Self {
            cluster,
            raw: Rc::new(RefCell::new(Vec::new())),
            _listener: Rc::new(RefCell::new(None)),
            _register_cb: RefCell::new(None),
        }
    }

    /// Build the app-side `{ register(...wallets) }` API object.
    ///
    /// Returns the object alongside the backing `Closure` so the caller can
    /// keep it alive for as long as the API object may be invoked (instead
    /// of leaking it via `.forget()`).
    fn make_api(raw: Rc<RefCell<Vec<JsValue>>>) -> (Object, Closure<dyn FnMut(JsValue)>) {
        let api = Object::new();
        let register = Closure::<dyn FnMut(JsValue)>::new(move |wallet: JsValue| {
            raw.borrow_mut().push(wallet);
        });
        // NOTE: wallets call register(wallet) with a single arg in practice.
        Reflect::set(&api, &"register".into(), register.as_ref().unchecked_ref()).unwrap();
        (api, register)
    }

    /// Run the handshake: listen for late registrations, then dispatch app-ready.
    pub fn discover(&self) {
        let window = web_sys::window().expect("no window");
        let (api, cb) = Self::make_api(self.raw.clone());
        *self._register_cb.borrow_mut() = Some(cb);

        // 1. Answer any `register-wallet` events (wallet -> app callback).
        let api_for_listener = api.clone();
        let listener =
            EventListener::new(&window, "wallet-standard:register-wallet", move |event| {
                if let Ok(ce) = event.clone().dyn_into::<web_sys::CustomEvent>() {
                    if let Ok(cb) = ce.detail().dyn_into::<Function>() {
                        let _ = cb.call1(&JsValue::NULL, &api_for_listener);
                    }
                }
            });
        *self._listener.borrow_mut() = Some(listener);

        // 2. Dispatch `app-ready` with our api as detail (app -> wallet).
        let init = web_sys::CustomEventInit::new();
        init.set_detail(&api);
        let event =
            web_sys::CustomEvent::new_with_event_init_dict("wallet-standard:app-ready", &init)
                .expect("construct app-ready");
        let _ = window.dispatch_event(&event);
    }

    /// Snapshot the currently-registered wallets as core `StandardWallet`s.
    pub fn wallets(&self) -> Vec<StandardWallet> {
        self.raw
            .borrow()
            .iter()
            .cloned()
            .map(|raw| StandardWallet::new(raw, self.cluster))
            .collect()
    }
}
