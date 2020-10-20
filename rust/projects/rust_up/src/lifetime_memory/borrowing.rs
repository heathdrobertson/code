fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

pub fn main() {
    let print_vector = |x:&Vec<i32>| { // The type is changed from vector to a reference
        println!("{:?}", x);
        println!("x[0] = {}", x[0]);
    };

    let v = vec![1, 2, 3]; // variable on stack and data on the heap.
    print_vector(&v); // print_vector borrows the varriable.
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);
    who_am_i();

    let z = vec![3, 4, 5];

    for i in &z {
        println!("i = {}", i);
        // z.push(5); this fails.
    }
}

