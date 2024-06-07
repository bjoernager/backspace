// Copyright 2022-2024 Gabriel Bjørnager Jensen.

pub mod checksum;
pub mod deserialise;
pub mod error;
pub mod fixed_string;
pub mod packet;
pub mod serialise;

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(in crate) use use_mod;
