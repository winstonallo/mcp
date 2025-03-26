use serde::{Deserialize, Serialize};

use super::jsonrpc::{Method, NumberOrString, Request};

#[derive(Debug, Serialize, Deserialize)]
pub struct RootCapability {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SamplingCapability {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentalCapability {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<ExperimentalCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: Capabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}

pub struct InitializeRequest {}

impl InitializeRequest {
    pub fn new(protocol_version: &str, name: &str, version: &str, capabilities: Option<Capabilities>) -> Request {
        let caps = capabilities.unwrap_or(Capabilities {
            roots: Some(RootCapability { list_changed: Some(true) }),
            sampling: None,
            experimental: None,
        });
        let params = InitializeParams {
            protocol_version: protocol_version.to_string(),
            capabilities: caps,
            client_info: ClientInfo {
                name: name.to_string(),
                version: version.to_string(),
            },
        };

        let params_value = serde_json::to_value(params).expect("could not serialize initialize params");

        Request {
            jsonrpc: "2.0".to_string(),
            id: Some(NumberOrString::Number(1)),
            method: "initialize".to_string(),
            params: Some(params_value),
        }
    }
}
