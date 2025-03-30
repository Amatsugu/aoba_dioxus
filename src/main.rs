use dioxus::prelude::*;

use components::MainLayout;
use views::*;

mod components;
mod db;
mod models;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
	#[layout(MainLayout)]
	#[route("/")]
	Home {},
	#[route("/settings")]
	Settings {},
}

const MAIN_CSS: Asset = asset!("/assets/style/main.scss");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	// Build cool things ✌️

	rsx! {
		// Global app resources
		document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
		}


		Router::<Route> {}
	}
}
