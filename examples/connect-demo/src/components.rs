use dioxus::prelude::*;
use dioxus_solana::rpc::{use_account, use_balance, use_rpc};
use dioxus_solana::{serialize_transaction, use_wallet, Pubkey};

use solana_hash::Hash;
use solana_instruction::{AccountMeta, Instruction};
use solana_message::{Message, VersionedMessage};
use solana_signature::Signature;
use solana_transaction::versioned::VersionedTransaction;

use crate::{lamports_to_sol, truncate};

/// SPL Memo program (v2).
const MEMO_PROGRAM: Pubkey = Pubkey::from_str_const("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

/// Build an unsigned legacy transaction with a single Memo instruction.
/// The wallet fills in the signature when it signs-and-sends.
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

/// Connected-account panel. Only mounts while connected, so its RPC hooks
/// (`use_balance`, `use_account`) run for exactly that lifetime.
#[component]
pub fn Connected(pubkey: Pubkey) -> Element {
    let wallet = use_wallet();
    let rpc = use_rpc();
    let balance = use_balance(pubkey);
    let account = use_account(pubkey);
    let mut last_sig = use_signal(|| Option::<String>::None);
    let mut airdrop = use_signal(|| Option::<String>::None);
    let mut memo = use_signal(|| "gm from dioxus-solana".to_string());
    let mut memo_status = use_signal(|| Option::<String>::None);

    let owner = account().map(|a| truncate(&a.owner));

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
                if let Some(owner) = &owner {
                    span { class: "chip chip-muted", "owner {owner}" }
                }
            }
        }

        div { class: "actions",
            button {
                class: "btn btn-primary",
                onclick: move |_| async move {
                    if let Ok(sig) = wallet.sign_message(b"gm from dioxus".to_vec()).await {
                        last_sig.set(Some(sig.to_string()));
                    }
                },
                "Sign message"
            }
            button {
                class: "btn btn-secondary",
                title: "Devnet faucet via use_rpc().request_airdrop",
                onclick: {
                    let rpc = rpc.clone();
                    move |_| {
                        let rpc = rpc.clone();
                        async move {
                            airdrop.set(Some("Requesting 1 SOL…".into()));
                            match rpc.request_airdrop(pubkey.to_string(), 1_000_000_000, None).await {
                                Ok(sig) => airdrop.set(Some(format!("Airdropped · {}", truncate(&sig)))),
                                Err(e) => airdrop.set(Some(format!("Airdrop failed: {e}"))),
                            }
                        }
                    }
                },
                "Airdrop 1 SOL"
            }
            button {
                class: "btn btn-ghost",
                onclick: move |_| wallet.disconnect(),
                "Disconnect"
            }
        }

        // Send an on-chain memo: build a tx, then have the wallet sign & send it.
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
                        async move {
                            let text = memo.peek().clone();
                            if text.is_empty() {
                                memo_status.set(Some("Enter some memo text".into()));
                                return;
                            }
                            memo_status.set(Some("Fetching blockhash…".into()));
                            let blockhash = match rpc.get_latest_blockhash(None).await {
                                Ok(resp) => resp.value.blockhash,
                                Err(e) => {
                                    memo_status.set(Some(format!("Blockhash failed: {e}")));
                                    return;
                                }
                            };
                            let Ok(blockhash) = blockhash.parse::<Hash>() else {
                                memo_status.set(Some("Bad blockhash".into()));
                                return;
                            };
                            let tx = memo_tx(pubkey, &text, blockhash);

                            // Sign with the wallet, then submit through our own RPC — so the
                            // send targets the same cluster the blockhash came from. Relying on
                            // the wallet's sign-and-send can hit "invalid blockhash" when the
                            // extension is pointed at a different cluster.
                            memo_status.set(Some("Awaiting signature…".into()));
                            let signed = match wallet.sign_transaction(tx).await {
                                Ok(t) => t,
                                Err(e) => {
                                    memo_status.set(Some(format!("Sign failed: {e}")));
                                    return;
                                }
                            };
                            let bytes = match serialize_transaction(&signed) {
                                Ok(b) => b,
                                Err(e) => {
                                    memo_status.set(Some(format!("Encode failed: {e}")));
                                    return;
                                }
                            };
                            memo_status.set(Some("Sending…".into()));
                            match rpc.send_transaction(bs58::encode(bytes).into_string(), None).await {
                                Ok(sig) => memo_status.set(Some(format!("Memo sent · {}", truncate(&sig)))),
                                Err(e) => memo_status.set(Some(format!("Send failed: {e}"))),
                            }
                        }
                    }
                },
                "Send memo"
            }
        }

        if let Some(msg) = memo_status() {
            div { class: "note", "{msg}" }
        }
        if let Some(msg) = airdrop() {
            div { class: "note", "{msg}" }
        }
        if let Some(sig) = last_sig() {
            div { class: "signature",
                span { class: "label", "Signature" }
                code { "{truncate(&sig)}" }
            }
        }
    }
}
