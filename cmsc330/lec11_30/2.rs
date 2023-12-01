fn main() {
	let mut x = String::from("kndsj");
	let y = &mut x;
	let z = &mut x;
	println!("y = {}, z = {}", y, z);
}
