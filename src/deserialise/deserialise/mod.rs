// Copyright 2022-2024 Gabriel Bjørnager Jensen.

use crate::deserialise::DStream;
use crate::error::Error;

use std::convert::Infallible;
use std::error::Error as StdError;
use std::mem::size_of;
use std::num::NonZero;

pub trait Deserialise: Sized {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>>;
}

macro_rules! impl_float {
	($type:ty) => {
		impl Deserialise for $type {
			fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
				let data = stream
					.take(size_of::<Self>())?
					.try_into()
					.unwrap();

				Ok(Self::from_be_bytes(data))
			}
		}
	};
}

macro_rules! impl_int {
	($type:ty) => {
		impl Deserialise for $type {
			fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
				let data = stream
					.take(size_of::<Self>())?
					.try_into()
					.unwrap();

				Ok(Self::from_be_bytes(data))
			}
		}

		impl Deserialise for NonZero<$type> {
			fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
				Ok(Option::<Self>::deserialise(stream)?
					.ok_or(Error::NullInteger)?)
			}
		}
	};
}

impl<T: Deserialise, const N: usize> Deserialise for [T; N] {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
		let len = usize::try_from(u64::deserialise(stream)?).unwrap();
		if len != N { return Err(Box::new(Error::ArrayLengthMismatch { len, ok_len: N })) };

		let mut buf = Vec::with_capacity(len);
		for _ in 0x0..len { buf.push(Deserialise::deserialise(stream)?); }

		// If we had used the checked unwrap, we would also
		// have to require `T: Debug`.
		Ok(unsafe { buf.try_into().unwrap_unchecked() })
	}
}

impl Deserialise for () {
	fn deserialise(_stream: &mut DStream) -> Result<Self, Box<dyn StdError>> { Ok(()) }
}

impl Deserialise for bool {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
		let value = u8::deserialise(stream)?;

		match value {
			0x00 => Ok(false),
			0x01 => Ok(true),
			_    => Err(Box::new(Error::InvalidBoolean { value }))
		}
	}
}

impl Deserialise for char {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
		let value = u32::deserialise(stream)?;

		Self::from_u32(value)
			.ok_or(Box::new(Error::InvalidCodePoint { value }))
	}
}

impl Deserialise for Infallible {
	fn deserialise(_stream: &mut DStream) -> Result<Self, Box<dyn StdError>> { unreachable!() }
}

impl<T: Deserialise> Deserialise for Option<T> {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
		let sign = bool::deserialise(stream)?;

		if sign {
			Ok(Some(T::deserialise(stream)?))
		} else {
			Ok(None)
		}
	}
}

impl<T: Deserialise, E: Deserialise> Deserialise for Result<T, E> {
	fn deserialise(stream: &mut DStream) -> Result<Self, Box<dyn StdError>> {
		let sign = bool::deserialise(stream)?;

		let value = if sign {
			Err(E::deserialise(stream)?)
		} else {
			Ok(T::deserialise(stream)?)
		};

		Ok(value)
	}
}

impl_float!(f32);
impl_float!(f64);

impl_int!(i128);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i8);
impl_int!(u128);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u8);
