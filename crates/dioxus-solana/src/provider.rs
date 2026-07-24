use dioxus::prelude::*;
use dioxus_solana_core::{Cluster, Wallet};
use std::rc::Rc;

use crate::state::WalletState;

#[derive(Clone, Copy)]
pub struct WalletContext {
    pub cluster: Signal<Cluster>,
    pub state: Signal<WalletState>,
    pub wallets: Signal<Vec<Rc<dyn Wallet>>>,
}

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

    // Persistent registry: lives for the provider's lifetime so its
    // register-wallet listener/closure aren't dropped after one call.
    // `new_registry` has no side effects; the handshake runs in the effect.
    let registry = use_hook(|| crate::platform::new_registry(props.cluster));

    // Runs exactly once on mount: the body only writes signals (never reactively
    // reads one), so it is not re-triggered. Do not add a signal read here.
    use_effect(move || {
        let mut wallets = wallets;
        let found = crate::platform::run_discovery(&registry);
        wallets.set(found.clone());
        // v1 limitation: wallets injected AFTER this mount snapshot are held by
        // the registry but not re-pushed to the signal. Mainstream wallets
        // register synchronously on app-ready, so this covers the common case.

        #[cfg(target_arch = "wasm32")]
        if autoconnect {
            let mut state = state;
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
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (autoconnect, &found);
    });

    rsx! { {props.children} }
}
