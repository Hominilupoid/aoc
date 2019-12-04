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
	overflow: bool,
}

impl Code {
	pub fn new (len: usize) -> Code {
		Code {
			entries: vec![Entry::new();len],
			overflow: false,
		}
	}

	pub fn code (&self) -> u32 {
		let mut c = 0;
		for i in 0..self.entries.len() {
			c += 10u32.pow(u32::try_from(i).unwrap())*self.entries[i].n;
		}
		c
	}

	fn valid (&self) -> bool {
		if self.entries.len() < 2 {
			false
		}
		else {
			let mut n = self.entries[0].n;
			let mut d = 1;
			let mut double = false;
			for i in 1..self.entries.len() {
				if self.entries[i].n > n {
					return false;
				}
				else if self.entries[i].n == n {
					d += 1;
				}
				else {
					n = self.entries[i].n;
					if d == 2 {
						double = true;
					}
					d = 1;
				}
			}
			if d == 2 {
				double = true;
			}
			double
		}
	}
}

impl Iterator for Code {
	type Item = u32;

	fn next (&mut self) -> Option<Self::Item> {
		if self.overflow {
			None
		}
		else if !self.entries.is_empty() {
			if !self.valid() {
				loop {
					let mut p = 0;
					while self.entries[p].next() {
						p += 1;
						if p == self.entries.len() {
							return None;
						}
					}
					if self.valid() {
						break;
					}
				}
			}
			let c = self.code();
			loop {
				let mut p = 0;
				while self.entries[p].next() {
					p += 1;
					if p == self.entries.len() {
						self.overflow = true;
						return Some(c);
					}
				}
				if self.valid() {
					break;
				}
			}
			Some(c)
		}
		else {
			None
		}
	}
}

fn main () {
	let code = Code::new(6);
	let mut v = Vec::new();
	for c in code {
		v.push(c);
	}
	println!("{:?}",v.iter().filter(|&&x| x >= 236_491 && x <= 713_787).collect::<Vec<_>>().len());
}
