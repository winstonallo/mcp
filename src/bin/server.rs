const NOTIFICATION: &str = "{\"jsonrpc\": \"2.0\",\"method\": \"tools/list\", \"params\": {\"foo\": \"bar\"}}";

#[tokio::main]
async fn main() {
    println!("{}", NOTIFICATION);
}
