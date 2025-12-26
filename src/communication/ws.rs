use crate::event::{Event, EventReceiver};
use async_trait::async_trait;
use flume::{Receiver, Sender};
use futures::StreamExt;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

// type ListenerList<T> = Arc<RwLock<Vec<T>>>;

pub struct WsClient {
	receiver: Receiver<Event>,
	is_running: Arc<AtomicBool>
}

impl WsClient {
	pub async fn new(url: &str) -> anyhow::Result<Self> {
		let (ws_stream, _) = connect_async(url).await?;
		let (tx, rx) = flume::unbounded::<Event>();
		let is_running = Arc::new(AtomicBool::new(true));
		Self::listen(tx, ws_stream, Arc::clone(&is_running));
		Ok(Self {
			receiver: rx,
			is_running
		})
	}

	pub async fn new_with_token(url: &str, token: Option<String>) -> anyhow::Result<Self> {
		if let Some(token) = token {
			let mut url = Url::from_str(url)?;
			url.set_query(Some(&format!("access_token={}", token)));
			Self::new(url.as_str()).await
		} else {
			Self::new(url).await
		}
	}

	fn listen(tx: Sender<Event>, mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, is_running: Arc<AtomicBool>) {
		tokio::spawn(async move {
			let mut task_fn = async move || {
				while let Some(msg) = ws_stream.next().await {
					let is_running = is_running.load(Ordering::Relaxed);
					if !is_running {
						return
					}
					match msg {
						Ok(Message::Text(data)) => {
							let str = data.as_str();
							if let Ok(event) = serde_json::from_str(str) {
								let _ = tx.send_async(event).await;
							};
						},
						_ => ()
					}
				}
			};

			task_fn().await;
		});
	}
	
	pub async fn restart(&mut self) -> anyhow::Result<()> {
		todo!() // TODO 客户端的手动重启及自动重启
	}
}

// 实现Drop特征防止内存泄露
impl Drop for WsClient {
	fn drop(&mut self) {
		self.is_running.fetch_not(Ordering::Relaxed);
	}
}

#[async_trait]
impl EventReceiver for WsClient {
	fn get_receiver(&self) -> Receiver<Event> {
		self.receiver.clone()
	}
}

// impl APISender for WsClient {}

// #[async_trait]
// impl<T: AsyncFn(Event) -> anyhow::Result<()> + Send + Sync +'static> EventReceiver<T> for WsClient<T> {
// 	async fn wait_event(&self) -> anyhow::Result<EventStream> {
//
// 	}
//
// 	async fn listen(&mut self, listener: T) -> anyhow::Result<()> {
// 		self.listeners.write().await.push(listener);
// 		if !self.is_handing {
// 			let rx = Arc::clone(&self.receiver);
// 			let listeners = Arc::clone(&self.listeners);
// 			tokio::spawn(async move {
// 				let mut rx = rx.lock().await;
// 				while let Some(event) = rx.recv().await {
// 					let listeners = listeners.read().await;
// 					for listener in listeners.iter() {
// 						listener(event.clone()).await;
// 					}
// 				}
// 			});
// 			self.is_handing = true;
// 		}
// 		Ok(())
// 	}
// }

