use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
struct Orbit {
	id: String,
	parent: Option<usize>,
}

impl Orbit {
	fn new (id: &str, parent: Option<usize>) -> Orbit {
		Orbit {
			id: id.to_owned(),
			parent,
		}
	}
}

pub struct Orbits {
	orbits: Vec<Orbit>,
}

impl Orbits {
	pub fn new (v: Vec<(String,String)>) -> Orbits {
		let mut hm = HashMap::new();
		let mut os = Vec::new();
		for ss in &v {
			if !hm.contains_key(&ss.0) {
				os.push(Orbit::new(&ss.0,None));
				hm.insert(ss.0.to_owned(),os.len() - 1);
			}
			if !hm.contains_key(&ss.1) {
				os.push(Orbit::new(&ss.1,Some(*hm.get(&ss.0).unwrap())));
				hm.insert(ss.1.to_owned(),os.len() - 1);
			}
			else {
				os[*hm.get(&ss.1).unwrap()].parent = Some(*hm.get(&ss.0).unwrap());
			}
		}
		Orbits {
			orbits: os,
		}
	}

	fn suborbits (&self, mut i: usize) -> u32 {
		let mut n = 0;
		while let Some(p) = self.orbits[i].parent {
			n += 1;
			i = p;
		}
		n
	}

	pub fn orbits (&self) -> u32 {
		let mut n = 0;
		for i in 0..self.orbits.len() {
			n += self.suborbits(i);
		}
		n
	}

	pub fn find (&self, k: &str) -> Vec<String> {
		let mut v = Vec::new();
		for o in &self.orbits {
			if o.id == k {
				let mut p = o.parent;
				while let Some(i) = p {
					v.push(self.orbits[i].id.to_owned());
					p = self.orbits[i].parent;
				}
			}
		}
		v
	}
}

fn parse (x: &str) -> Vec<(String,String)> {
	let mut v = Vec::new();
	let mut s = String::new();
	let mut s0 = String::new();
	for c in x.chars() {
		match c as u8 {
			10	=>	{
						v.push((s0.to_owned(),s.to_owned()));
						s.clear();
					},
			13	=>	(),
			41	=>	{
						s0 = s.to_owned();
						s.clear();
					}
			_	=>	s.push(c),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input06")?;
	file.read_to_string(&mut data)?;
	let orbits = Orbits::new(parse(&data));
	println!("{}",orbits.orbits());
	let (mut to_you,mut to_san) = (orbits.find("YOU"),orbits.find("SAN"));
	while to_you.last() == to_san.last() {
		to_you.pop();
		to_san.pop();
	}
	println!("{}",to_you.len() + to_san.len());
	Ok(())
}
