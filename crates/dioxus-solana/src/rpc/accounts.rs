//! Higher-level account queries: decoded program accounts and token holdings.

use dioxus::prelude::*;
use futures_util::future::join;
use solana_account_decoder_client_types::{UiAccountData, UiAccountEncoding, UiDataSliceConfig};
use solana_pubkey::Pubkey;

use super::config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcTokenAccountsFilter};
use super::filter::RpcFilterType;
use super::response::{RpcKeyedAccount, UiTokenAmount};
use super::{base64_account_config, use_fetch_with, FetchError, RpcClient, RpcResult};

/// The SPL Token program.
pub const TOKEN_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
/// The SPL Token-2022 program.
pub const TOKEN_2022_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

/// One SPL token account, as returned by [`RpcClient::token_accounts`].
#[derive(Debug, Clone, PartialEq)]
pub struct TokenHolding {
    /// The token account's address.
    pub address: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: UiTokenAmount,
    /// [`TOKEN_PROGRAM_ID`] or [`TOKEN_2022_PROGRAM_ID`].
    pub program_id: Pubkey,
}

impl TokenHolding {
    /// Parse a `jsonParsed` token account; `None` for anything else.
    fn from_keyed(keyed: &RpcKeyedAccount) -> Option<Self> {
        let UiAccountData::Json(parsed) = &keyed.account.data else {
            return None;
        };
        let info = parsed.parsed.get("info")?;
        let key = |field: &str| info.get(field)?.as_str()?.parse::<Pubkey>().ok();
        Some(Self {
            address: keyed.pubkey.parse().ok()?,
            mint: key("mint")?,
            owner: key("owner")?,
            amount: serde_json::from_value(info.get("tokenAmount")?.clone()).ok()?,
            program_id: keyed.account.owner.parse().ok()?,
        })
    }
}

/// Decode raw program accounts into `(address, T)`, skipping undecodable ones.
fn decode_keyed<T>(
    accounts: Vec<RpcKeyedAccount>,
    decode: impl Fn(&[u8]) -> Option<T>,
) -> Vec<(Pubkey, T)> {
    accounts
        .into_iter()
        .filter_map(|keyed| {
            let bytes = keyed.account.data.decode()?;
            Some((keyed.pubkey.parse().ok()?, decode(&bytes)?))
        })
        .collect()
}

impl RpcClient {
    /// Every SPL token account owned by `owner`, across both the Token and
    /// Token-2022 programs (fetched concurrently).
    pub async fn token_accounts(&self, owner: &Pubkey) -> RpcResult<Vec<TokenHolding>> {
        let config = || {
            Some(RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::JsonParsed),
                ..Default::default()
            })
        };
        let by_program = |program: Pubkey| {
            self.get_token_accounts_by_owner(
                owner,
                RpcTokenAccountsFilter::ProgramId(program.to_string()),
                config(),
            )
        };
        let (legacy, token_2022) = join(
            by_program(TOKEN_PROGRAM_ID),
            by_program(TOKEN_2022_PROGRAM_ID),
        )
        .await;
        Ok(legacy?
            .value
            .iter()
            .chain(token_2022?.value.iter())
            .filter_map(TokenHolding::from_keyed)
            .collect())
    }

    /// The account at `address` decoded from its raw bytes with `decode`;
    /// `Ok(None)` if it does not exist or does not decode.
    pub async fn account_data<T>(
        &self,
        address: &Pubkey,
        decode: impl FnOnce(&[u8]) -> Option<T>,
    ) -> RpcResult<Option<T>> {
        let account = self
            .get_account_info(address, Some(base64_account_config()))
            .await?
            .value;
        Ok(account
            .and_then(|a| a.data.decode())
            .and_then(|b| decode(&b)))
    }

    /// Addresses of the accounts owned by `program_id` matching `filters`,
    /// without downloading their data.
    pub async fn program_account_keys(
        &self,
        program_id: &Pubkey,
        filters: Vec<RpcFilterType>,
    ) -> RpcResult<Vec<Pubkey>> {
        let config = RpcProgramAccountsConfig {
            filters: (!filters.is_empty()).then_some(filters),
            account_config: RpcAccountInfoConfig {
                data_slice: Some(UiDataSliceConfig {
                    offset: 0,
                    length: 0,
                }),
                ..base64_account_config()
            },
            ..Default::default()
        };
        let accounts = self
            .get_program_accounts(program_id, Some(config))
            .await?
            .parse_value();
        Ok(accounts
            .iter()
            .filter_map(|k| k.pubkey.parse().ok())
            .collect())
    }

    /// Accounts owned by `program_id` matching `filters`, decoded from their raw
    /// bytes with `decode` (undecodable accounts are skipped).
    pub async fn program_accounts<T>(
        &self,
        program_id: &Pubkey,
        filters: Vec<RpcFilterType>,
        decode: impl Fn(&[u8]) -> Option<T>,
    ) -> RpcResult<Vec<(Pubkey, T)>> {
        let config = RpcProgramAccountsConfig {
            filters: (!filters.is_empty()).then_some(filters),
            account_config: base64_account_config(),
            ..Default::default()
        };
        let accounts = self
            .get_program_accounts(program_id, Some(config))
            .await?
            .parse_value();
        Ok(decode_keyed(accounts, decode))
    }
}

/// Every SPL token account (Token and Token-2022) owned by `owner`. Refetches
/// when `owner` or the cluster changes.
pub fn use_token_accounts(owner: Pubkey) -> Resource<Result<Vec<TokenHolding>, FetchError>> {
    use_fetch_with(
        owner,
        |c, owner| async move { c.token_accounts(&owner).await },
    )
}

/// Accounts owned by `program_id` matching `filters`, decoded with `decode` —
/// the reactive form of [`RpcClient::program_accounts`]. Refetches when the
/// program, the filters or the cluster change.
///
/// ```ignore
/// let counters = use_program_accounts(PROGRAM_ID, vec![RpcFilterType::DataSize(16)], |b| {
///     Counter::try_from_slice(b).ok()
/// });
/// ```
pub fn use_program_accounts<T, D>(
    program_id: Pubkey,
    filters: Vec<RpcFilterType>,
    decode: D,
) -> Resource<Result<Vec<(Pubkey, T)>, FetchError>>
where
    T: 'static,
    D: Fn(&[u8]) -> Option<T> + Clone + 'static,
{
    use_fetch_with((program_id, filters), move |c, (program_id, filters)| {
        let decode = decode.clone();
        async move { c.program_accounts(&program_id, filters, decode).await }
    })
}
