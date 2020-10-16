fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn basics() {
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { 
            println!("Found 8, all done!");
            break; 
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
    who_am_i();
}
