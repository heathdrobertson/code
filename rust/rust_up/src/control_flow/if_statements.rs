fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn basics(){
    println!("Howdy Dude, it is:");

    let temp = 5;

    let day = if temp > 20 {"sunny"} else {"cloudy"}; // Nested if statements are also permited in this format.

    if temp > 30 {
        println!("Really hot and {} outside!", &day);
    } else if temp < 10 {
        println!("Really cold and {} outside!", &day);
    } else {
        println!("The temp is just right and it is {}!", &day);
    }

    who_am_i();
}
