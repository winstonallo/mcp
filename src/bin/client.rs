use mcp::{client::Client, server::Server};

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = Client::new(Vec::new());

    let server = Server::new("/home/winstonallo/mpc/target/debug/server");
    client.start_server(server).await?;

    client.monitor_output(0).await?;

    client.send_command(0, "SOME_COMMAND").await?;

    let response = client.read_response(0).await?;
    println!("Got response: {}", response);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    client.stop_server(0).await?;

    Ok(())
}
