fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn basics() {
    let country_code = 1000;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        50 => "Mexico",
        1..=999 => "Unknown", // A range not in a loop 3 dots and also includes the final value, 999 in this case.
        _ => "invalid"
    };
    println!("The country with code {} is {}", country_code, country);

    who_am_i();
}
