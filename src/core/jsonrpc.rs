#![allow(unused)]

use serde::{Deserialize, Serialize};

const JSON_RPC_VERSION: &str = "2.0";

#[repr(i16)]
#[derive(Deserialize, Serialize)]
pub enum ErrorCode {
    ServerError(i16), // -32000 to -32099
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
}

#[derive(Deserialize, Serialize)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    code: ErrorCode,
    message: String,
    data: Option<serde_json::Value>,
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

pub struct Request {
    jsonrpc: String,
    id: NumberOrString,
    method: Method,
    params: Option<serde_json::Value>,
}

pub struct Response {
    jsonrpc: String,
    id: NumberOrString,
    result: Option<serde_json::Value>,
    error: Option<Error>,
}

pub struct Notification {
    jsonrpc: String,
    method: Method,
    params: Option<serde_json::Value>,
}

pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
    Error(Error),
    Null,
}

#[derive(Serialize, Deserialize)]
struct Raw {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<NumberOrString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Error>,
}

impl Response {
    pub fn new(id: NumberOrString, result: Option<serde_json::Value>, error: Option<Error>) -> Self {
        assert!(result.is_none() || error.is_none());

        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            result,
            error,
        }
    }
}

impl Notification {
    pub fn new(method: Method, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method,
            params,
        }
    }
}

impl Request {
    pub fn new(method: Method, id: NumberOrString, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method,
            id,
            params,
        }
    }
}

impl Error {
    pub fn new(code: ErrorCode, message: &str, data: Option<serde_json::Value>) -> Result<Self, String> {
        match code {
            ErrorCode::ServerError(code) => {
                if (-32099..=-32000).contains(&code) {
                    Ok(Self {
                        code: ErrorCode::ServerError(code),
                        message: message.to_string(),
                        data,
                    })
                } else {
                    Err(format!("ServerError code must be in range -32099..=-32000 (got {code})"))
                }
            }
            _ => Ok(Self {
                code,
                message: message.to_string(),
                data,
            }),
        }
    }
}
