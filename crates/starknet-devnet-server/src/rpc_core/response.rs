use serde::{Deserialize, Serialize};

use crate::rpc_core::error::RpcError;
use crate::rpc_core::request::{Id, Version};

/// Response of a _single_ rpc call
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcResponse {
    // JSON RPC version
    jsonrpc: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Id>,
    #[serde(flatten)]
    pub(crate) result: ResponseResult,
}

impl RpcResponse {
    pub fn new(id: Id, content: impl Into<ResponseResult>) -> Self {
        RpcResponse { jsonrpc: Version::V2, id: Some(id), result: content.into() }
    }

    pub fn invalid_request(id: Id) -> Self {
        Self::new(id, RpcError::invalid_request())
    }

    pub fn from_rpc_error(e: RpcError, id: Id) -> Self {
        Self { jsonrpc: Version::V2, id: Some(id), result: ResponseResult::Error(e) }
    }
}

/// Represents the result of a call either success or error
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ResponseResult {
    #[serde(rename = "result")]
    Success(serde_json::Value),
    #[serde(rename = "error")]
    Error(RpcError),
}

impl ResponseResult {
    pub fn error(error: RpcError) -> Self {
        ResponseResult::Error(error)
    }
}

impl From<RpcError> for ResponseResult {
    fn from(err: RpcError) -> Self {
        ResponseResult::error(err)
    }
}
/// Synchronous response
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
    /// single json rpc response
    Single(RpcResponse),
    /// batch of several responses
    Batch(Vec<RpcResponse>),
}

impl Response {
    /// Creates new [Response] with the given [Error]
    pub fn error(error: RpcError) -> Self {
        RpcResponse::new(Id::Null, ResponseResult::Error(error)).into()
    }
}

impl From<RpcError> for Response {
    fn from(err: RpcError) -> Self {
        Response::error(err)
    }
}

impl From<RpcResponse> for Response {
    fn from(resp: RpcResponse) -> Self {
        Response::Single(resp)
    }
}
