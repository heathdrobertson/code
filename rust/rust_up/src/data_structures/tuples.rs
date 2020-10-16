fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

fn sum_n_project(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn main() {
    let x = 3;
    let y = 4;
    let sp = sum_n_project(x, y);

    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a={}, b={}", a, b);

    let sp2 = sum_n_project(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("Last elemenet={}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    println!("c={}, d={}, e={}, f={}", c, d, e, f);
    
    who_am_i();
}
