//! Live devnet checks for the native (Android/iOS) RPC transport, run on the
//! host. Network-dependent, so ignored by default:
//! `cargo test -p dioxus-solana --features server -- --ignored`
#![cfg(not(target_arch = "wasm32"))]

use dioxus_solana::rpc::{PubsubClient, RpcClient, RpcError};
use dioxus_solana::{Cluster, Pubkey};
use futures_util::StreamExt;
use std::time::Duration;

#[tokio::test]
#[ignore = "hits devnet"]
async fn http_reads_and_rpc_errors() {
    let rpc = RpcClient::new(Cluster::Devnet.rpc_url());
    assert!(rpc.get_slot(None).await.unwrap() > 0);
    rpc.get_latest_blockhash(None).await.unwrap();
    rpc.get_balance(&Pubkey::default(), None).await.unwrap();

    // JSON-RPC errors surface as the typed response error, as on wasm.
    let err = rpc.get_balance("not-a-pubkey", None).await.unwrap_err();
    assert!(
        matches!(*err, RpcError::RpcResponseError { .. }),
        "got {err:?}"
    );
}

#[tokio::test]
#[ignore = "hits devnet"]
async fn pubsub_slot_subscription_streams() {
    let ws = PubsubClient::connect(Cluster::Devnet.ws_url()).unwrap();
    let mut slots = ws.slot_subscribe().await.unwrap();
    let first = tokio::time::timeout(Duration::from_secs(20), slots.next())
        .await
        .expect("no slot notification within 20s")
        .expect("stream ended")
        .unwrap();
    assert!(first.slot > 0);
}

#[test]
fn pubsub_requires_a_tokio_runtime() {
    let err = PubsubClient::connect("wss://example.invalid").unwrap_err();
    assert!(err.to_string().contains("tokio runtime"), "got {err}");
}
