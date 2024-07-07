use cosmic::{widget::Grid, Element};

use crate::rss;

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
