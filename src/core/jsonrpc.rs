#![allow(unused)]

use serde::{Deserialize, Serialize};

const JSON_RPC_VERSION: &str = "2.0";

#[repr(i16)]
#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    ServerError(i16), // -32000 to -32099
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorData {
    pub code: ErrorCode,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub enum Method {
    Initialize,            // initialize
    Ping,                  // ping
    Cancelled,             // notifications/cancelled
    Progress,              // notifications/progress
    PromptsList,           // prompts/list
    PromptsGet,            // prompts/get
    PromptListChanged,     // notifications/prompts/list_changed
    ResourcesList,         // resources/list
    ResourcesRead,         // resources/read
    ResourcesListChanged,  // notifications/resources/list_changed
    ResourcesSubscribe,    // resources/subscribe
    ResourcesUpdated,      // notifications/resources/updated
    ToolsList,             // tools/list
    ToolsCall,             // tools/call
    ToolsListChanged,      // notifications/tools/list_changed
    Completion,            // completion/complete
    LoggingSetLevel,       // logging/setLevel,
    Message,               // notifications/message
    RootsList,             // roots/list
    RootsListChanged,      // notifications/roots/list_changed
    SamplingCreateMessage, // sampling/createMessage
}

#[derive(Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<NumberOrString>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct Response {
    pub jsonrpc: String,
    pub id: Option<NumberOrString>,
    pub result: Option<serde_json::Value>,
    pub error: Option<ErrorData>,
}

#[derive(Debug)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub jsonrpc: String,
    pub id: Option<NumberOrString>,
    pub error: ErrorData,
}

#[derive(Debug)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
    Error(Error),
    Null,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Raw {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<NumberOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorData>,
}

impl TryFrom<Raw> for Message {
    type Error = String;

    fn try_from(value: Raw) -> Result<Self, <Self as TryFrom<Raw>>::Error> {
        if value.error.is_some() {
            return Ok(Message::Error(Error {
                jsonrpc: value.jsonrpc,
                id: value.id,
                error: value.error.unwrap(),
            }));
        }

        if value.result.is_some() {
            return Ok(Message::Response(Response {
                jsonrpc: value.jsonrpc,
                id: value.id,
                result: value.result,
                error: None,
            }));
        }

        if let Some(method) = value.method {
            if value.id.is_none() {
                return Ok(Message::Notification(Notification {
                    jsonrpc: value.jsonrpc,
                    method,
                    params: value.params,
                }));
            } else {
                return Ok(Message::Request(Request {
                    jsonrpc: value.jsonrpc,
                    id: value.id,
                    method,
                    params: value.params,
                }));
            }
        }

        if value.id.is_none() && value.result.is_none() && value.error.is_none() {
            return Ok(Message::Null);
        }

        Err(format!(
            "invalid JSON-RPC format: id: {:?}, method: {:?}, result: {:?}, error: {:?}",
            value.id, value.method, value.result, value.error
        ))
    }
}
