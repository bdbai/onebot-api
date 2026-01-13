use super::send_segment::*;

#[derive(Default, Debug, Clone)]
pub struct SegmentBuilder {
	segments: Vec<SendSegment>,
}

impl SegmentBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build(self) -> Vec<SendSegment> {
		self.segments
	}

	pub fn text(mut self, text: String) -> Self {
		self.segments.push(SendSegment::Text {
			data: TextData { text },
		});
		self
	}

	pub fn face(mut self, id: String) -> Self {
		self.segments.push(SendSegment::Face {
			data: FaceData { id },
		});
		self
	}

	pub fn image(
		mut self,
		file: String,
		image_type: Option<String>,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<bool>,
	) -> Self {
		self.segments.push(SendSegment::Image {
			data: ImageData {
				file,
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
		file: String,
		magic: String,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<bool>,
	) -> Self {
		self.segments.push(SendSegment::Record {
			data: RecordData {
				file,
				magic,
				cache,
				proxy,
				timeout,
			},
		});
		self
	}

	pub fn video(
		mut self,
		file: String,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<bool>,
	) -> Self {
		self.segments.push(SendSegment::Video {
			data: VideoData {
				file,
				cache,
				proxy,
				timeout,
			},
		});
		self
	}

	pub fn at(mut self, qq: String) -> Self {
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

	pub fn poke(mut self, poke_type: String, id: String) -> Self {
		self.segments.push(SendSegment::Poke {
			data: PokeData { poke_type, id },
		});
		self
	}

	pub fn anonymous(mut self, ignore: Option<bool>) -> Self {
		self.segments.push(SendSegment::Anonymous {
			data: AnonymousData { ignore },
		});
		self
	}

	pub fn share(mut self, url: String, title: String, content: String, image: String) -> Self {
		self.segments.push(SendSegment::Share {
			data: ShareData {
				url,
				title,
				content,
				image,
			},
		});
		self
	}

	pub fn contact(mut self, contact_type: String, id: String) -> Self {
		self.segments.push(SendSegment::Contact {
			data: ContactData { contact_type, id },
		});
		self
	}

	pub fn contact_qq(self, id: String) -> Self {
		self.contact("qq".to_string(), id)
	}

	pub fn contact_group(self, id: String) -> Self {
		self.contact("group".to_string(), id)
	}

	pub fn location(
		mut self,
		lat: String,
		lon: String,
		title: Option<String>,
		content: Option<String>,
	) -> Self {
		self.segments.push(SendSegment::Location {
			data: LocationData {
				lat,
				lon,
				title,
				content,
			},
		});
		self
	}

	#[allow(clippy::too_many_arguments)]
	pub fn music(
		mut self,
		music_type: String,
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

	pub fn music_typed(self, music_type: String, id: String) -> Self {
		self.music(music_type, Some(id), None, None, None, None, None)
	}

	pub fn music_qq(self, id: String) -> Self {
		self.music_typed("qq".to_string(), id)
	}

	pub fn music_163(self, id: String) -> Self {
		self.music_typed("163".to_string(), id)
	}

	pub fn music_xm(self, id: String) -> Self {
		self.music_typed("xm".to_string(), id)
	}

	pub fn music_custom(
		self,
		url: String,
		audio: String,
		title: String,
		content: String,
		image: String,
	) -> Self {
		self.music(
			"custom".to_string(),
			None,
			Some(url),
			Some(audio),
			Some(title),
			Some(content),
			Some(image),
		)
	}

	pub fn reply(mut self, id: String) -> Self {
		self.segments.push(SendSegment::Reply {
			data: ReplyData { id },
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

	pub fn node_forward(self, id: String) -> Self {
		self.node(Some(id), None, None, None)
	}

	pub fn node_custom(self, user_id: String, nickname: String, content: Vec<SendSegment>) -> Self {
		self.node(None, Some(user_id), Some(nickname), Some(content))
	}

	pub fn xml(mut self, data: String) -> Self {
		self.segments.push(SendSegment::Xml {
			data: XmlData { data },
		});
		self
	}

	pub fn json(mut self, data: String) -> Self {
		self.segments.push(SendSegment::Json {
			data: JsonData { data },
		});
		self
	}
}
