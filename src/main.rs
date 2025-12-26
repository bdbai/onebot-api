use serde_json::json;
use onebot_api::communication::ws::WsClient;
use onebot_api::event::EventReceiver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let wsc = WsClient::new_with_token("wss://ws-napcat.acalex.net", Some("Yaom3cWGd2hPvsQC0U2CaJoxozgjsr0t0EQVPA10T4JL8kZGjB".to_string())).await?;
	let rx = wsc.get_receiver();
	while let Ok(event) = rx.recv_async().await {
		println!("{event:#?}")
	}
	Ok(())
}
