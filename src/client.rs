pub struct Client {
    pub name: String,
    pub version: String,
    pub protocol_version: String,
}

impl Client {
    pub fn new(name: &str, version: &str, protocol_version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            protocol_version: protocol_version.to_string(),
        }
    }
}
