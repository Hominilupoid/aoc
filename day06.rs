use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub struct Orbits {
	orbits: HashMap<String,Vec<String>>,
}

impl Orbits {
	pub fn new (hm: HashMap<String,Vec<String>>) -> Orbits {
		Orbits {
			orbits: hm,
		}
	}

	fn suborbits (&self, k: &str) -> u32 {
		if let Some(os) = self.orbits.get(k) {
			let mut x = 0;
			for o in os {
				x += self.suborbits(o) + 1;
			}
			x
		}
		else {
			0
		}
	}

	pub fn orbits (&self) -> u32 {
		let mut x = 0;
		for o in &self.orbits {
			x += self.suborbits(o.0);
		}
		x
	}

	pub fn find<'a> (&'a self, mut k: &'a str) -> Vec<String> {
		let mut v = Vec::new();
		'outer: loop {
			for os in &self.orbits {
				for o in os.1 {
					if o == k {
						v.push(o.to_owned());
						k = os.0;
						continue 'outer;
					}
				}
			}
			break;
		}
		v
	}
}

fn parse (x: &str) -> HashMap<String,Vec<String>> {
	let mut hm = HashMap::<String,Vec<String>>::new();
	let mut s = String::new();
	let mut par = String::new();
	for c in x.chars() {
		match c as u8 {
			10	=>	{
						let mut v = hm.get_mut(&par);
						if let Some(ref mut v) = v {
							v.push(s.to_owned());
						}
						else {
							let mut vs = Vec::new();
							vs.push(s.to_owned());
							hm.insert(par.to_owned(),vs);
						}
						s.clear();
					},
			13	=>	(),
			41	=>	{
						par = s.to_owned();
						s.clear();
					}
			_	=>	s.push(c),
		};
	}
	hm
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input06")?;
	file.read_to_string(&mut data)?;
	let orbits = Orbits::new(parse(&data));
	println!("{:?}",orbits.find("YOU"));
	println!("{:?}",orbits.find("SAN"));
	Ok(())
}
