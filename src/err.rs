use std::{fmt::Debug, process};

#[cfg(feature = "backtrace")]
use backtrace::Backtrace;

// Universal Error struct.
// Anything that can be converted to string can be converted to this Error.
// If the 'Backtrace' feature is enabled it will also automatically generate and print a Trace when converted to a string.
pub struct Error {
	message: String,
	#[cfg(feature = "backtrace")]
	trace: Backtrace,
}

impl Error {
	pub fn new(message: &str) -> Error {
		#[cfg(feature = "backtrace")]
		return Error {
			message: message.to_string(),
			trace: Backtrace::new(),
		};

		#[cfg(not(feature = "backtrace"))]
		return Error {
			message: message.to_string(),
		};
	}

	pub fn out(&self) {
		eprintln!("{:?}", self);
	}

	pub fn fatal(&self) {
		self.out();
		process::exit(1);
	}
}

impl<T: ToString> From<T> for Error {
	#[cfg(feature = "backtrace")]
	fn from(value: T) -> Self {
		Error {
			message: value.to_string(),
			trace: Backtrace::new(),
		}
	}

	#[cfg(not(feature = "backtrace"))]
	fn from(value: T) -> Self {
		Error {
			message: value.to_string(),
		}
	}
}

impl Debug for Error {
	#[cfg(feature = "backtrace")]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\nStacktrace:\n{:?}", self.message, self.trace)
	}

	#[cfg(not(feature = "backtrace"))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
