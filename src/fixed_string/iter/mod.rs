// Copyright 2022-2024 Gabriel Bjørnager Jensen.

pub struct Iter<const N: usize> {
	pub(in super) buf: [char; N],
	pub(in super) len: usize,

	pub(in super) pos: Option<usize>,
}

impl<const N: usize> Iterator for Iter<N> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let pos = self.pos.as_mut()?;

		if *pos >= self.len { return None };

		let item = self.buf[*pos];
		*pos += 0x1;

		Some(item)
	}
}
