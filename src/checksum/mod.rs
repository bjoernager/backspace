// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::deserialise::{Deserialise, DStream};
use crate::serialise::{SStream, Serialise};

use sha2::{Digest, Sha256};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Checksum(pub(in super) [u8; 0x20]);

impl Checksum {
	#[must_use]
	pub fn new<T: Serialise>(data: &T) -> Self {
		let data = SStream::from(data);

		let mut hasher = Sha256::new();
		hasher.update(data);

		Self(hasher.finalize().into())
	}
}

impl Debug for Checksum {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { Display::fmt(self, f) }
}

impl Deserialise for Checksum {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn std::error::Error>> {
		let data = Deserialise::deserialise(stream)?;

		Ok(Self(data))
	}
}

impl Display for Checksum {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f,
			"{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}",
			self.0[0x1F], self.0[0x1E], self.0[0x1D], self.0[0x1C],
			self.0[0x1B], self.0[0x1A], self.0[0x19], self.0[0x18],
			self.0[0x17], self.0[0x16], self.0[0x15], self.0[0x14],
			self.0[0x13], self.0[0x12], self.0[0x11], self.0[0x10],
			self.0[0x0F], self.0[0x0E], self.0[0x0D], self.0[0x0C],
			self.0[0x0B], self.0[0x0A], self.0[0x09], self.0[0x08],
			self.0[0x07], self.0[0x06], self.0[0x05], self.0[0x04],
			self.0[0x03], self.0[0x02], self.0[0x01], self.0[0x00],
		)
	}
}

impl Serialise for Checksum {
	fn serialise(&self, stream: &mut SStream) {
		self.0.serialise(stream);
	}
}
