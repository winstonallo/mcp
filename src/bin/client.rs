use std::{error::Error, path::Path};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, Command},
};

use mcp::core::{
    jsonrpc::{self, Message, NumberOrString, Request},
    methods::InitializeRequest,
};

struct ServerProcess {
    id: String,
    process: Child,
}

async fn spawn_server(id: String, path: &Path) -> Result<ServerProcess, Box<dyn Error>> {
    let mut process = Command::new(path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take().expect("could not take stdout");
    let stdin = process.stdin.take().expect("could not take stdin");

    let request = InitializeRequest::new("2024-11-05", "arthur", "1.0", None);

    tokio::spawn({
        let id = id.clone();
        async move {
            let mut reader = BufReader::new(stdout);
            let mut writer = BufWriter::new(stdin);
            let mut line = String::new();

            let req_str = format!("{}\n", serde_json::to_string(&request).expect("could not serialize"));
            println!("{}", req_str);
            let _ = writer.write_all(req_str.as_bytes()).await;
            let _ = writer.flush().await;

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        println!("[{}:stdout]: {:?}", id, line);

                        let raw: jsonrpc::Raw = serde_json::from_str(&line).expect("could not deserialize");
                        let notification = Message::try_from(raw).expect("invalid jsonrpc");
                        println!("[{}:stdout]: {:?}", id, notification);
                    }
                    Err(e) => {
                        eprintln!("[{}:error]: {}", id, e);
                        break;
                    }
                }
            }
        }
    });

    Ok(ServerProcess { id, process })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_path = vec![Path::new("/home/winstonallo/mcp/target/debug/server")];

    let mut servers = Vec::new();
    for (idx, path) in server_path.iter().enumerate() {
        let server = spawn_server(format!("{}", idx + 1), &path).await?;
        servers.push(server);
    }

    tokio::signal::ctrl_c().await?;

    Ok(())
}
