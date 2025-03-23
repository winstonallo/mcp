use crate::server::Server;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

pub struct Client {
    servers: Vec<Server>,
    processes: Vec<Arc<Mutex<Child>>>,
}

impl Client {
    pub fn new(servers: Vec<Server>) -> Self {
        Self {
            servers,
            processes: Vec::new(),
        }
    }

    pub async fn start_server(&mut self, server: Server) -> Result<(), String> {
        let proc = Command::new(server.executable_path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start serevr: {}", e))?;

        self.servers.push(server);
        self.processes.push(Arc::new(Mutex::new(proc)));

        Ok(())
    }

    pub async fn send_command(&self, server_idx: usize, command: &str) -> Result<(), String> {
        if server_idx >= self.processes.len() {
            return Err("invalid server index".to_string());
        }

        let proc = Arc::clone(&self.processes[server_idx]);
        let mut proc = proc.lock().await;

        let stdin = proc.stdin.as_mut().ok_or_else(|| "failed to open stdin".to_string())?;

        stdin
            .write_all(command.as_bytes())
            .await
            .map_err(|e| format!("failed to write to server stdin: {}", e))?;

        Ok(())
    }

    pub async fn read_response(&self, server_idx: usize) -> Result<String, String> {
        if server_idx >= self.processes.len() {
            return Err("invalid server index".to_string());
        }

        let proc = Arc::clone(&self.processes[server_idx]);
        let mut proc = proc.lock().await;

        let stdout = proc.stdout.as_mut().ok_or_else(|| "failed to open stdout: {}".to_string())?;

        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        reader
            .read_line(&mut line)
            .await
            .map_err(|e| format!("failed to read from server stdout: {}", e))?;

        Ok(line)
    }

    pub async fn monitor_output(&self, server_idx: usize) -> Result<(), String> {
        if server_idx >= self.processes.len() {
            return Err("invalid server index".to_string());
        }

        let proc_arc = Arc::clone(&self.processes[server_idx]);

        tokio::spawn(async move {
            let mut process = proc_arc.lock().await;

            if let Some(stdout) = process.stdout.take() {
                let mut reader = BufReader::new(stdout);
                let mut line = String::new();

                while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                    println!("Server output: {}", line.trim());
                    line.clear();
                }
            }
        });

        Ok(())
    }

    pub async fn stop_server(&mut self, server_idx: usize) -> Result<(), String> {
        if server_idx >= self.processes.len() {
            return Err("Invalid server index".to_string());
        }

        let process = Arc::clone(&self.processes[server_idx]);
        let mut process = process.lock().await;

        process.kill().await.map_err(|e| format!("Failed to kill server process: {}", e))?;

        self.processes.remove(server_idx);
        self.servers.remove(server_idx);

        Ok(())
    }
}
