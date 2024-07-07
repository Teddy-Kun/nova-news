use rss::Channel;

use crate::err::Error;

pub async fn get_feed(url: &str) -> Result<Channel, Error> {
	let content = reqwest::get(url).await?.bytes().await?;
	let channel = Channel::read_from(&content[..])?;
	Ok(channel)
}
