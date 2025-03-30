#[cfg(feature = "server")]
use crate::models::media::Media;
use crate::models::media::MediaViewModel;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use mongodb::bson::doc;

#[component]
pub fn Home() -> Element {
	let images = use_server_future(get_images)?.value().cloned();
	rsx! {
		"This is home"
		for img in images.unwrap().unwrap() {
			MediaDisplay { media: img }
		}
	}
}
#[component]
fn MediaDisplay(media: MediaViewModel) -> Element {
	rsx! {
		div {}
	}
}

#[server]
async fn get_images() -> Result<Vec<MediaViewModel>, ServerFnError> {
	let db = crate::db::get_db().await;
	let colection = db.collection::<Media>("media");
	let mut cursor = colection.find(doc! {}).limit(100).await?;

	let mut results = Vec::with_capacity(100);
	while cursor.advance().await? {
		let item = cursor.deserialize_current().expect("Failed to deserailize");
		results.push(item.into());
	}

	Ok(results)
}
