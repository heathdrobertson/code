use std::thread;
use std::sync::Arc;

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person {name: name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn arc_demo() {
    let name = Arc::new("Heath".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}
pub fn main() {

    arc_demo();
    who_am_i();
}

