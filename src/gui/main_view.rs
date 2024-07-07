#[cfg(feature = "wayland")]
use libcosmic_wayland as cosmic;

use crate::rss;
use cosmic::{widget::Grid, Element};

use super::app::Message;

pub async fn get_feeds(urls: &[&str]) -> Element<'static, Message> {
	let mut grid = Grid::new();
	for u in urls {
		match rss::get_feed(u).await {
			Ok(o) => {}
			Err(e) => {}
		}
	}

	grid.into()
}
