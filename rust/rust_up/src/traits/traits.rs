fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk!", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        Human{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello Rust world!", self.name());
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        Cat{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow Rust world!", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {result += *x;}
        return result;
    }
}

fn traits() {
    //let h = Human{name: "Heath"};
    //let h = Human::create("Heath");
    let h:Human = Animal::create("Heath");
    h.talk();

    let c = Cat{name: "Ebony"};
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}

pub fn main() {
    traits();    
    who_am_i();
}

