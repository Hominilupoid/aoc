use std::fs::File;
use std::io::{/*stdin,stdout,BufRead,*/Read/*,Write*/};

pub struct Komputer {
	input: Vec<i32>,
	params: Vec<i32>,
	output: Vec<i32>,
	cursor: usize,
	pause: bool,
}

impl Komputer {
	pub fn new (input: &[i32], params: &[i32]) -> Komputer {
		Komputer {
			input: input.to_vec(),
			params: params.to_vec(),
			output: Vec::new(),
			cursor: 0,
			pause: false,
		}
	}

	fn params (&mut self, ps: &[i32]) {
		self.params.extend_from_slice(ps);
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
		/*
		let mut s = String::new();
		print!("Enter integer: ");
		stdout().flush();
		stdin().read_line(&mut s).expect("Input error");*/
		let p = self.offset(1);
		if !self.params.is_empty() {
			self.input[p] = self.params.remove(0);
			2
		}
		else {
			self.pause = true;
			0
		}
	}

	fn write (&mut self, modes: [i32;1]) -> usize {
		self.output.push(self.param(0,&modes));
		2
	}

	fn jumpt (&mut self, modes: [i32;2]) -> usize {
		if self.param(0,&modes) != 0 {
			self.cursor = self.param(1,&modes) as usize;
			0
		}
		else {
			3
		}
	}

	fn jumpf (&mut self, modes: [i32;2]) -> usize {
		if self.param(0,&modes) == 0 {
			self.cursor = self.param(1,&modes) as usize;
			0
		}
		else {
			3
		}
	}

	fn less (&mut self, modes: [i32;2]) -> usize {
		let p = self.offset(3);
		self.input[p] = if self.param(0,&modes) < self.param(1,&modes) {
			1
		}
		else {
			0
		};
		4
	}

	fn equal (&mut self, modes: [i32;2]) -> usize {
		let p = self.offset(3);
		self.input[p] = if self.param(0,&modes) == self.param(1,&modes) {
			1
		}
		else {
			0
		};
		4
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
			self.cursor += match x {
				1	=>	self.add([m1,m2]),
				2	=>	self.mply([m1,m2]),
				3	=>	self.read(),
				4	=>	self.write([m1]),
				5	=>	self.jumpt([m1,m2]),
				6	=>	self.jumpf([m1,m2]),
				7	=>	self.less([m1,m2]),
				8	=>	self.equal([m1,m2]),
				99	=>	break,
				x	=>	panic!("Horrible at {}: {}",self.cursor,x),
			};
			if self.pause {
				self.pause = false;
				break;
			}
		}
		let o = self.output.clone();
		self.output.clear();
		o
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

fn permute (a: &[i32], v: &[i32], vs: &mut Vec<Vec<i32>>) {
	if a.is_empty() {
		vs.push(v.to_vec());
	}
	else {
		for i in 0..a.len() {
			let mut a = a.to_vec();
			let mut v = v.to_vec();
			v.push(a.remove(i));
			permute(&a,&v,vs);
		}
	}
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input07")?;
	file.read_to_string(&mut data)?;

	let a = [5,6,7,8,9];
	let v = Vec::new();
	let mut vs = Vec::new();
	permute(&a,&v,&mut vs);

	let mut highest = 0;
	for v in &vs {
		let mut ks = Vec::new();
		for i in 0..5 {
			ks.push(Komputer::new(&parse(&data),&[v[i]]));
		}
		let mut o = 0;
		let mut i = 0;
		loop {
			ks[i].params(&[o]);
			print!("{:?}",ks[i].params);
			let x = ks[i].run();
			if x.is_empty() {
				break;
			}
			o = x[0];
			println!(" -> {:?}",o);
			i += 1;
			if i == ks.len() {
				i = 0;
			}
		}
		if o > highest {
			highest = o;
		}
	}
	println!("\n\n{}",highest);

	Ok(())
}
