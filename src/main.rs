mod err;
mod gui;
mod rss;

use err::Error;

fn main() -> Result<(), Error> {
	gui::run_gui()?;
	Ok(())
}
