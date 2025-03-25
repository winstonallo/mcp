const NOTIFICATION: &str = "{\"jsonrpc\": \"2.0\",\"method\": \"tools/list\", \"params\": {\"foo\": \"bar\"}}";
const REQUEST: &str = "{\"jsonrpc\": \"2.0\", \"id\": \"69\", \"method\": \"tools/list\", \"params\": {\"foo\": \"bar\"}}";
const RESPONSE: &str = "{\"jsonrpc\": \"2.0\", \"id\": \"69\", \"result\": {\"foo\": \"bar\"}}";
const ERROR: &str = "{\"jsonrpc\": \"2.0\", \"id\": \"69\", \"error\": {\"code\": -32600, \"message\": \"oops\"}}";

#[tokio::main]
async fn main() {
    println!("{}", REQUEST);
    println!("{}", RESPONSE);
    println!("{}", NOTIFICATION);
    println!("{}", ERROR);
}
