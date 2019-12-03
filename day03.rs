use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;

#[derive(Clone,Debug)]
pub enum Direction {
	Up(usize),
	Down(usize),
	Left(usize),
	Right(usize),
}

#[derive(Clone,Copy)]
pub struct Wire {
	id: usize,
}

impl Wire {
	pub fn new (id: usize) -> Wire {
		Wire {
			id,
		}
	}
}

pub struct WireGrid {
	grid: Vec<Vec<Vec<Wire>>>,
}

impl WireGrid {
	pub fn new (wh: [usize;2]) -> WireGrid {
		WireGrid {
			grid: vec![vec![Vec::new();wh[0]];wh[1]],
		}
	}

	fn at (&self, xy: [usize;2]) -> &Vec<Wire> {
		&self.grid[xy[1]][xy[0]]
	}

	fn at_mut (&mut self, xy: [usize;2]) -> &mut Vec<Wire> {
		&mut self.grid[xy[1]][xy[0]]
	}

	fn center (&mut self) -> &mut Vec<Wire> {
		let (y,x) = (self.grid.len()/2,self.grid[0].len()/2);
		&mut self.grid[y][x]
	}

	pub fn add (&mut self, ds: &[Direction], w: Wire) {
		let xy = self.center();
		self.center().push(w);
		for d in ds {
			match d {
				Direction::Up(x)	=>	println!("up({}):{}",w.id,x),
				Direction::Down(x)	=>	println!("down({}):{}",w.id,x),
				Direction::Left(x)	=>	println!("left({}):{}",w.id,x),
				Direction::Right(x)	=>	println!("right({}):{}",w.id,x),
			}
		}
	}
}

fn parse (s: &str) -> Vec<Vec<Direction>> {
	let mut wires = Vec::new();
	let mut v = Vec::new();
	let mut ns = None;
	let mut dir = None;
	for c in s.chars() {
		match c as usize {
			10			=>	{
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
			44			=>	{
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
	let mut wg = WireGrid::new([1024,1024]);
	for (i,ds) in parse(&data).iter().enumerate() {
		let w = Wire::new(i);
		wg.add(&ds,w);
	}
	Ok(())
}
