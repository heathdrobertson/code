#![allow(dead_code)]

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

// <T> lets us define a generic type
// Using generic types example: A strct with different data types

struct Point<T, V> {
    x: T,
    y: V
}

struct Line<T, V> {
    start: Point<T, V>,
    end: Point<T, V>
}

fn generic_line() {
    // You can define f64 with deciaml point or appending f64 to the number.
    let a:Point<f64, f64> = Point {x: 0.0, y: 4f64 };  
    let b = Point {x: 1.2, y: 3.4};
    let _myline = Line { start: a, end: b};
}

pub fn main() {

    generic_line();
    who_am_i();
}
