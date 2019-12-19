use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Chemical {
	ingredients: Vec<(u32,String)>,
	batch: u32,
}

impl Chemical {
	pub fn new (v: &[(u32,String)], n: u32) -> Chemical {
		Chemical {
			ingredients: v.to_vec(),
			batch: n,
		}
	}
}

pub struct Chemicalizer {
	chemicals: HashMap<String,Chemical>,
	surplus: HashMap<String,u32>,
}

impl Chemicalizer {
	pub fn new (hm: HashMap<String,Chemical>) -> Chemicalizer {
		let mut surplus = HashMap::new();
		for (k,_) in &hm {
			surplus.insert(k.to_owned(),0);
		}
		Chemicalizer {
			chemicals: hm,
			surplus,
		}
	}

	pub fn generate<'a> (&'a mut self, mut s: &'a str) -> u32 {
		let mut x = 0;
		let mut is = Vec::new();
		is.push(s);
		while !is.is_empty() {
			s = is.remove(0);
			let c = self.chemicals.get(s).unwrap();
			for ii in 0..c.ingredients.len() {
				let n = c.ingredients[ii].0;
				let i = &c.ingredients[ii].1;
				if i == "ORE" {
					x += n;
				}
				else {
					if self.surplus[i] >= n {
						*self.surplus.get_mut(i).unwrap() -= n;
					}
					else {
						while self.surplus[i] < n {
							*self.surplus.get_mut(i).unwrap() += self.chemicals[i].batch;
							is.push(i);
						}
						*self.surplus.get_mut(i).unwrap() -= n;
					}
				}
			}
		}
		println!("{:?}",self.surplus);
		x
	}
}

fn parse (s: &str) -> HashMap<String,Chemical> {
	let mut hm = HashMap::new();
	let mut is = Vec::new();
	let mut r = (0,String::new());
	let mut n = String::new();
	for c in s.chars() {
		match c as u8 {
			10			=>	{
								hm.insert(r.1.to_owned(),Chemical::new(&is,r.0));
								r.0 = 0;
								r.1 = String::new();
								is.clear();
							},
			44			=>	{
								is.push((r.0,r.1.to_owned()));
								r.0 = 0;
								r.1.clear();
							},
			48..=57		=>	n.push(c),
			61			=>	{
								is.push((r.0,r.1.to_owned()));
								r.0 = 0;
								r.1.clear();
							},
			65..=90		=>	{
								if r.1.is_empty() {
									r.0 = str::parse(&n).unwrap();
									n.clear();
								}
								r.1.push(c);
							},
			_			=>	(),
		};
	}
	hm
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input14")?;
	file.read_to_string(&mut data)?;
	let mut c = Chemicalizer::new(parse(&data));
	println!("{}",c.generate("FUEL"));

	Ok(())
}
