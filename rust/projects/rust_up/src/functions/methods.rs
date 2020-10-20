fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line { // This is how you add behavior, with a method, to structs.
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
    
    println!("length = {}", myline.len());
    who_am_i();
}

