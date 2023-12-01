fn main(){
    let mut x = String::from("hello");
    let y = &mut x;
    println!("{}",y);
}
