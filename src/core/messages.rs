#![allow(unused)]

const JSON_RPC_VERSION: &str = "2.0";

type KV = serde_json::Map<String, serde_json::Value>;

#[repr(i16)]
pub enum ErrorCode {
    ServerError(i16), // -32000 to -32099
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
}

pub enum NumberOrString {
    Number(i32),
    String(String),
}

pub struct Error {
    code: ErrorCode,
    message: String,
    data: Option<KV>,
}

impl Error {
    pub fn new(code: ErrorCode, message: &str, data: Option<KV>) -> Result<Self, String> {
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
    method: Method,
    id: NumberOrString,
    params: Option<KV>,
}

impl Request {
    pub fn new(method: Method, id: NumberOrString, params: Option<KV>) -> Self {
        Self { method, id, params }
    }
}

pub struct Response {
    id: NumberOrString,
    result: Option<KV>,
    error: Option<Error>,
}

impl Response {
    pub fn new(id: NumberOrString, result: Option<KV>, error: Option<Error>) -> Self {
        assert!(result.is_none() || error.is_none());

        Self { id, result, error }
    }
}

pub struct Notification {
    method: Method,
    params: Option<KV>,
}

impl Notification {
    pub fn new(method: Method, params: Option<KV>) -> Self {
        Self { method, params }
    }
}
