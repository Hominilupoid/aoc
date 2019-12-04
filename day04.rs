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
	//min: Vec<Entry>,
	max: u32,
}

impl Code {
	pub fn new (len: usize) -> Code {
		Code {
			entries: vec![Entry::new();len],
			//min: vec![Entry::new();len],
			max: 10u32.pow(u32::try_from(len).unwrap() - 1),
		}
	}

	pub fn with_bounds (min: u32, max: u32) -> Code {
		if max < min {
			panic!("Horrible!");
		}
		//let es = vec![Entry::new();(max as f32).log10().floor() as usize + 1];
		let mut c = Code {
			//entries: es.clone(),
			//min: es,
			entries: vec![Entry::new();(max as f32).log10().floor() as usize + 1],
			max,
		};
		while c.code() < min {
			c.next();
		}
		//c.min = c.entries.clone();
		c
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
				//self.entries = self.min.clone();
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
	let code = Code::with_bounds(236491,713787);
	for c in code {
		println!("{}",c);
	}
}
