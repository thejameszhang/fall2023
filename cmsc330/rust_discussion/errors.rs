pub fn sum_evens(i: i32, j: i32) -> i32 {
    // Make the sum variable mutable
    let mut sum = 0;

    for k in i..j {
        if k % 2 == 0 {
            sum += k;
        }
    }
    sum
}

pub fn distance((ax, ay): (f64, f64), (bx, by): (f64, f64)) -> f64 {
    // Powi for integer powers
    ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt()
}

pub fn raise_1(arr: &mut [i32]) {
    for i in arr {
        // Dereference each pointer to the array index to get the element
        *i += 1;
    }
}

pub fn add_hello(a: &mut String) {
    // Make the string mutable
    a.push_str("hello");

}

pub fn create_hello_world() -> String {
    let mut s = String::from("");
    // Pass in a &mut s to match the types
    add_hello(&mut s);
    s.push_str("world");

    return s;
}

pub fn get_first_elem(a: &Vec<u32>) -> u32 {
    if a.len() == 0 {
        return 0;
    }
    // Unwrap it because get returns type Option, so .unwrap() extracts the value.
    return *a.get(0).unwrap();
}

fn main() {
    println!("yay!");
}