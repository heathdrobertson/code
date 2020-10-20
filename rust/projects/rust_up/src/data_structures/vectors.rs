fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

pub fn main() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(4);
    println!("a = {:?}", a);

    // usize isize

    let idx:usize = 3; // index must be within currnet range 

    a[idx] = 312; // replaces value at index idx on arrary a
    println!("a[{}] = {}", idx, a[idx]);
    println!("a = {:?}", a);
    
    // Option
    let new_idx = 3;
    match a.get(new_idx) {
        Some(x) => println!("a[{}] = {}", new_idx, x),
        None => println!("Error, no value at Index {} in array.", new_idx)
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("{:?}", a);
    
    // Some(77)
    let last_elem = a.pop(); //Option
    println!("Last elem popped and is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("Popped elem is:{}", x);
    }

    who_am_i();
}
