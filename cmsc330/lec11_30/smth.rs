fn main() {
	let x = String::from("Hello"); //x owns value
	let y = x; // y now owns, x becomes invalid
	let z = y; // z now owns, y becomes invalid
	let a = String::from("Hello"); // a now owns it’s own copy of hello
	let b = a; // b now owns the second hello, a is invalid
	println!("{} is {}", z, b); // this is fine
}
