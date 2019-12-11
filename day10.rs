use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Asteroid {
	x: i32,
	y: i32,
}

impl Asteroid {
	pub fn new (x: i32, y: i32) -> Asteroid {
		Asteroid {
			x,
			y,
		}
	}
}

fn parse (s: &str) -> Vec<Asteroid> {
	let mut v = Vec::new();
	let (mut x,mut y) = (0,0);
	for c in s.chars() {
		match c as u8 {
			10			=>	{
								y += 1;
								x = 0;
							},
			35			=>	{
								v.push(Asteroid::new(x,y));
								x += 1;
							},
			46			=>	x += 1,
			_			=>	(),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input10")?;
	file.read_to_string(&mut data)?;

	println!("{:?}",parse(&data));

	Ok(())
}
