pub struct Server {
    executable_path: String,
}

impl Server {
    pub fn new(executable_path: &str) -> Self {
        Self {
            executable_path: executable_path.to_string(),
        }
    }

    pub fn executable_path(&self) -> &str {
        &self.executable_path
    }
}
