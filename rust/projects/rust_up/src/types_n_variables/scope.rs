fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn main() {
    // {} braces denote a scope.
    let a = 123;

    {
        let b = 456;
        println!("a={}, b={}", a, b);
    }
    println!("a={}", a);

    who_am_i();
}
