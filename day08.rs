use std::fs::File;
use std::io::Read;

fn parse (s: &str) -> Vec<i32> {
	let mut v = Vec::new();
	for c in s.chars() {
		match c as u8 {
			n @ 48..=57	=>	v.push(n as i32 - 48),
			_			=>	(),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input08")?;
	file.read_to_string(&mut data)?;
	let v = parse(&data);
	let mut ls = Vec::new();
	ls.push(Vec::new());
	for p in &v {
		let l = ls.len() - 1;
		ls[l].push(p);
		if ls[l].len() == 150 {
			ls.push(Vec::new());
		}
	}
	let mut fewest = (-1,std::i32::MAX);
	for i in 0..ls.len() {
		let mut zs = 0;
		for p in &ls[i] {
			if **p == 0 {
				zs += 1;
			}
		}
		if zs != 0 && zs < fewest.1 {
			println!("{}",zs);
			fewest = (i as i32,zs);
		}
	}
	let mut xs = (0,0);
	for p in &ls[fewest.0 as usize] {
		if **p == 1 {
			xs.0 += 1;
		}
		if **p == 2 {
			xs.1 += 1;
		}
	}
	println!("{}",xs.0*xs.1);
	Ok(())
}
