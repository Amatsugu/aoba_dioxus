#[cfg(feature = "server")]
use mongodb::{Client, Database};
#[cfg(feature = "server")]
use std::sync::RwLock;

#[cfg(feature = "server")]
static DB: DatabaseConncection = DatabaseConncection {
	client: RwLock::new(None),
	db: RwLock::new(None),
};

#[cfg(feature = "server")]
#[derive(Default)]
pub struct DatabaseConncection {
	client: RwLock<Option<Client>>,
	db: RwLock<Option<Database>>,
}

#[cfg(feature = "server")]
impl DatabaseConncection {
	pub async fn get_db(&self) -> Database {
		self.ensure_client().await;
		return self.db.read().unwrap().clone().unwrap();
	}

	pub async fn get_client(&self) -> Client {
		self.ensure_client().await;
		return self.client.read().unwrap().clone().unwrap();
	}

	async fn ensure_client(&self) {
		if self.client.read().unwrap().is_none() {
			dioxus::logger::tracing::info!("Creating Mongo Client");
			let client = Client::with_uri_str("mongodb://NinoIna:27017").await.unwrap();
			*self.db.write().unwrap() = Some(client.database("Aoba"));
			*self.client.write().unwrap() = Some(client);
		}
	}
}

#[cfg(feature = "server")]
pub async fn get_db() -> Database {
	return DB.get_db().await;
}
#[cfg(feature = "server")]
pub async fn get_db_client() -> Client {
	return DB.get_client().await;
}
