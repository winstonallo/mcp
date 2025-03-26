use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, BufReader, stdin},
    sync::mpsc,
};

use crate::core::jsonrpc;

pub struct Server {
    receiver: mpsc::Receiver<jsonrpc::Message>,
}

fn poll(sender: mpsc::Sender<jsonrpc::Message>) -> impl Future<Output = ()> {
    async move {
        let mut reader = BufReader::new(stdin());
        let mut line = String::new();

        loop {
            line.clear();

            let bytes = match reader.read_line(&mut line).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("error reading from stdin: {}", e);
                    continue;
                }
            };
            if bytes == 0 {
                continue;
            }

            let raw = match serde_json::from_str::<jsonrpc::Raw>(&line.trim_end()) {
                Ok(raw) => raw,
                Err(e) => {
                    eprintln!("could not deserialize `{}`: {}", line, e);
                    continue;
                }
            };

            let msg = match jsonrpc::Message::try_from(raw) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("`{}` is not valid JSON-RPC: {}", line, e);
                    continue;
                }
            };

            if let Err(e) = sender.send(msg).await {
                eprintln!("could not send message: {}", e);
            }
        }
    }
}

impl Server {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (sender, receiver) = mpsc::channel(100);

        tokio::spawn(poll(sender));

        Ok(Self { receiver })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(msg) = self.receiver.recv().await {
            eprintln!("got message: {:?}", msg);
        }

        Ok(())
    }
}
