use std::{error::Error, path::Path};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
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

    tokio::spawn({
        let id = id.clone();
        async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        println!("[{}:stdout]: {}", id, line);
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

    servers[0].process.stdin.as_mut().unwrap().write_all("hello\n".as_bytes()).await?;
    servers[0].process.stdin.as_mut().unwrap().flush().await?;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
