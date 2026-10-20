use dioxus::prelude::*;
use dioxus_solana::format::{format_sol, short_address};
use dioxus_solana::{use_wallet, AccountMeta, Instruction, Pubkey};

/// SPL Memo program (v2).
const MEMO_PROGRAM: Pubkey = Pubkey::from_str_const("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");

/// A single Memo instruction signed by `payer`.
fn memo_ix(payer: Pubkey, memo: &str) -> Instruction {
    Instruction {
        program_id: MEMO_PROGRAM,
        accounts: vec![AccountMeta::new_readonly(payer, true)],
        data: memo.as_bytes().to_vec(),
    }
}

/// Connected-account panel. Only mounts while connected, so its RPC hooks
/// (`use_balance`, `use_account`) run for exactly that lifetime.
#[component]
pub fn Connected(pubkey: Pubkey) -> Element {
    use dioxus_solana::rpc::{use_account, use_balance, use_rpc};

    let wallet = use_wallet();
    let rpc = use_rpc();
    let balance = use_balance(pubkey);
    let account = use_account(pubkey);
    let mut last_sig = use_signal(|| Option::<String>::None);
    let mut airdrop = use_signal(|| Option::<String>::None);
    let mut memo = use_signal(|| "gm from dioxus-solana".to_string());
    let mut memo_status = use_signal(|| Option::<String>::None);

    let owner = account().map(|a| short_address(&a.owner));

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
                                Ok(sig) => airdrop.set(Some(format!("Airdropped · {}", short_address(&sig)))),
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
                            // Signed by the wallet, then submitted through our own RPC, so the
                            // send targets the same cluster the blockhash came from.
                            memo_status.set(Some("Awaiting signature…".into()));
                            match wallet.send_instructions(&rpc, &[memo_ix(pubkey, &text)]).await {
                                Ok(sig) => memo_status.set(Some(format!("Memo sent · {}", short_address(&sig)))),
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
                code { "{short_address(&sig)}" }
            }
        }
    }
}
