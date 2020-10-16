#![allow(unused_variables)]
#![allow(dead_code)]

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
    println!("Closures, a function-like construct you can store in a variable");
    println!("A close is defined with | | pipes instead of ( ) ");
}

fn say_hello() { println!("hello"); }

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    { // This creates a narrower scope, so we can use two again if need be.
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z // Returns value
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }

    let borrow_two = &mut two;
    
    // T: by value
    // T&
    // &mut &
    
    let plus_three = |mut x:i32| { x += 3; &x; };
    let f = 12;
    plus_three(f);
    println!("f = {}", f);
    
}

pub fn main() {
    closures();
    who_am_i();
}

