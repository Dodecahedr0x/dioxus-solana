//! Ready-made, unstyled wallet UI. Every element carries a `ds-*` class to
//! style from your own stylesheet.

use dioxus::prelude::*;

use crate::format::short_address;
use crate::{use_wallet, WalletState};

/// Connect / disconnect button with a wallet picker — the equivalent of
/// wallet-adapter's `WalletMultiButton`.
///
/// Disconnected, it shows "Select wallet" and opens a list of discovered
/// wallets (`.ds-wallet-menu`). Connected, it shows the short address and opens
/// a menu with "Disconnect". Errors from the last attempt render as
/// `.ds-wallet-error`.
#[component]
pub fn WalletButton(
    /// Extra class on the outer `.ds-wallet` element.
    #[props(default)]
    class: String,
) -> Element {
    let wallet = use_wallet();
    let mut open = use_signal(|| false);
    let state = wallet.state();

    let label = match &state {
        WalletState::Connected(acc) => short_address(&acc.pubkey()),
        WalletState::Connecting => "Connecting…".to_string(),
        _ => "Select wallet".to_string(),
    };

    rsx! {
        div { class: "ds-wallet {class}",
            button {
                class: "ds-wallet-button",
                disabled: matches!(state, WalletState::Connecting),
                onclick: move |_| open.toggle(),
                "{label}"
            }
            if open() {
                div { class: "ds-wallet-menu",
                    if state.is_connected() {
                        button {
                            class: "ds-wallet-option",
                            onclick: move |_| {
                                open.set(false);
                                wallet.disconnect();
                            },
                            "Disconnect"
                        }
                    } else {
                        for info in wallet.wallets() {
                            button {
                                key: "{info.name}",
                                class: "ds-wallet-option",
                                onclick: move |_| {
                                    open.set(false);
                                    wallet.connect(info.name.clone());
                                },
                                if let Some(icon) = &info.icon {
                                    img { class: "ds-wallet-icon", src: "{icon}", alt: "" }
                                }
                                span { "{info.name}" }
                            }
                        }
                        if wallet.wallets().is_empty() {
                            p { class: "ds-wallet-empty", "No wallets found" }
                        }
                    }
                }
            }
            if let WalletState::Error(e) = &state {
                p { class: "ds-wallet-error", "{e}" }
            }
        }
    }
}
