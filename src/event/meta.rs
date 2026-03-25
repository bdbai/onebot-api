use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use strum::{Display, EnumIs};

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum LifecycleSubType {
	#[serde(rename = "enable")]
	Enable,
	#[serde(rename = "disable")]
	Disable,
	#[serde(rename = "connect")]
	Connect,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetaEventLifecycle {
	pub sub_type: LifecycleSubType,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetaEventHeartbeat {
	pub status: HashMap<String, Value>,
	pub interval: i64,
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs, Eq, PartialEq)]
#[serde(tag = "meta_event_type")]
pub enum MetaEvent {
	#[serde(rename = "lifecycle")]
	Lifecycle(MetaEventLifecycle),

	#[serde(rename = "heartbeat")]
	Heartbeat(MetaEventHeartbeat),
}

impl MetaEvent {
	pub fn on_lifecycle<T>(&self, handler: impl FnOnce(&MetaEventLifecycle) -> T) -> Option<T> {
		if let Self::Lifecycle(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_lifecycle_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MetaEventLifecycle) -> T,
	) -> Option<T> {
		if let Self::Lifecycle(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn on_heartbeat<T>(&self, handler: impl FnOnce(&MetaEventHeartbeat) -> T) -> Option<T> {
		if let Self::Heartbeat(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_heartbeat_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MetaEventHeartbeat) -> T,
	) -> Option<T> {
		if let Self::Heartbeat(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}
