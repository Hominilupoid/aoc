use std::fs::File;
use std::io::prelude::*;

fn parse (s: &str) -> Vec<i32> {
	let mut v = Vec::new();
	let mut ns = None;
	for c in s.chars() {
		match c as u8 {
			10			=>	{
								if let Some(x) = ns {
									v.push(x);
								}
								ns = None;
							},
			n @ 48..=57	=>	if let Some(x) = ns {
								ns = Some(10*x + n as i32 - 48)
							}
							else {
								ns = Some(n as i32 - 48)
							}
			x			=>	panic!("Horrible: {}",x),
		};
	}
	v
}

fn fuel (n: i32) -> i32 {
	let mut n = n;
	let mut f = 0;
	loop {
		n = n/3 - 2;
		if n <= 0 {
			break;
		}
		f += n;
	}
	f
}

fn main () -> std::io::Result<()> {
	let mut file = File::open("input01")?;
	let mut data = String::new();
	file.read_to_string(&mut data)?;
	let mut f = 0;
	for n in parse(&data) {
		f += fuel(n);
	}
	println!("{:?}",f);
	Ok(())
}
