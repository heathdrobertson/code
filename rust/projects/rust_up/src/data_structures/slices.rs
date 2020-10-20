fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

fn use_slice(slice: &mut[i32]) {
    println!("First element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn main() {
    let mut data = [1, 2, 3, 4, 5];

    println!("{:?}", data);
    // use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);

    who_am_i();
}
