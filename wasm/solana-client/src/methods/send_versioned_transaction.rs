use std::str::FromStr;

use solana_extra_wasm::transaction_status::UiTransactionEncoding;
use solana_sdk::{signature::Signature, transaction::VersionedTransaction};

use crate::utils::rpc_config::{serialize_and_encode, RpcSendTransactionConfig};
use crate::{ClientRequest, ClientResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVersionedTransactionRequest {
    transaction: VersionedTransaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<RpcSendTransactionConfig>,
}

impl SendVersionedTransactionRequest {
    pub fn new(transaction: VersionedTransaction) -> Self {
        Self {
            transaction,
            config: None,
        }
    }
    pub fn new_with_config(
        transaction: VersionedTransaction,
        config: RpcSendTransactionConfig,
    ) -> Self {
        Self {
            transaction,
            config: Some(config),
        }
    }
}

impl From<SendVersionedTransactionRequest> for serde_json::Value {
    fn from(value: SendVersionedTransactionRequest) -> Self {
        let encoding = match value.config {
            Some(ref c) => c.encoding.unwrap_or(UiTransactionEncoding::Base64),
            None => UiTransactionEncoding::Base64,
        };

        let serialized_encoded =
            serialize_and_encode::<VersionedTransaction>(&value.transaction, encoding).unwrap();

        match value.config {
            Some(config) => serde_json::json!([serialized_encoded, config]),
            None => serde_json::json!([serialized_encoded]),
        }
    }
}

impl From<SendVersionedTransactionRequest> for ClientRequest {
    fn from(val: SendVersionedTransactionRequest) -> Self {
        let mut request = ClientRequest::new("sendTransaction");
        let params = val.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVersionedTransactionResponse(Signature);

impl From<SendVersionedTransactionResponse> for Signature {
    fn from(val: SendVersionedTransactionResponse) -> Self {
        val.0
    }
}

impl From<ClientResponse> for SendVersionedTransactionResponse {
    fn from(response: ClientResponse) -> Self {
        let signature = response.result.as_str().expect("invalid response");
        let signature = Signature::from_str(signature).expect("invalid signature");

        SendVersionedTransactionResponse(signature)
    }
}
