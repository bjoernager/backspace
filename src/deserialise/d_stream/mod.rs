// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::error::{Error, Result};

use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct DStream<'a> {
	data: &'a [u8],
	len:  usize,
}

impl DStream<'_> {
	pub fn take(&mut self, len: usize) -> Result<&[u8]> {
		if self.len < len { return Err(Error::EndOfDStream { len: self.len, ok_len: len } ) }

		let start = self.data.len() - self.len;
		let stop  = start + len;

		self.len -= len;

		Ok(&self.data[start..stop])
	}
}

impl Debug for DStream<'_> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let stop  = self.data.len();
		let start = self.data.len() - self.len;

		write!(f, "[")?;

		for v in &self.data[start..stop] { write!(f, "{v:#02X},")? };

		write!(f, "]")?;

		Ok(())
	}
}

impl<'a> From<&'a [u8]> for DStream<'a> {
	fn from(value: &'a [u8]) -> Self { Self {
		data: value,
		len:  value.len(),
	} }
}

impl<'a, const N: usize> From<&'a [u8; N]> for DStream<'a> {
	fn from(value: &'a [u8; N]) -> Self { Self {
		data: value,
		len:  N,
	} }
}
