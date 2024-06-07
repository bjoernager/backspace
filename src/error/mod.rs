// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::checksum::Checksum;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::process::{ExitCode, Termination};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	ArrayLengthMismatch { len: usize, ok_len: usize },

	DeserialiseFailure { source: Box<dyn StdError> },

	EndOfDStream { len: usize, ok_len: usize },

	InvalidBoolean { value: u8 },

	InvalidCodePoint { value: u32 },

	FixedStringTooShort { len: usize, s: String },

	InvalidPackageLength { len: usize, ok_len: usize },

	NullInteger,

	PackageChecksumMismatch { sum: Checksum, ok_sum: Checksum },
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		use Error::*;

		match *self {
			ArrayLengthMismatch { len, ok_len } => {
				write!(f, "expected array of length ({ok_len}) but got ({len}) elements")
			},

			DeserialiseFailure { ref source } => {
				write!(f, "unable to deserialise: \"{source}\"")
			},

			EndOfDStream { len, ok_len } => {
				write!(f, "({ok_len}) byte(s) were requested but only ({len}) byte(s) were left")
			},

			FixedStringTooShort { len, ref s } => {
				write!(f, "fixed string with `N = {len}` cannot hold {s:?}")
			},

			InvalidBoolean { value } => {
				write!(f, "expected boolean but got {value:#02X}")
			},

			InvalidCodePoint { value } => {
				write!(f, "code point U+{value:04X} is not valid")
			}

			InvalidPackageLength { len, ok_len } => {
				write!(f, "invalid packet length: expected at most ({ok_len}) byte(s) but got ({len}) byte(s)")
			},

			NullInteger => {
				write!(f, "expected non-zero integer but got (0)")
			},

			PackageChecksumMismatch { ref sum, ref ok_sum } => {
				write!(f, "expected packet checksum {ok_sum} but got {sum}")
			},
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		use Error::*;

		match *self {
			DeserialiseFailure { ref source } => Some(source.as_ref()),

			_ => None,
		}
	}
}

impl From<Error> for i32 {
	fn from(_: Error) -> Self { 0x1 }
}

impl Termination for Error {
	#[inline(always)]
	fn report(self) -> ExitCode { ExitCode::FAILURE }
}
