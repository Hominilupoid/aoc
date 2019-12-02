use std::fs::File;
use std::io::prelude::*;

pub struct Komputer {
	input: Vec<i32>,
	cursor: usize,
}

impl Komputer {
	pub fn new (input: Vec<i32>) -> Komputer {
		Komputer {
			input,
			cursor: 0,
		}
	}

	fn add (&mut self, ps: [usize;3]) {
		self.input[ps[2]] = self.input[ps[0]] + self.input[ps[1]];
	}

	fn mply (&mut self, ps: [usize;3]) {
		self.input[ps[2]] = self.input[ps[0]]*self.input[ps[1]];
	}

	fn at (&self, offset: usize) -> usize {
		self.input[self.cursor + offset] as usize
	}

	pub fn run (&mut self) -> Vec<i32> {
		loop {
			match self.input[self.cursor] {
				1	=>	self.add([self.at(1),self.at(2),self.at(3)]),
				2	=>	self.mply([self.at(1),self.at(2),self.at(3)]),
				99	=>	break,
				_	=>	panic!("Horrible!"),
			}
			self.cursor += 4;
		}
		self.input.clone()
	}
}

fn parse (s: &str) -> Vec<i32> {
	let mut v = Vec::new();
	let mut ns = None;
	for c in s.chars() {
		match c as u8 {
			10			=>	(),
			44			=>	{
								if let Some(x) = ns {
									v.push(x);
								}
								ns = None;
							},
			n @ 48..=57	=>	if let Some(x) = ns {
								ns = Some(10*x + n as i32 - 48)
							}
							else {
								ns = Some(n as i32 - 48)
							}
			x			=>	panic!("Horrible: {}",x),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	//let mut file = File::open("input02")?;
	let mut file = File::open("input02x")?;
	let mut data = String::new();
	file.read_to_string(&mut data)?;
	for noun in 0..=100 {
		for verb in 0..=100 {
			let mut data = parse(&data);
			data[1] = noun;
			data[2] = verb;
			let output = Komputer::new(data).run();
			if output[0] == 19_690_720 {
				println!("{}",100*noun + verb);
				break;
			}
		}
	}
	Ok(())
}
