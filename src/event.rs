use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
use async_trait::async_trait;
// use flume::Receiver;
use message::MessageEvent;
use serde::Deserialize;
use strum::{Display, EnumIs};
use tokio::sync::broadcast;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Deserialize, Debug, Clone)]
pub struct EventMessage {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: Box<MessageEvent>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventNotice {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: NoticeEvent,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventRequest {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: RequestEvent,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventMetaEvent {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: MetaEvent,
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs)]
#[serde(tag = "post_type")]
pub enum Event {
	#[serde(rename = "message")]
	Message(EventMessage),

	#[serde(rename = "notice")]
	Notice(EventNotice),

	#[serde(rename = "request")]
	Request(EventRequest),

	#[serde(rename = "meta_event")]
	MetaEvent(EventMetaEvent),
}

impl Event {
	pub fn on_message<T>(&self, handler: impl FnOnce(&EventMessage) -> T) -> Option<T> {
		if let Self::Message(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_message_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventMessage) -> T,
	) -> Option<T> {
		if let Self::Message(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_notice<T>(&self, handler: impl FnOnce(&EventNotice) -> T) -> Option<T> {
		if let Self::Notice(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_notice_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventNotice) -> T,
	) -> Option<T> {
		if let Self::Notice(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_request<T>(&self, handler: impl FnOnce(&EventRequest) -> T) -> Option<T> {
		if let Self::Request(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_request_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventRequest) -> T,
	) -> Option<T> {
		if let Self::Request(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_meta_event<T>(&self, handler: impl FnOnce(&EventMetaEvent) -> T) -> Option<T> {
		if let Self::MetaEvent(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_meta_event_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventMetaEvent) -> T,
	) -> Option<T> {
		if let Self::MetaEvent(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

pub trait EventTrait {}

impl EventTrait for Event {}

#[async_trait]
pub trait EventReceiver<T: EventTrait> {
	fn subscribe(&self) -> broadcast::Receiver<T>;
}
