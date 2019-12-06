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

	fn add (&mut self, modes: [i32;2]) -> usize {
		let p = self.offset(3);
		self.input[p] = self.param(0,&modes) + self.param(1,&modes);
		4
	}

	fn mply (&mut self, modes: [i32;2]) -> usize {
		let p = self.offset(3);
		self.input[p] = self.param(0,&modes)*self.param(1,&modes);
		4
	}

	fn read (&mut self) -> usize {
		let mut s = String::new();
		print!("Enter integer: ");
		stdout().flush();
		stdin().read_line(&mut s).expect("Input error");
		let p = self.offset(1);
		self.input[p] = s.trim().parse().expect("Not an integer");
		2
	}

	fn write (&mut self, modes: [i32;1]) -> usize {
		println!("{}",self.param(0,&modes));
		2
	}

	fn offset (&self, p: usize) -> usize {
		self.input[self.cursor + p] as usize
	}

	fn param (&self, p: usize, modes: &[i32]) -> i32 {
		if modes[p] == 0 {
			self.input[self.input[self.cursor + p + 1] as usize]
		}
		else {
			self.input[self.cursor + p + 1]
		}
	}

	pub fn run (&mut self) -> Vec<i32> {
		loop {
			let mut x = self.input[self.cursor];
			let m2 = x/1000;
			x = x%1000;
			let m1 = x/100;
			x = x%100;
			println!("{} {} {:02}",m2,m1,x);
			self.cursor += match x {
				1	=>	self.add([m1,m2]),
				2	=>	self.mply([m1,m2]),
				3	=>	self.read(),
				4	=>	self.write([m1]),
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
