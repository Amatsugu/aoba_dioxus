#[cfg(feature = "server")]
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Media {
	#[serde(rename = "_id")]
	pub id: ObjectId,
	#[serde(alias = "MediaId")]
	pub media_id: ObjectId,
	#[serde(alias = "Filename")]
	pub filename: String,
	#[serde(alias = "MediaType")]
	pub media_type: MediaType,
	#[serde(alias = "Ext")]
	pub ext: String,
	#[serde(alias = "ViewCount")]
	pub view_count: i32,
	#[serde(alias = "Owner")]
	pub owner: ObjectId,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MediaViewModel {
	pub id: String,
	pub media_id: String,
	pub filename: String,
	pub media_type: MediaType,
	pub ext: String,
	pub view_count: i32,
	pub owner: String,
}

#[cfg(feature = "server")]
impl Into<MediaViewModel> for Media {
	fn into(self) -> MediaViewModel {
		MediaViewModel {
			id: self.id.to_string(),
			media_id: self.id.to_string(),
			filename: self.filename.clone(),
			media_type: self.media_type.clone(),
			ext: self.ext.clone(),
			view_count: self.view_count,
			owner: self.owner.to_string(),
		}
	}
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(i32)]
pub enum MediaType {
	Image,
	Audio,
	Video,
	Text,
	Code,
	Raw,
}
