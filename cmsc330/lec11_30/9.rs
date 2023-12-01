fn make_vec() -> Vec<String>{
  let s = String::from("abc");
  let mut v = vec![];
  v.push(s);
  return v;
}
fn use_make_vec() {
  let v = make_vec();
  println!("v[0]: {}",v[0]);
}
fn main() {
	use_make_vec();
}
