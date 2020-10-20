fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

// Memory Safety


pub fn main() {
    let v = vec![1, 2, 3]; // variable on stack and data on the heap.

    // let v2 = v; // varibale v is no longer usable. error[E0382]

    // let foo = |v:Vec<i32>| ();
    // foo(v);

    // println!("{:?}", v);
    
    let u = 1; // i32 is cheap to copy 32bits of data
    let _u2 = u;

    // println!("u = {}", *u);

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{}", vv[0]);
    who_am_i();
}

