use crate::api::APISender;
use crate::error::APIResult;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;

#[async_trait]
pub trait QuickSendMsg<T: APISender + Sync + Send> {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;
}
