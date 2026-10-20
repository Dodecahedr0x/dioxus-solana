use gloo_events::EventListener;
use js_sys::{Function, Object, Reflect};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

use dioxus_solana_core::{AppIdentity, Cluster, Wallet};

use crate::wallet::StandardWallet;

type OnChange = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Collects wallets registered through the Wallet Standard window handshake.
pub struct WalletRegistry {
    cluster: RefCell<Cluster>,
    identity: Option<AppIdentity>,
    raw: Rc<RefCell<Vec<JsValue>>>,
    on_change: OnChange,
    _listener: Rc<RefCell<Option<EventListener>>>,
    #[allow(clippy::type_complexity)]
    _register_cb: RefCell<Option<Closure<dyn FnMut(JsValue)>>>,
}

impl WalletRegistry {
    pub fn new(cluster: Cluster) -> Self {
        Self::new_with_identity(cluster, None)
    }

    pub fn new_with_identity(cluster: Cluster, identity: Option<AppIdentity>) -> Self {
        Self {
            cluster: RefCell::new(cluster),
            identity,
            raw: Rc::new(RefCell::new(Vec::new())),
            on_change: Rc::new(RefCell::new(None)),
            _listener: Rc::new(RefCell::new(None)),
            _register_cb: RefCell::new(None),
        }
    }

    /// Build the app-side `{ register(...wallets) }` API object.
    ///
    /// Returns the object alongside the backing `Closure` so the caller can
    /// keep it alive for as long as the API object may be invoked (instead
    /// of leaking it via `.forget()`).
    fn make_api(
        raw: Rc<RefCell<Vec<JsValue>>>,
        on_change: OnChange,
    ) -> (Object, Closure<dyn FnMut(JsValue)>) {
        let api = Object::new();
        let register = Closure::<dyn FnMut(JsValue)>::new(move |wallet: JsValue| {
            raw.borrow_mut().push(wallet);
            if let Some(notify) = on_change.borrow().as_ref() {
                notify();
            }
        });
        // NOTE: wallets call register(wallet) with a single arg in practice.
        Reflect::set(&api, &"register".into(), register.as_ref().unchecked_ref()).unwrap();
        (api, register)
    }

    /// Run the handshake: listen for late registrations, then dispatch app-ready.
    pub fn discover(&self) {
        crate::mwa::register(self.identity.as_ref(), self.cluster.borrow().chain_id());
        let window = web_sys::window().expect("no window");
        let (api, cb) = Self::make_api(self.raw.clone(), self.on_change.clone());
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

    /// Call `notify` whenever a wallet registers — including late ones that
    /// load after [`discover`](Self::discover) — so the caller can re-snapshot
    /// [`wallets`](Self::wallets).
    pub fn on_change(&self, notify: impl Fn() + 'static) {
        *self.on_change.borrow_mut() = Some(Box::new(notify));
    }

    /// Point wallets at a new cluster. Takes effect for wallets snapshotted
    /// (and so connected) afterwards; a live session keeps its chain until
    /// it reconnects.
    pub fn set_cluster(&self, cluster: Cluster) {
        *self.cluster.borrow_mut() = cluster;
    }

    /// Snapshot the currently-registered wallets as core `StandardWallet`s.
    ///
    /// A wallet can register twice — once via the `register-wallet` event and
    /// again in response to our `app-ready` dispatch — so entries are
    /// deduplicated by name, keeping the first occurrence.
    pub fn wallets(&self) -> Vec<StandardWallet> {
        let mut seen = std::collections::HashSet::new();
        let cluster = self.cluster.borrow().clone();
        self.raw
            .borrow()
            .iter()
            .cloned()
            .map(|raw| StandardWallet::new(raw, cluster.clone()))
            .filter(|w| seen.insert(w.info().name.clone()))
            .collect()
    }
}
