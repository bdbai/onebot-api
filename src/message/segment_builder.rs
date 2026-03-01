use super::send_segment::*;
use super::utils::*;

#[derive(Default, Debug, Clone)]
pub struct SegmentBuilder {
	segments: Vec<SendSegment>,
}

impl From<Vec<SendSegment>> for SegmentBuilder {
	fn from(value: Vec<SendSegment>) -> Self {
		Self { segments: value }
	}
}

impl SegmentBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build(self) -> Vec<SendSegment> {
		self.segments
	}

	pub fn text(mut self, text: impl ToString) -> Self {
		self.segments.push(SendSegment::Text {
			data: TextData {
				text: text.to_string(),
			},
		});
		self
	}

	pub fn face(mut self, id: impl ToString) -> Self {
		self.segments.push(SendSegment::Face {
			data: FaceData { id: id.to_string() },
		});
		self
	}

	pub fn image(
		mut self,
		file: impl ToString,
		image_type: Option<ImageType>,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i32>,
	) -> Self {
		self.segments.push(SendSegment::Image {
			data: ImageData {
				file: file.to_string(),
				image_type,
				cache,
				proxy,
				timeout,
			},
		});
		self
	}

	pub fn record(
		mut self,
		file: impl ToString,
		magic: impl ToString,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i32>,
	) -> Self {
		self.segments.push(SendSegment::Record {
			data: RecordData {
				file: file.to_string(),
				magic: magic.to_string(),
				cache,
				proxy,
				timeout,
			},
		});
		self
	}

	pub fn video(
		mut self,
		file: impl ToString,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i32>,
	) -> Self {
		self.segments.push(SendSegment::Video {
			data: VideoData {
				file: file.to_string(),
				cache,
				proxy,
				timeout,
			},
		});
		self
	}

	pub fn at(mut self, qq: AtType) -> Self {
		self.segments.push(SendSegment::At {
			data: AtData { qq },
		});
		self
	}

	pub fn rps(mut self) -> Self {
		self.segments.push(SendSegment::Rps { data: RpsData {} });
		self
	}

	pub fn dice(mut self) -> Self {
		self.segments.push(SendSegment::Dice { data: DiceData {} });
		self
	}

	pub fn shake(mut self) -> Self {
		self
			.segments
			.push(SendSegment::Shake { data: ShakeData {} });
		self
	}

	pub fn poke(mut self, poke_type: impl ToString, id: impl ToString) -> Self {
		self.segments.push(SendSegment::Poke {
			data: PokeData {
				poke_type: poke_type.to_string(),
				id: id.to_string(),
			},
		});
		self
	}

	pub fn anonymous(mut self, ignore: Option<bool>) -> Self {
		self.segments.push(SendSegment::Anonymous {
			data: AnonymousData { ignore },
		});
		self
	}

	pub fn share(
		mut self,
		url: impl ToString,
		title: impl ToString,
		content: impl ToString,
		image: impl ToString,
	) -> Self {
		self.segments.push(SendSegment::Share {
			data: ShareData {
				url: url.to_string(),
				title: title.to_string(),
				content: content.to_string(),
				image: image.to_string(),
			},
		});
		self
	}

	pub fn contact(mut self, contact_type: ContactType, id: impl ToString) -> Self {
		self.segments.push(SendSegment::Contact {
			data: ContactData {
				contact_type,
				id: id.to_string(),
			},
		});
		self
	}

	pub fn contact_qq(self, id: impl ToString) -> Self {
		self.contact(ContactType::QQ, id)
	}

	pub fn contact_group(self, id: impl ToString) -> Self {
		self.contact(ContactType::Group, id)
	}

	pub fn location(
		mut self,
		lat: impl ToString,
		lon: impl ToString,
		title: Option<String>,
		content: Option<String>,
	) -> Self {
		self.segments.push(SendSegment::Location {
			data: LocationData {
				lat: lat.to_string(),
				lon: lon.to_string(),
				title,
				content,
			},
		});
		self
	}

	#[allow(clippy::too_many_arguments)]
	pub fn music(
		mut self,
		music_type: MusicType,
		id: Option<String>,
		url: Option<String>,
		audio: Option<String>,
		title: Option<String>,
		content: Option<String>,
		image: Option<String>,
	) -> Self {
		self.segments.push(SendSegment::Music {
			data: MusicData {
				music_type,
				id,
				url,
				audio,
				title,
				content,
				image,
			},
		});
		self
	}

	pub fn music_typed(self, music_type: MusicType, id: impl ToString) -> Self {
		self.music(
			music_type,
			Some(id.to_string()),
			None,
			None,
			None,
			None,
			None,
		)
	}

	pub fn music_qq(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::QQ, id)
	}

	pub fn music_163(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::NetEaseCloudMusic, id)
	}

	pub fn music_xm(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::Xm, id)
	}

	pub fn music_custom(
		self,
		url: impl ToString,
		audio: impl ToString,
		title: impl ToString,
		content: impl ToString,
		image: impl ToString,
	) -> Self {
		self.music(
			MusicType::Custom,
			None,
			Some(url.to_string()),
			Some(audio.to_string()),
			Some(title.to_string()),
			Some(content.to_string()),
			Some(image.to_string()),
		)
	}

	pub fn reply(mut self, id: impl ToString) -> Self {
		self.segments.push(SendSegment::Reply {
			data: ReplyData { id: id.to_string() },
		});
		self
	}

	pub fn forward(mut self) -> Self {
		self.segments.push(SendSegment::Forward {
			data: ForwardData {},
		});
		self
	}

	pub fn node(
		mut self,
		id: Option<String>,
		user_id: Option<String>,
		nickname: Option<String>,
		content: Option<Vec<SendSegment>>,
	) -> Self {
		self.segments.push(SendSegment::Node {
			data: NodeData {
				id,
				user_id,
				nickname,
				content,
			},
		});
		self
	}

	pub fn node_forward(self, id: impl ToString) -> Self {
		self.node(Some(id.to_string()), None, None, None)
	}

	pub fn node_custom(
		self,
		user_id: impl ToString,
		nickname: impl ToString,
		content: Vec<SendSegment>,
	) -> Self {
		self.node(
			None,
			Some(user_id.to_string()),
			Some(nickname.to_string()),
			Some(content),
		)
	}

	pub fn xml(mut self, data: impl ToString) -> Self {
		self.segments.push(SendSegment::Xml {
			data: XmlData {
				data: data.to_string(),
			},
		});
		self
	}

	pub fn json(mut self, data: impl ToString) -> Self {
		self.segments.push(SendSegment::Json {
			data: JsonData {
				data: data.to_string(),
			},
		});
		self
	}
}

#[macro_export]
macro_rules! text {
	( $( $arg: tt )* ) => {
		{
			$crate::message::segment_builder::SegmentBuilder::new().text( format!( $( $arg )* ) ).build()
		}
	};
}
