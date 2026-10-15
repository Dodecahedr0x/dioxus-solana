//! Browser transport: spume's `fetch` and `WebSocket` providers.

use dioxus_solana_web::rpc::spume::{provider::HttpProvider, pubsub_provider::PubsubProvider};
use serde::de::DeserializeOwned;
use serde_json::Value;
use solana_rpc_client_types::request::RpcRequest;

use super::RpcResult;

pub use dioxus_solana_web::rpc::Subscription;

#[derive(Clone)]
pub struct Http(HttpProvider);

impl Http {
    pub fn new(url: &str) -> Self {
        Self(HttpProvider::new(url))
    }

    pub async fn send<R: DeserializeOwned>(
        &self,
        request: RpcRequest,
        params: Value,
    ) -> RpcResult<R> {
        self.0.send(request, params).await
    }
}

#[derive(Clone)]
pub struct Pubsub(PubsubProvider);

impl Pubsub {
    pub fn connect(url: String) -> RpcResult<Self> {
        PubsubProvider::connect(url).map(Self)
    }

    pub fn url(&self) -> &str {
        self.0.url()
    }

    pub async fn subscribe<T: DeserializeOwned + 'static>(
        &self,
        subscribe_method: &'static str,
        unsubscribe_method: &'static str,
        params: Value,
    ) -> RpcResult<Subscription<T>> {
        self.0
            .subscribe(subscribe_method, unsubscribe_method, params)
            .await
    }
}
