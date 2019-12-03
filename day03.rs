use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;

#[derive(Clone,Debug)]
pub enum Direction {
	Up(i32),
	Down(i32),
	Left(i32),
	Right(i32),
}

#[derive(Clone,Copy,Debug)]
pub struct Wire {
	id: u32,
	xy: [i32;2],
	dim: [i32;2],
}

impl Wire {
	pub fn new (id: u32, xy: [i32;2], dim: [i32;2]) -> Wire {
		Wire {
			id,
			xy,
			dim,
		}
	}
}

fn parse (s: &str) -> Vec<Vec<Direction>> {
	let mut wires = Vec::new();
	let mut v = Vec::new();
	let mut ns = None;
	let mut dir = None;
	for c in s.chars() {
		match c as u32 {
			10			=>	{
								v.push(match dir {
									Some('U')	=>	Direction::Up(ns.unwrap() as i32),
									Some('D')	=>	Direction::Down(ns.unwrap() as i32),
									Some('L')	=>	Direction::Left(ns.unwrap() as i32),
									Some('R')	=>	Direction::Right(ns.unwrap() as i32),
									_			=>	unreachable!(),
								});
								dir = None;
								ns = None;
								wires.push(v);
								v = Vec::new();
							},
			44			=>	{
								v.push(match dir {
									Some('U')	=>	Direction::Up(ns.unwrap() as i32),
									Some('D')	=>	Direction::Down(ns.unwrap() as i32),
									Some('L')	=>	Direction::Left(ns.unwrap() as i32),
									Some('R')	=>	Direction::Right(ns.unwrap() as i32),
									_			=>	unreachable!(),
								});
								dir = None;
								ns = None;
							},
			n @ 48..=57	=>	if let Some(x) = ns {
								ns = Some(10*x + n - 48)
							}
							else {
								ns = Some(n - 48)
							},
			_			=>	dir = Some(c),
		};
	}
	wires
}

fn main () -> std::io::Result<()> {
	let mut file = File::open("input03")?;
	let mut data = String::new();
	file.read_to_string(&mut data)?;
	let mut superwires = Vec::new();
	for (i,ds) in parse(&data).iter().enumerate() {
		let mut wires = Vec::new();
		let mut xy = [0,0];
		for d in ds {
			let dim = match d {
				&Direction::Up(y)	=>	[0,y],
				&Direction::Down(y)	=>	[0,-y],
				&Direction::Left(x)	=>	[-x,0],
				&Direction::Right(x)	=>	[x,0],
			};
			let wdim = [dim[0]/2,dim[1]/2];
			wires.push(Wire::new(i as u32,[xy[0] + wdim[0],xy[1] + wdim[1]],wdim));
			xy[0] += dim[0];
			xy[1] += dim[1];
		}
		superwires.push(wires);
	}
	let mut nearest = std::i32::MAX;
	let (wsa,wsb) = superwires.split_first().unwrap();
	for a in wsa {
		for b in &wsb[0] {
			if (a.xy[0] - b.xy[0]).abs() <= a.dim[0] + b.dim[0]
			&& (a.xy[1] - b.xy[1]).abs() <= a.dim[1] + b.dim[1] {
				let cx = [if a.dim[0] == 0 {
					a.xy[0]
				}
				else {
					b.xy[0]
				},if a.dim[1] == 0 {
					a.xy[1]
				}
				else {
					b.xy[1]
				}];
				let md = cx[0].abs() + cx[1].abs();
				if md < nearest {
					nearest = md;
				}
			}
		}
	}
	println!("SHORTEST MANHATTAN DISTANCE:\t{}",nearest);
	Ok(())
}
