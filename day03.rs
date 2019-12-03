use std::fs::File;
use std::io::prelude::*;
use std::fmt::{Debug,Formatter,Result};

enum Direction {
	Up(usize),
	Down(usize),
	Left(usize),
	Right(usize),
}

impl Debug for Direction {
	fn fmt (&self, f: &mut Formatter<'_>) -> Result {
		let mut s = String::new();
		match self {
			Direction::Up(x)	=>	{
										s.push_str("up:");
										s.push_str(&x.to_string());
									},
			Direction::Down(x)	=>	{
										s.push_str("down:");
										s.push_str(&x.to_string());
									},
			Direction::Left(x)	=>	{
										s.push_str("left:");
										s.push_str(&x.to_string());
									},
			Direction::Right(x)	=>	{
										s.push_str("right:");
										s.push_str(&x.to_string());
									},
		};
		write!(f,"{}",s)
	}
}

fn parse (s: &str) -> Vec<Vec<Direction>> {
	let mut wires = Vec::new();
	let mut v = Vec::new();
	let mut ns = None;
	let mut dir = None;
	for c in s.chars() {
		match c as usize {
			10							=>	{
												v.push(match dir {
													Some('U')	=>	Direction::Up(ns.unwrap()),
													Some('D')	=>	Direction::Down(ns.unwrap()),
													Some('L')	=>	Direction::Left(ns.unwrap()),
													Some('R')	=>	Direction::Right(ns.unwrap()),
													_			=>	unreachable!(),
												});
												dir = None;
												ns = None;
												wires.push(v);
												v = Vec::new();
											},
			44							=>	{
												v.push(match dir {
													Some('U')	=>	Direction::Up(ns.unwrap()),
													Some('D')	=>	Direction::Down(ns.unwrap()),
													Some('L')	=>	Direction::Left(ns.unwrap()),
													Some('R')	=>	Direction::Right(ns.unwrap()),
													_			=>	unreachable!(),
												});
												dir = None;
												ns = None;
										},
			n @ 48..=57					=>	if let Some(x) = ns {
												ns = Some(10*x + n - 48)
											}
											else {
												ns = Some(n - 48)
											},
			_ 							=>	dir = Some(c),
		};
	}
	wires
}

fn main () -> std::io::Result<()> {
	let mut file = File::open("input03")?;
	let mut data = String::new();
	file.read_to_string(&mut data)?;
	println!("{:?}",parse(&data));
	Ok(())
}
