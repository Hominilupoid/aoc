use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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
	println!("{:?}",parse(&data));
	Ok(())
}
