use async_trait::async_trait;

use crate::{api::arg_type::MessageType};
use crate::message::Segment;

mod arg_type;
mod return_type;

pub(crate) trait APIArg {}

#[async_trait]
pub trait APISender {
  async fn send_private_msg(user_id: i64, message: Segment, auto_escape: Option<bool>) -> anyhow::Result<i32>;
  async fn send_group_msg(group_id: i64, message: Segment, auto_escape: Option<bool>) -> anyhow::Result<i32>;
  async fn send_msg(message_type: Option<MessageType>, user_id: i64, group_id: i64, message: Segment, auto_escape: Option<bool>) -> anyhow::Result<i32>;
  async fn delete_msg(message_id: i32) -> anyhow::Result<()>;
  
}



