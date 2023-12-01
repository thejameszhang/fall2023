fn main() {
	let x = String::from("Hello");
	let y = x;
	let z = identity(y);
	println!("z={}", z);
}

fn identity(w : String) -> String {
	w
}
