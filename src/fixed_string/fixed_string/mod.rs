// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::deserialise::{Deserialise, DStream};
use crate::error::Error;
use crate::fixed_string::Iter;
use crate::serialise::{SStream, Serialise};

use std::fmt::{Display, Debug, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct FixedString<const N: usize> {
	pub(in super) buf: [char; N],
	pub(in super) len: usize,
}

impl<const N: usize> FixedString<N> {
	pub fn new(s: &str) -> Result<Self, Error> {
		let mut buf = ['\0'; N];
		let     len = s.chars().count();

		for (i, c) in s.chars().enumerate() {
			if i >= N { return Err(Error::FixedStringTooShort { len: N, s: s.to_owned() }) }

			buf[i] = c;
		}

		Ok(Self { buf, len })
	}

	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize { self.len }

	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool { self.len == 0x0 }

	#[inline]
	pub fn iter(&self) -> std::slice::Iter<'_, char> { self.buf[0x0..self.len].iter() }

	#[inline]
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, char> { self.buf[0x0..self.len].iter_mut() }
}

impl<const N: usize> Debug for FixedString<N> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "\"")?;

		for c in self {
			if c.is_ascii_graphic() {
				write!(f, "{c}")?;
			} else if *c == '\0' {
				write!(f, "\\0")?;
			} else {
				write!(f, "{c}")?;
			}
 		}

		 write!(f, "\"")?;

		Ok(())
	}
}

impl<const N: usize> Deserialise for FixedString<N> {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn std::error::Error>> {
		let len = usize::try_from(u64::deserialise(stream)?).unwrap();

		let data = stream.take(len)?;
		let s = std::str::from_utf8(data)?;

		let len = s.chars().count();
		if len > N {
			return Err(Box::new(Error::FixedStringTooShort { len, s: s.to_owned() }));
		}

		let mut buf = ['\0'; N];
		for (i, c) in s.chars().enumerate() {
			buf[i] = c;
		}

		Ok(Self { buf, len })
	}
}

impl<const N: usize> Default for FixedString<N> {
	#[inline(always)]
	fn default() -> Self { Self {
		buf: ['\0'; N],
		len: 0x0,
	} }
}

impl<const N: usize> Display for FixedString<N> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		for c in self { write!(f, "{c}")? }

		Ok(())
	}
}

impl<const N: usize> Eq for FixedString<N> { }

impl<const N: usize> FromStr for FixedString<N> {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Error> { Self::new(s) }
}

impl<const N: usize> IntoIterator for FixedString<N> {
	type Item = char;

	type IntoIter = Iter<N>;

	fn into_iter(self) -> Self::IntoIter {
		Iter {
			buf: self.buf,
			len: self.len,

			pos: Some(0x0),
		}
	}
}

impl<'a, const N: usize> IntoIterator for &'a FixedString<N> {
	type Item = &'a char;

	type IntoIter = std::slice::Iter<'a, char>;

	fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, const N: usize> IntoIterator for &'a mut FixedString<N> {
	type Item = &'a mut char;

	type IntoIter = std::slice::IterMut<'a, char>;

	fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<const N: usize> PartialEq for FixedString<N> {
	fn eq(&self, other: &Self) -> bool {
		if self.len() != other.len() { return false };

		for i in 0x0..self.len() {
			if self.buf[i] != other.buf[i] { return false };
		}

		true
	}
}

impl<const N: usize> PartialEq<&str> for FixedString<N> {
	fn eq(&self, other: &&str) -> bool {
		for (i, c) in other.chars().enumerate() {
			if self.buf.get(i) != Some(&c) { return false };
		}

		true
	}
}

impl<const N: usize> Serialise for FixedString<N> {
	fn serialise(&self, stream: &mut SStream) {
		let s: String = self.iter().collect();

		let len = u64::try_from(s.len()).unwrap();

		stream.append(&len.to_be_bytes());
		stream.append(&s.into_bytes());
	}
}

impl<const N: usize> TryFrom<&str> for FixedString<N> {
	type Error = Error;

	#[inline(always)]
	fn try_from(value: &str) -> Result<Self, Self::Error> { Self::new(value) }
}
