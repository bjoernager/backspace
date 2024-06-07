// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::checksum::Checksum;
use crate::deserialise::{Deserialise, DStream};
use crate::error::Error;
use crate::serialise::{SStream, Serialise};

use std::mem::size_of;

#[derive(Clone, Debug)]
pub struct Packet<T: Deserialise + Serialise> {
	message:  T,
	checksum: Checksum,
}

impl<T: Deserialise + Serialise> Packet<T> {
	#[must_use]
	pub fn encode(message: T) -> Box<[u8]> {
		let checksum = Checksum::new(&message);
		let packet = Self { message, checksum };

		SStream::from(&packet).into()
	}

	pub fn decode(data: &[u8]) -> Result<T, Error> {
		if data.len() > size_of::<Self>() {
			return Err(Error::InvalidPackageLength { len: data.len(), ok_len: size_of::<Self>() });
		}

		let mut data = DStream::from(data);

		let packet = Self::deserialise(&mut data)
			.map_err(|e| Error::DeserialiseFailure { source: e })?;

		let checksum = Checksum::new(&packet.message);

		if packet.checksum != checksum {
			return Err(Error::PackageChecksumMismatch { sum: packet.checksum, ok_sum: checksum });
		}

		Ok(packet.message)
	}
}

impl<T: Deserialise + Serialise> Deserialise for Packet<T> {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn std::error::Error>> {
		Ok(Self {
			message:  Deserialise::deserialise(stream)?,
			checksum: Deserialise::deserialise(stream)?,
		})
	}
}

impl<T: Deserialise + Serialise> Serialise for Packet<T> {
	fn serialise(&self, stream: &mut SStream) {
		self.message.serialise(stream);
		self.checksum.serialise(stream);
	}
}
