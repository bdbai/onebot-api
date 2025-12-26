use async_trait::async_trait;
use flume::Receiver;
use serde::Deserialize;

use message::MessageEvent;

use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
mod message;
mod meta;
mod notice;
mod request;


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "post_type")]
pub enum Event {
	#[serde(rename = "message")]
	Message {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: MessageEvent
	},

	#[serde(rename = "notice")]
	Notice {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: NoticeEvent
	},

	#[serde(rename = "request")]
	Request {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: RequestEvent
	},

	#[serde(rename = "meta_event")]
	MetaEvent {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: MetaEvent
	}
}

// pub struct EventStream {}
//
// impl Stream for EventStream {
// 	type Item = ();
//
// 	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
// 		todo!()
// 	}
// }

#[async_trait]
pub trait EventReceiver {
	// async fn wait_event(&self) -> anyhow::Result<EventStream>; TODO
	// async fn listen(&mut self, listener: T) -> anyhow::Result<()>; TODO

	fn get_receiver(&self) -> Receiver<Event>;
}

