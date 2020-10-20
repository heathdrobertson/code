#![allow(dead_code)]

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

fn print_values(x: i32) {
    println!("print_values = {}", x);
}

fn increase(x: &mut i32) {
   *x += 1; 
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn functions() {
    print_values(22);

    let mut z = 1;
    increase(&mut z);
    println!("increase z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

pub fn main() {
    functions();
    who_am_i();
}

