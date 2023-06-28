mod cli;
mod rpc;

// 1. Setup node: ./target/release/node-template --dev --tmp --rpc-methods=Unsafe --rpc-cors all --rpc-external --ws-external
// 2: ws://192.168.31.52:9944

use jsonrpsee::client_transport::ws::{Uri, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::rpc_params;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("hello world");
    // TODO: Query the block number of the Polkadot
    let addr = "127.0.0.1:9944";
    // let addr = "crab-rpc.darwinia.network:9944";
    let uri: Uri = format!("ws://{}", addr).parse()?;
    let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
    let client = ClientBuilder::default().build_with_tokio(tx, rx);
    let response: String = client.request("system_version", rpc_params![]).await?;
    println!("bear: the result is {:?}", response);
    Ok(())
}
