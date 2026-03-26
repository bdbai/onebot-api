use crate::api::APISender;
use crate::error::{APIRequestError, APIResult};
use crate::message::receive_segment::ReceiveSegment;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs};

#[cfg(feature = "quick_operation")]
use crate::quick_operation::QuickSendMsg;

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum Sex {
	#[serde(rename = "male")]
	Male,
	#[serde(rename = "female")]
	Female,
	#[serde(rename = "unknown")]
	Unknown,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrivateMessageSender {
	pub user_id: Option<i64>,
	pub nickname: Option<String>,
	pub sex: Option<Sex>,
	pub age: Option<i32>,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for PrivateMessageSender {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api
			.send_private_msg(
				self.user_id.ok_or(APIRequestError::MissingParameters)?,
				msg,
				auto_escape,
			)
			.await
	}
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum PrivateMessageSubType {
	#[serde(rename = "friend")]
	Friend,
	#[serde(rename = "group")]
	Group,
	#[serde(rename = "other")]
	Other,
}

#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupMessageAnonymous {
	id: i64,
	name: String,
	flag: String,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupMessageSender {
	pub user_id: Option<i64>,
	pub nickname: Option<String>,
	pub card: Option<String>,
	pub sex: Option<Sex>,
	pub age: Option<i32>,
	pub area: Option<String>,
	pub level: Option<String>,
	pub role: Option<GroupMessageSenderRole>,
	pub title: Option<String>,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for GroupMessageSender {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api
			.send_private_msg(
				self.user_id.ok_or(APIRequestError::MissingParameters)?,
				msg,
				auto_escape,
			)
			.await
	}
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupMessageSenderRole {
	#[serde(rename = "owner")]
	Owner,
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "member")]
	Member,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupMessageSubType {
	#[serde(rename = "normal")]
	Normal,
	#[serde(rename = "anonymous")]
	Anonymous,
	#[serde(rename = "notice")]
	Notice,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventPrivate {
	pub sub_type: PrivateMessageSubType,
	pub message_id: i32,
	pub user_id: i64,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i32,
	pub sender: PrivateMessageSender,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventPrivate {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api.send_private_msg(self.user_id, msg, auto_escape).await
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventGroup {
	pub sub_type: GroupMessageSubType,
	pub message_id: i32,
	pub group_id: i64,
	pub user_id: i64,
	pub anonymous: Option<GroupMessageAnonymous>,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i32,
	pub sender: GroupMessageSender,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventGroup {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api.send_group_msg(self.group_id, msg, auto_escape).await
	}
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
	#[serde(rename = "private")]
	Private(MessageEventPrivate),

	#[serde(rename = "group")]
	Group(MessageEventGroup),
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEvent {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		match self {
			Self::Group(data) => data.send_msg(api, msg, auto_escape),
			Self::Private(data) => data.send_msg(api, msg, auto_escape),
		}
		.await
	}
}

impl MessageEvent {
	pub fn on_private<T>(&self, handler: impl FnOnce(&MessageEventPrivate) -> T) -> Option<T> {
		if let Self::Private(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_private_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MessageEventPrivate) -> T,
	) -> Option<T> {
		if let Self::Private(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_group<T>(&self, handler: impl FnOnce(&MessageEventGroup) -> T) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MessageEventGroup) -> T,
	) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}
