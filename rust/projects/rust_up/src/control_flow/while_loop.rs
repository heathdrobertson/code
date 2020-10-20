fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn basics() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        if x == 64 { continue; }
        println!("x = {}", x);
    }

    // loop == while true
    // 1<<10 == 2^10 "2 to the power of 10"
    let mut y = 1;
    loop { 
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 { break; }
    }

    who_am_i();
}
