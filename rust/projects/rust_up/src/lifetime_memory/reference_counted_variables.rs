use std::rc::Rc;

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person {name: name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("Heath".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

}

pub fn main() {
    rc_demo();
    who_am_i();
}

