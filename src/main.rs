mod err;
mod gui;
mod rss;

use err::Error;

fn main() -> Result<(), Error> {
	#[cfg(all(not(feature = "wayland"), not(feature = "winit")))]
	compile_error!("Either `winit` OR `wayland` must be enabled!");

	#[cfg(all(feature = "wayland", feature = "winit"))]
	compile_error!("Either `winit` OR `wayland` must be enabled!");

	gui::run_gui()?;
	Ok(())
}
