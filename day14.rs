use std::fs::File;
use std::io::Read;

fn parse (s: &str) -> Vec<(i32,String)> {
	let mut v = Vec::new();
	let mut r = (0,String::new());
	let mut n = String::new();
	for c in s.chars() {
		match c as u8 {
			10			=>	{
								v.push((r.0,r.1.to_owned()));
								r.0 = 0;
								r.1 = String::new();
							},
			44			=>	{
								v.push((r.0,r.1.to_owned()));
								r.0 = 0;
								r.1 = String::new();
							},
			48..=57		=>	n.push(c),
			61			=>	{
								v.push((r.0,r.1.to_owned()));
								r.0 = 0;
								r.1 = String::new();
							},
			65..=90		=>	{
								if r.1.is_empty() {
									r.0 = str::parse(&n).unwrap();
									n = String::new();
								}
								r.1.push(c);
							},
			_			=>	(),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input14")?;
	file.read_to_string(&mut data)?;

	println!("{:?}",parse(&data));

	Ok(())
}
