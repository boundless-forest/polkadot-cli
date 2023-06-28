use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT},
	rpc_params,
};

// let addr = "127.0.0.1:9944";
// let uri: Uri = format!("ws://{}", addr).parse()?;
// let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
// let client = ClientBuilder::default().build_with_tokio(tx, rx);
// let response: String = client.request("system_version", rpc_params![]).await?;
// println!("bear: the result is {:?}", response);
