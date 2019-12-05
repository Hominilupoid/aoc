use std::fs::File;
use std::io::{stdin,stdout,BufRead,Read,Write};

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

	fn add (&mut self) -> usize {
		let ps = self.params();
		self.input[ps[2]] = self.input[ps[0]] + self.input[ps[1]];
		4
	}

	fn mply (&mut self) -> usize {
		let ps = self.params();
		self.input[ps[2]] = self.input[ps[0]]*self.input[ps[1]];
		4
	}

	fn read (&mut self) -> usize {
		let mut s = String::new();
		let ps = self.params();
		print!("Enter integer: ");
		stdout().flush();
		stdin().read_line(&mut s).expect("Input error");
		self.input[ps[0]] = s.trim().parse().expect("Not an integer");
		2
	}

	fn write (&mut self) -> usize {
		let ps = self.params();
		println!("{}",self.input[ps[0]]);
		2
	}

	fn at (&self, offset: usize) -> usize {
		self.input[self.cursor + offset] as usize
	}

	fn params (&self) -> [usize;3] {
		[self.at(1),self.at(2),self.at(3)]
	}

	pub fn run (&mut self) -> Vec<i32> {
		loop {
			self.cursor += match self.input[self.cursor] {
				1	=>	self.add(),
				2	=>	self.mply(),
				3	=>	self.read(),
				4	=>	self.write(),
				99	=>	break,
				x	=>	panic!("Horrible at {}: {}",self.cursor,x),
			};
		}
		self.input.clone()
	}
}

fn parse (s: &str) -> Vec<i32> {
	let mut v = Vec::new();
	let mut ns = None;
	let mut neg = false;
	for c in s.chars() {
		match c as u8 {
			45			=>	neg = true,
			n @ 48..=57	=>	if let Some(x) = ns {
								ns = Some(10*x + n as i32 - 48)
							}
							else {
								ns = Some(n as i32 - 48)
							},
			_			=>	{
								if let Some(x) = ns {
									v.push(if neg {
										neg = false;
										-x
									}
									else {
										x
									});
								}
								ns = None;
							},
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input05")?;
	file.read_to_string(&mut data)?;
	println!("{:?}",Komputer::new(parse(&data)).run());
	Ok(())
}
