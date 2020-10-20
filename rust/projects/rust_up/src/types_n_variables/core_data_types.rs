fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


use std::mem;

pub fn main() {
    // u = unsigned 0 or positive +
    // i = signed neg or pos
    let a:u8 = 123;
    println!("a = {}", a);

    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b now = {}", b);

    let mut c = 123456789;
    println!("c = {}, size of c = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after mutated 'mut'", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, and my system is {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size of d = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.5; //double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size of e = {} bytes", e, mem::size_of_val(&e));

    // boolean 
    let g:bool = false;
    println!("g = {}, size of g = {} bytes", g, mem::size_of_val(&g));

    who_am_i();
}
