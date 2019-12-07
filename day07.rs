
fn permute (a: &[i32], v: &[i32], vs: &mut Vec<Vec<i32>>) {
	if a.is_empty() {
		vs.push(v.to_vec());
	}
	else {
		for i in 0..a.len() {
			let mut a = a.to_vec();
			let mut v = v.to_vec();
			v.push(a.remove(i));
			permute(&a,&v,vs);
		}
	}
}

fn main () {
	let a = [0,1,2,3,4];
	let v = Vec::new();
	let mut vs = Vec::new();
	permute(&a,&v,&mut vs);
	println!("{:?}",vs);
}
