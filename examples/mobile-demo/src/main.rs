use dioxus::prelude::*;
use dioxus_solana::rpc::{use_rpc, RpcClient, RpcProvider};
use dioxus_solana::{
    use_wallet, AppIdentity, Cluster, Pubkey, WalletError, WalletProvider, WalletState,
};
use solana_hash::Hash;
use solana_instruction::{AccountMeta, Instruction};
use solana_message::{Message, VersionedMessage};
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;
use std::sync::Arc;

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

    rsx! {
        main { class: "page",
            section { class: "card",
                header { class: "card-head",
                    div { class: "logo", "◎" }
                    div {
                        h1 { "dioxus-solana" }
                        p { class: "subtitle", "Native mobile wallet demo" }
                    }
                    div { class: "slot", title: "Signing cluster",
                        span { class: "slot-dot" }
                        span { class: "slot-label", "cluster" }
                        span { class: "slot-value", "{wallet.cluster().chain_id()}" }
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
                                "Connect the on-device Solana wallet (Android MWA or iOS Phantom)."
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
                                    "No mobile wallet bridge on this target. Run with "
                                    code { "dx serve --android" }
                                    " or "
                                    code { "dx serve --ios" }
                                    "."
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
    let mut balance = use_signal(|| Option::<u64>::None);
    let mut last_sig = use_signal(|| Option::<String>::None);
    let mut memo = use_signal(|| "gm from dioxus-solana mobile".to_string());
    let mut status = use_signal(|| Option::<String>::None);
    let mut tick = use_signal(|| 0u32);

    // Refresh balance on connect and after each successful action.
    let rpc_for_balance = rpc.clone();
    use_effect(move || {
        let _ = tick();
        let rpc = rpc_for_balance.clone();
        spawn(async move {
            match rpc.get_balance(&pubkey).await {
                Ok(lamports) => balance.set(Some(lamports)),
                Err(e) => status.set(Some(format!("Balance failed: {e}"))),
            }
        });
    });

    rsx! {
        div { class: "account",
            span { class: "label", "Connected account" }
            code { class: "pubkey", "{truncate(&pubkey.to_string())}" }
            div { class: "meta",
                span { class: "chip", "{wallet.cluster().chain_id()}" }
                match balance() {
                    Some(lamports) => rsx! {
                        span { class: "chip chip-balance", "{lamports_to_sol(lamports)} SOL" }
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
                onclick: move |_| {
                    tick.set(tick() + 1);
                },
                "Refresh balance"
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
                onclick: {
                    let rpc = rpc.clone();
                    move |_| {
                        let rpc = rpc.clone();
                        spawn(async move {
                            let text = memo.peek().clone();
                            if text.is_empty() {
                                status.set(Some("Enter some memo text".into()));
                                return;
                            }
                            status.set(Some("Fetching blockhash…".into()));
                            let blockhash = match rpc.get_latest_blockhash().await {
                                Ok(h) => h,
                                Err(e) => {
                                    status.set(Some(format!("Blockhash failed: {e}")));
                                    return;
                                }
                            };
                            let tx = memo_tx(pubkey, &text, blockhash);
                            status.set(Some("Awaiting wallet…".into()));
                            match send_memo(&wallet, &rpc, tx).await {
                                Ok(sig) => {
                                    status.set(Some(format!(
                                        "Memo sent · {}",
                                        truncate(&sig)
                                    )));
                                    last_sig.set(Some(sig));
                                    tick.set(tick() + 1);
                                }
                                Err(e) => status.set(Some(e)),
                            }
                        });
                    }
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
                code { "{truncate(&sig)}" }
            }
        }
    }
}

/// Android MWA can sign-and-send; iOS Phantom signs only — then we submit via RPC.
async fn send_memo(
    wallet: &dioxus_solana::WalletHandle,
    rpc: &Arc<RpcClient>,
    tx: VersionedTransaction,
) -> Result<String, String> {
    match wallet.sign_and_send(tx.clone()).await {
        Ok(sig) => Ok(sig.to_string()),
        Err(WalletError::Feature(_)) => {
            let signed = wallet
                .sign_transaction(tx)
                .await
                .map_err(|e| format!("Sign failed: {e}"))?;
            rpc.send_transaction(&signed)
                .await
                .map(|sig| sig.to_string())
                .map_err(|e| format!("Send failed: {e}"))
        }
        Err(e) => Err(format!("Send failed: {e}")),
    }
}

fn memo_tx(payer: Pubkey, memo: &str, blockhash: Hash) -> VersionedTransaction {
    let ix = Instruction {
        program_id: MEMO_PROGRAM,
        accounts: vec![AccountMeta::new_readonly(payer, true)],
        data: memo.as_bytes().to_vec(),
    };
    let message = Message::new_with_blockhash(&[ix], Some(&payer), &blockhash);
    let signatures = vec![Signature::default(); message.header.num_required_signatures as usize];
    VersionedTransaction {
        signatures,
        message: VersionedMessage::Legacy(message),
    }
}

fn lamports_to_sol(lamports: u64) -> String {
    format!("{:.4}", lamports as f64 / 1_000_000_000.0)
}

fn truncate(s: &str) -> String {
    if s.len() <= 12 {
        return s.to_string();
    }
    format!("{}…{}", &s[..6], &s[s.len() - 6..])
}

fn initial(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default()
}

const STYLE: &str = include_str!("style.css");
