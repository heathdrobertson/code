fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

pub fn main() {
    let x = 3.0;
    let y = 0.0;

    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None};
    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannont divide {} by {}", x, y)
    }

    // e.g. if let / while let 
    if let Some(z) = result { println!("z = {}", z); }
    let doc_url = "https://doc.rust-lang.org/std/option/enum.Option.html";
    println!("\nRust Option and Some(T) Doc: {}", doc_url);

    who_am_i();
}
