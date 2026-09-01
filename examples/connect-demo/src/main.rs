use dioxus::prelude::*;
use dioxus_solana::rpc::{use_get_version, use_slot_subscription, RpcProvider};
use dioxus_solana::{use_wallet, AppIdentity, Cluster, WalletProvider, WalletState};

mod components;
use components::Connected;

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        style { dangerous_inner_html: STYLE }
        // RpcProvider owns the cluster (endpoints + chain); WalletProvider reads it.
        RpcProvider { cluster: Cluster::Devnet,
            WalletProvider {
                autoconnect: true,
                app_identity: Some(AppIdentity::named("dioxus-solana demo")),
                Demo {}
            }
        }
    }
}

#[component]
fn Demo() -> Element {
    let wallet = use_wallet();
    let slot = use_slot_subscription();
    let version = use_get_version(); // one-shot fetch hook (getVersion)

    let (slot_class, slot_title, slot_label, slot_value) = match slot() {
        Some(n) => ("slot", "Live devnet slot", "devnet slot", n.to_string()),
        None => (
            "slot slot-pending",
            "Connecting to devnet",
            "devnet",
            "…".into(),
        ),
    };

    rsx! {
        main { class: "page",
            section { class: "card",
                header { class: "card-head",
                    div { class: "logo", "◎" }
                    div {
                        h1 { "dioxus-solana" }
                        p { class: "subtitle", "Wallet Standard connect demo" }
                    }
                    div { class: "{slot_class}", title: "{slot_title}",
                        span { class: "slot-dot" }
                        span { class: "slot-label", "{slot_label}" }
                        span { class: "slot-value", "{slot_value}" }
                    }
                }

                match wallet.state() {
                    WalletState::Connected(_) => rsx! {
                        Connected { pubkey: wallet.pubkey().unwrap() }
                    },

                    WalletState::Connecting => rsx! {
                        div { class: "status",
                            span { class: "spinner" }
                            "Connecting…"
                        }
                    },

                    other => {
                        let wallets = wallet.wallets();
                        rsx! {
                            p { class: "prompt", "Choose a wallet to connect" }
                            div { class: "wallet-list",
                                for info in &wallets {
                                    button {
                                        key: "{info.name}",
                                        class: "wallet",
                                        onclick: {
                                            let name = info.name.clone();
                                            move |_| wallet.connect(name.clone())
                                        },
                                        if let Some(icon) = &info.icon {
                                            img { class: "wallet-icon", src: "{icon}", alt: "" }
                                        } else {
                                            span { class: "wallet-icon wallet-icon-fallback",
                                                "{initial(&info.name)}"
                                            }
                                        }
                                        span { class: "wallet-name", "{info.name}" }
                                        span { class: "wallet-arrow", "→" }
                                    }
                                }
                            }
                            if wallets.is_empty() {
                                p { class: "empty", "No wallets detected. On desktop, install a Solana wallet extension. On Android Chrome, Mobile Wallet Adapter appears automatically." }
                            }
                            if let WalletState::Error(e) = other {
                                div { class: "error", "{e}" }
                            }
                        }
                    }
                }
            }
            footer { class: "foot",
                "Powered by "
                a { href: "https://github.com/Dodecahedr0x/dioxus-solana", "dioxus-solana" }
                if let Some(v) = version().flatten() {
                    span { " · node v{v.solana_core}" }
                }
            }
        }
    }
}

/// Format lamports as SOL with 4 decimals.
fn lamports_to_sol(lamports: u64) -> String {
    format!("{:.4}", lamports as f64 / 1_000_000_000.0)
}

/// Shorten a base58 string to `head…tail` for display.
fn truncate(s: &str) -> String {
    if s.len() <= 12 {
        return s.to_string();
    }
    format!("{}…{}", &s[..6], &s[s.len() - 6..])
}

/// First character of a wallet name, uppercased, for the icon fallback.
fn initial(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default()
}

const STYLE: &str = include_str!("style.css");
