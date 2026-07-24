use dioxus::prelude::*;
use dioxus_solana::{use_wallet, Cluster, WalletProvider, WalletState};

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        WalletProvider { cluster: Cluster::Devnet, autoconnect: true, Demo {} }
    }
}

#[component]
fn Demo() -> Element {
    let wallet = use_wallet();
    let mut last_sig = use_signal(|| Option::<String>::None);

    rsx! {
        h1 { "dioxus-solana demo" }
        match wallet.state() {
            WalletState::Connected(_) => rsx! {
                p { "Connected: {wallet.pubkey().unwrap()}" }
                p { "Cluster: {wallet.cluster().chain_id()}" }
                button { onclick: move |_| wallet.disconnect(), "Disconnect" }
                button {
                    onclick: move |_| async move {
                        if let Ok(sig) = wallet.sign_message(b"gm from dioxus".to_vec()).await {
                            last_sig.set(Some(sig.to_string()));
                        }
                    },
                    "Sign message"
                }
                if let Some(sig) = last_sig() {
                    p { "Signature: {sig}" }
                }
            },
            WalletState::Connecting => rsx! { p { "Connecting…" } },
            other => rsx! {
                p { "Choose a wallet:" }
                for info in wallet.wallets() {
                    button {
                        key: "{info.name}",
                        onclick: {
                            let name = info.name.clone();
                            move |_| wallet.connect(name.clone())
                        },
                        "{info.name}"
                    }
                }
                if let WalletState::Error(e) = other {
                    p { style: "color:red", "Error: {e}" }
                }
            },
        }
    }
}
