use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ImageType {
	#[serde(rename = "flash")]
	Flash,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AtType {
	#[serde(rename = "all")]
	All,
	Id(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ContactType {
	#[serde(rename = "qq")]
	QQ,
	#[serde(rename = "group")]
	Group,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MusicType {
	#[serde(rename = "qq")]
	QQ,
	#[serde(rename = "163")]
	NetEaseCloudMusic,
	#[serde(rename = "xm")]
	Xm,
	#[serde(rename = "custom")]
	Custom,
}
