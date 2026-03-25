use serde::Deserialize;
use strum::{Display, EnumIs};

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupType {
	#[serde(rename = "add")]
	Add,
	#[serde(rename = "invite")]
	Invite,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventFriend {
	user_id: i64,
	comment: String,
	flag: String,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventGroup {
	sub_type: GroupType,
	group_id: i64,
	user_id: i64,
	comment: String,
	flag: String,
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
#[serde(tag = "request_type")]
pub enum RequestEvent {
	#[serde(rename = "friend")]
	Friend(RequestEventFriend),

	#[serde(rename = "group")]
	Group(RequestEventGroup),
}

impl RequestEvent {
	pub fn on_friend<T>(&self, handler: impl FnOnce(&RequestEventFriend) -> T) -> Option<T> {
		if let Self::Friend(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_friend_async<T>(
		&self,
		handler: impl AsyncFnOnce(&RequestEventFriend) -> T,
	) -> Option<T> {
		if let Self::Friend(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_group<T>(&self, handler: impl FnOnce(&RequestEventGroup) -> T) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_async<T>(
		&self,
		handler: impl AsyncFnOnce(&RequestEventGroup) -> T,
	) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}
