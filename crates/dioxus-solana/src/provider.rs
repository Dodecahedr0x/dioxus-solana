use dioxus::prelude::*;
use dioxus_solana_core::{AppIdentity, Cluster, Wallet};
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
    /// Fallback cluster, used only when no ancestor [`RpcProvider`] provides one.
    ///
    /// [`RpcProvider`]: crate::rpc::RpcProvider
    #[props(default = Cluster::MainnetBeta)]
    pub cluster: Cluster,
    #[props(default = true)]
    pub autoconnect: bool,
    /// Shown to the mobile wallet during Mobile Wallet Adapter authorization.
    ///
    /// When omitted, the page title and origin are used. `icon` must be a
    /// relative path (MWA wallets reject absolute icon URLs).
    #[props(default)]
    pub app_identity: Option<AppIdentity>,
    pub children: Element,
}

/// Silent reconnect to the last-used wallet, attempted once — as soon as that
/// wallet has registered (it may load after the app).
#[cfg(target_arch = "wasm32")]
struct Autoconnect {
    remembered: Option<String>,
    state: Signal<WalletState>,
}

#[cfg(target_arch = "wasm32")]
impl Autoconnect {
    fn try_with(&mut self, found: &[Rc<dyn Wallet>]) {
        let Some(name) = &self.remembered else { return };
        let Some(w) = found.iter().find(|w| &w.info().name == name).cloned() else {
            return;
        };
        self.remembered = None;
        if !matches!(*self.state.peek(), WalletState::Disconnected) {
            return; // the user already picked a wallet
        }
        let mut state = self.state;
        spawn(async move {
            state.set(WalletState::Connecting);
            match w.connect_silent().await {
                Ok(acc) => state.set(WalletState::Connected(acc)),
                Err(_) => state.set(WalletState::Disconnected),
            }
        });
    }
}

/// Provides wallet context to descendants. Mount once near the app root.
///
/// The cluster (which chain wallets sign against) comes from an ancestor
/// [`RpcProvider`](crate::rpc::RpcProvider) if one is mounted; otherwise it
/// falls back to the `cluster` prop (default [`Cluster::MainnetBeta`]).
#[component]
pub fn WalletProvider(props: WalletProviderProps) -> Element {
    // Own the fallback so the hook count is stable whether or not an
    // RpcProvider is present; the ancestor's cluster wins when it is.
    let fallback = use_signal(move || props.cluster);
    let cluster = use_hook(|| {
        try_consume_context::<crate::rpc::RpcContext>()
            .map(|rpc| rpc.cluster)
            .unwrap_or(fallback)
    });
    let state = use_signal(WalletState::default);
    let wallets = use_signal(Vec::<Rc<dyn Wallet>>::new);

    let ctx = WalletContext {
        cluster,
        state,
        wallets,
    };
    use_context_provider(|| ctx);

    let autoconnect = props.autoconnect;
    let identity = props.app_identity.clone();

    // Persistent registry: lives for the provider's lifetime so its
    // register-wallet listener/closure aren't dropped after one call.
    // `new_registry` has no side effects; the handshake runs in the effect.
    let registry = use_hook(|| crate::platform::new_registry(cluster.peek().clone(), identity));

    let discovery = registry.clone();
    // Runs exactly once on mount: the body only writes signals (never reactively
    // reads one), so it is not re-triggered. Do not add a signal read here.
    use_effect(move || {
        let mut wallets = wallets;
        let registry = &discovery;
        let found = crate::platform::run_discovery(registry);
        wallets.set(found);

        #[cfg(target_arch = "wasm32")]
        {
            let remembered = autoconnect.then(crate::storage::last_wallet).flatten();
            let mut autoconnect = Autoconnect { remembered, state };
            autoconnect.try_with(&wallets.peek());

            // Wallets that load late register after the handshake; re-snapshot
            // on each registration. The registry calls back from a JS event, so
            // hop onto the dioxus runtime through a channel.
            let (tx, mut rx) = futures_channel::mpsc::unbounded::<()>();
            registry.on_change(move || {
                let _ = tx.unbounded_send(());
            });
            let registry = registry.clone();
            spawn(async move {
                use futures_util::StreamExt;
                while rx.next().await.is_some() {
                    let found = crate::platform::snapshot(&registry);
                    autoconnect.try_with(&found);
                    wallets.set(found);
                }
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = autoconnect;
    });

    // Follow cluster switches (e.g. a reactive `RpcProvider` cluster): new
    // connections target the new chain, and a live session is silently
    // re-established on it so signing requests carry the right chain.
    let mut applied = use_hook(|| CopyValue::new(cluster.peek().clone()));
    use_effect(move || {
        let next = cluster();
        if *applied.peek() == next {
            return;
        }
        applied.set(next.clone());
        crate::platform::set_cluster(&registry, next.clone());
        let found = crate::platform::snapshot(&registry);
        let mut wallets = wallets;
        wallets.set(found.clone());

        let name = match &*state.peek() {
            WalletState::Connected(acc) => acc.wallet_name.clone(),
            _ => return,
        };
        let mut state = state;
        spawn(async move {
            let result = match found.iter().find(|w| w.info().name == name) {
                Some(w) => w.connect_silent().await,
                None => Err(dioxus_solana_core::WalletError::NotInstalled),
            };
            // A newer switch superseded this one; its own reconnect wins.
            if *applied.peek() != next {
                return;
            }
            match result {
                Ok(acc) => state.set(WalletState::Connected(acc)),
                Err(_) => state.set(WalletState::Disconnected),
            }
        });
    });

    rsx! { {props.children} }
}
