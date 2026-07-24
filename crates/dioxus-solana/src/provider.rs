use dioxus::prelude::*;
use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

use crate::state::WalletState;

/// Shared reactive state placed in Dioxus context by `WalletProvider`.
#[derive(Clone, Copy)]
pub struct WalletContext {
    pub cluster: Signal<Cluster>,
    pub state: Signal<WalletState>,
    pub wallets: Signal<Vec<Rc<dyn Wallet>>>,
}

/// Props for [`WalletProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct WalletProviderProps {
    #[props(default = Cluster::MainnetBeta)]
    pub cluster: Cluster,
    #[props(default = true)]
    pub autoconnect: bool,
    pub children: Element,
}

/// Provides wallet context to descendants. Mount once near the app root.
#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    let cluster = use_signal(|| props.cluster);
    let state = use_signal(WalletState::default);
    let wallets = use_signal(Vec::<Rc<dyn Wallet>>::new);

    let ctx = WalletContext { cluster, state, wallets };
    use_context_provider(|| ctx);

    let autoconnect = props.autoconnect;

    #[cfg(target_arch = "wasm32")]
    {
        // Persistent registry: lives for the provider's lifetime so its
        // register-wallet listener keeps capturing wallets and the register
        // closure isn't dropped after a single call. `new` has no side effects;
        // the handshake runs later in the effect.
        let registry = use_hook(|| {
            Rc::new(dioxus_solana_web::discovery::WalletRegistry::new(props.cluster))
        });

        use_effect(move || {
            let registry = registry.clone();
            let mut wallets = wallets;
            let mut state = state;

            registry.discover();
            let found: Vec<Rc<dyn Wallet>> = registry
                .wallets()
                .into_iter()
                .map(|w| Rc::new(w) as Rc<dyn Wallet>)
                .collect();
            wallets.set(found.clone());
            // v1 limitation: wallets injected AFTER this mount snapshot are held
            // by the registry but not re-pushed to the signal. Mainstream wallets
            // register synchronously on app-ready, so this covers the common case.

            if autoconnect {
                if let Some(name) = crate::storage::last_wallet() {
                    if let Some(w) = found.iter().find(|w| w.info().name == name).cloned() {
                        spawn(async move {
                            state.set(WalletState::Connecting);
                            match w.connect().await {
                                Ok(acc) => state.set(WalletState::Connected(acc)),
                                Err(_) => state.set(WalletState::Disconnected),
                            }
                        });
                    }
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (autoconnect, cluster); // desktop connector not yet implemented

    rsx! { {props.children} }
}
