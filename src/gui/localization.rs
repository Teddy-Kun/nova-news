use i18n_embed::{
	fluent::{fluent_language_loader, FluentLanguageLoader},
	LanguageLoader,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/gui/embed"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
	let loader: FluentLanguageLoader = fluent_language_loader!();

	loader
		.load_fallback_language(&Localizations)
		.expect("Error while loading fallback language");

	loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::gui::localization::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::gui::localization::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}
