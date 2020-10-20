fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn main() {
    // str is a sequence of utf-8 
    let s:&'static str = "Hello there!"; // &str = string slice

    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First letter of s is: {}", first_char);
    }
    println!("\n-----------------------\n");
    // heap
    // String is kind of a vector
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // To change from &str <> String
    let u:&str = &letters;
    
    println!("{}", u);

    // Concatenation
    // String + str
    // let z = letters + &letters;

    let mut abc = "Hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "Goodbye"));

    who_am_i();
}
