use std::error::Error;

use mcp::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new().await?;
    if let Err(e) = server.run().await {
        eprintln!("error running server: {}", e)
    }

    Ok(())
}
