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

pub fn main() {
    let p = Point {
        x: 3.0, 
        y: 4.0
    };
    println!("Point P is at: ({}, {})", p.x, p.y);
    
    let p2 = Point {
        x: 5.0,
        y: 10.0
    };

    let my_line = Line { start: p, end: p2 };
    println!("start point: ({}, {}), end point: ({}, {})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);

    who_am_i();
}
