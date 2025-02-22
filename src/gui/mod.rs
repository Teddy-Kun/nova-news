mod about;
mod app;
mod localization;
mod main_view;

use crate::err::Error;

#[cfg(feature = "wayland")]
use libcosmic_wayland as cosmic;

use app::NovaNews;
use cosmic::app::Settings;

pub fn run_gui() -> Result<(), Error> {
	tracing_subscriber::fmt::init();
	let _ = tracing_log::LogTracer::init();

	let settings = Settings::default().debug(true);
	if let Err(e) = cosmic::app::run::<NovaNews>(settings, ()) {
		Err(Error::new(&e.to_string()))
	} else {
		Ok(())
	}
}
