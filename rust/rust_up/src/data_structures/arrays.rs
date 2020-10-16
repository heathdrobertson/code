use std::mem;

fn who_am_i() {
    println!("Who Am I >> {}", module_path!());
}

pub fn main() {
    let mut a:[i32;5] = [1, 2, 3, 4, 5];
    
    println!("A has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("Now a[0] = {}", a[0]);

    println!("{:?}", a);

    if a == [321, 2, 3, 4, 5] {
        println!("Match!");
    }

    let b = [1u16; 10]; // adding u16 down from u32, reduces memory to 20 bytes

    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3]; 2] =  [ // [[f32;3]; 2] a 2 element matrix with 3 element
        // arrays.
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 3.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }

    who_am_i();
}
