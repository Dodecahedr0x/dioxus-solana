use dioxus::prelude::*;
use dioxus_solana::format::{format_sol, short_address};
use dioxus_solana::rpc::{use_balance, use_rpc, use_slot_subscription, RpcProvider};
use dioxus_solana::{
    use_wallet, AccountMeta, AppIdentity, Cluster, Instruction, Pubkey, WalletProvider, WalletState,
};

/// SPL Memo program (v2).
const MEMO_PROGRAM: Pubkey = Pubkey::from_str_const("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

fn main() {
    dioxus::launch(Root);
}

#[component]
fn Root() -> Element {
    rsx! {
        style { dangerous_inner_html: STYLE }
        RpcProvider { cluster: Cluster::Devnet,
            WalletProvider {
                autoconnect: false,
                app_identity: Some(AppIdentity {
                    name: "dioxus-solana mobile".into(),
                    uri: "https://dioxuslabs.com".into(),
                    icon: "favicon.ico".into(),
                }),
                App {}
            }
        }
    }
}

#[component]
fn App() -> Element {
    let wallet = use_wallet();
    let slot = use_slot_subscription();
    let slot_value = slot().map_or("…".to_string(), |s| s.to_string());

    rsx! {
        main { class: "page",
            section { class: "card",
                header { class: "card-head",
                    div { class: "logo", "◎" }
                    div {
                        h1 { "dioxus-solana" }
                        p { class: "subtitle", "Cross-platform wallet demo" }
                    }
                    div { class: "slot", title: "Live slot ({wallet.cluster().chain_id()})",
                        span { class: "slot-dot" }
                        span { class: "slot-label", "slot" }
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
                            "Opening wallet…"
                        }
                    },
                    other => {
                        let wallets = wallet.wallets();
                        rsx! {
                            p { class: "prompt",
                                "Connect a Solana wallet (Android MWA, iOS Phantom, or a browser extension on web)."
                            }
                            div { class: "wallet-list",
                                for info in &wallets {
                                    button {
                                        key: "{info.name}",
                                        class: "wallet",
                                        onclick: {
                                            let name = info.name.clone();
                                            move |_| wallet.connect(name.clone())
                                        },
                                        span { class: "wallet-icon wallet-icon-fallback",
                                            "{initial(&info.name)}"
                                        }
                                        span { class: "wallet-name", "{info.name}" }
                                        span { class: "wallet-arrow", "→" }
                                    }
                                }
                            }
                            if wallets.is_empty() {
                                p { class: "empty",
                                    "No wallet found. On web, install a Solana wallet extension; on mobile, install Phantom (iOS) or an MWA wallet (Android)."
                                }
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
            }
        }
    }
}

#[component]
fn Connected(pubkey: Pubkey) -> Element {
    let wallet = use_wallet();
    let rpc = use_rpc();
    // Fetched on mount, then kept current over a WebSocket subscription.
    let balance = use_balance(pubkey);
    let mut last_sig = use_signal(|| Option::<String>::None);
    let mut memo = use_signal(|| "gm from dioxus-solana mobile".to_string());
    let mut status = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "account",
            span { class: "label", "Connected account" }
            code { class: "pubkey", "{short_address(&pubkey)}" }
            div { class: "meta",
                span { class: "chip", "{wallet.cluster().chain_id()}" }
                match balance() {
                    Some(lamports) => rsx! {
                        span { class: "chip chip-balance", "{format_sol(lamports)} SOL" }
                    },
                    None => rsx! { span { class: "chip chip-muted", "… SOL" } },
                }
            }
        }

        div { class: "actions",
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    spawn(async move {
                        status.set(Some("Awaiting signature…".into()));
                        match wallet.sign_message(b"gm from dioxus mobile".to_vec()).await {
                            Ok(sig) => {
                                last_sig.set(Some(sig.to_string()));
                                status.set(Some("Message signed".into()));
                            }
                            Err(e) => status.set(Some(format!("Sign failed: {e}"))),
                        }
                    });
                },
                "Sign message"
            }
            button {
                class: "btn btn-ghost",
                onclick: move |_| wallet.disconnect(),
                "Disconnect"
            }
        }

        div { class: "memo",
            input {
                class: "memo-input",
                value: "{memo}",
                placeholder: "memo text",
                oninput: move |e| memo.set(e.value()),
            }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    let rpc = rpc.clone();
                    spawn(async move {
                        let text = memo.peek().clone();
                        if text.is_empty() {
                            status.set(Some("Enter some memo text".into()));
                            return;
                        }
                        status.set(Some("Awaiting wallet…".into()));
                        match wallet.send_instructions(&rpc, &[memo_ix(pubkey, &text)]).await {
                            Ok(sig) => {
                                status.set(Some(format!("Memo sent · {}", short_address(&sig))));
                                last_sig.set(Some(sig.to_string()));
                            }
                            Err(e) => status.set(Some(format!("Send failed: {e}"))),
                        }
                    });
                },
                "Send memo"
            }
        }

        if let Some(msg) = status() {
            div { class: "note", "{msg}" }
        }
        if let Some(sig) = last_sig() {
            div { class: "signature",
                span { class: "label", "Last signature" }
                code { "{short_address(&sig)}" }
            }
        }
    }
}

fn memo_ix(payer: Pubkey, memo: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM,
        accounts: vec![AccountMeta::new_readonly(payer, true)],
        data: memo.as_bytes().to_vec(),
    }
}

fn initial(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default()
}

const STYLE: &str = include_str!("style.css");
