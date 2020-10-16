fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn main(){
    // Box<T> a generic type that allows us to keep variables for longer term, adds the to the stack and references
    // memory location on the heap.
    let x = Box::new(5555);
    println!("{}", *x);

    who_am_i();
}
