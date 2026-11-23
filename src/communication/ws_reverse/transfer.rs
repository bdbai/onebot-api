use std::pin::Pin;
use std::task::{Context, Poll, ready};

use axum::extract::ws::{Message, WebSocket};
use flume::r#async::{RecvStream, SendFut};
use futures::{Sink, Stream};

use crate::communication::utils::{
	APIRequest, DeserializedEvent, InternalAPIReceiver, InternalEventSender,
};
use crate::error::{ServiceRuntimeError, ServiceRuntimeResult};

pub(super) struct ReverseWsTransfer<'a, 'b> {
	ws: WebSocket,
	api_stream: RecvStream<'a, APIRequest>,
	event_sender: &'b InternalEventSender,
	event_send_task: Option<SendFut<'b, DeserializedEvent>>,
	upload_state: UploadState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UploadState {
	AwaitingEvent,
	Flushing,
	ClosingByLocal,
	ClosedByLocal,
	ClosedByPeer,
}

impl<'a, 'b> ReverseWsTransfer<'a, 'b> {
	pub fn new(
		ws: WebSocket,
		api_receiver: &'a InternalAPIReceiver,
		event_sender: &'b InternalEventSender,
	) -> Self {
		Self {
			ws,
			api_stream: api_receiver.stream(),
			event_sender,
			event_send_task: None,
			upload_state: UploadState::AwaitingEvent,
		}
	}

	fn initiate_close(&mut self) {
		if self.upload_state != UploadState::ClosedByLocal {
			self.upload_state = UploadState::ClosingByLocal;
		}
	}

	fn poll_upload_one_event(&mut self, cx: &mut Context<'_>) -> Poll<ServiceRuntimeResult<()>> {
		let mut ws = Pin::new(&mut self.ws);
		ready!(ws.as_mut().poll_ready(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
		let api_stream = Pin::new(&mut self.api_stream);
		match ready!(api_stream.poll_next(cx)) {
			Some(event) => {
				let Ok(msg) = serde_json::to_string(&event) else {
					return Poll::Ready(Ok(()));
				};
				ws.as_mut()
					.start_send(Message::Text(msg.into()))
					.map_err(|_| ServiceRuntimeError::WebSocketError)?;
				self.upload_state = UploadState::Flushing;
				Poll::Ready(Ok(()))
			}
			None => {
				self.initiate_close();
				Poll::Ready(Ok(()))
			}
		}
	}

	/// 推进挂起的事件下发，返回 `true` 表示通道仍满、事件尚未送出。  
	/// 这里不能改回阻塞的 `event_sender.send()`：单线程 runtime 下通道满时
	/// 会把 worker 线程卡死，其他任务（包括事件消费者）都无法推进。
	fn poll_event_send(&mut self, cx: &mut Context<'_>) -> bool {
		let Some(task) = self.event_send_task.as_mut() else {
			return false;
		};
		match Pin::new(task).poll(cx) {
			// 事件消费者已消失时为 Err，事件直接丢弃
			Poll::Ready(_result) => {
				self.event_send_task = None;
				false
			}
			Poll::Pending => true,
		}
	}

	fn poll_progress(&mut self, cx: &mut Context<'_>) -> Poll<ServiceRuntimeResult<()>> {
		loop {
			// 事件下发不阻塞其余 IO，通道满时只暂停读取新帧
			let event_pending = self.poll_event_send(cx);

			let ws = Pin::new(&mut self.ws);
			match self.upload_state {
				UploadState::AwaitingEvent => {
					if self.poll_upload_one_event(cx)?.is_ready() {
						continue;
					}
				}
				UploadState::Flushing => {
					ready!(ws.poll_flush(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					self.upload_state = UploadState::AwaitingEvent;
					continue;
				}
				UploadState::ClosingByLocal => {
					ready!(ws.poll_close(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					self.upload_state = UploadState::ClosedByLocal;
					continue;
				}
				UploadState::ClosedByLocal => {
					// 连接已收尾，不再等待事件通道，在途事件随 Drop 丢弃
					return Poll::Ready(Ok(()));
				}
				UploadState::ClosedByPeer => {
					ready!(ws.poll_close(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					return Poll::Ready(Err(ServiceRuntimeError::WebSocketClosedByPeer));
				}
			}

			// 事件通道满时暂停读取新帧：保证事件顺序，且最多只多缓存一条事件。
			// 此处 poll_event_send 已注册 waker，可以安全返回 Pending。
			if event_pending {
				return Poll::Pending;
			}

			let ws = Pin::new(&mut self.ws);
			match ready!(ws.poll_next(cx)) {
				Some(Ok(Message::Text(msg))) => {
					let Ok(event) = serde_json::from_str::<DeserializedEvent>(msg.as_str()) else {
						continue;
					};
					self.event_send_task = Some(self.event_sender.send_async(event));
				}
				Some(Ok(Message::Close(_))) | None => self.upload_state = UploadState::ClosedByPeer,
				Some(Err(_)) => return Poll::Ready(Err(ServiceRuntimeError::WebSocketError)),
				Some(Ok(_)) => (),
			}
		}
	}
}

impl<'a, 'b> Future for ReverseWsTransfer<'a, 'b> {
	type Output = ServiceRuntimeResult<()>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.poll_progress(cx)
	}
}
