#![allow(unused_variables)]
#![allow(dead_code)]

fn who_am_i() {
    println!("This is how you control lifetime");
    println!("\nWho Am I >> {}", module_path!());
}

struct Person {
    name: String
}

impl Person {
    // Rust gives you:  ' single quote = lifetime
    // fn get_ref_name<'a>(&'a self) -> &'a String
    fn get_ref_name(&self) -> &String { 
        &self.name
    }
}

struct Company<'z>{ // <'z> sets the same lifetime to Company as Person
    name: String,
    ceo: &'z Person
}

pub fn main() {
    let z: &String;
    {
        let p = Person {name: String::from("Heath") };
        // z = p.get_ref_name();
    }
    // let boss = Person {name: String::from("Heath Robertson") };
    // let toilethill = Company {name: String::from("ToiletHill"), ceo: &boss};
    who_am_i();
}

