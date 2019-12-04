use std::convert::TryFrom;

#[derive(Clone)]
pub struct Entry {
	n: u32,
}

impl Entry {
	fn new () -> Entry {
		Entry {
			n: 0,
		}
	}

	fn next (&mut self) -> bool {
		self.n += 1;
		if self.n == 10 {
			self.n = 0;
			true
		}
		else {
			false
		}
	}
}

pub struct Code {
	entries: Vec<Entry>,
	max: u32,
}

impl Code {
	pub fn new (len: usize) -> Code {
		Code {
			entries: vec![Entry::new();len],
			max: 10u32.pow(u32::try_from(len).unwrap() - 1),
		}
	}

	pub fn code (&self) -> u32 {
		let mut c = 0;
		for i in 0..self.entries.len() {
			c += 10u32.pow(u32::try_from(i).unwrap())*self.entries[i].n;
		}
		c
	}
}

impl Iterator for Code {
	type Item = u32;

	fn next (&mut self) -> Option<Self::Item> {
		if !self.entries.is_empty() {
			let c = self.code();
			let mut p = 0;
			while self.entries[p].next() {
				p += 1;
				if p == self.entries.len() {
					break;
				}
			}
			if c == self.max {
				None
			}
			else {
				Some(c)
			}
		}
		else {
			None
		}
	}
}

fn main () {
	let code = Code::new(6);
	for c in code {
		println!("{}",c);
	}
}
