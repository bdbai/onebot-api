use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum MessageType {
  #[serde(rename = "private")]
  Private,
  #[serde(rename = "group")]
  Group
}

