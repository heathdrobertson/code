fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


const GLOBAL_VARRIABLE: u8 = 44;

pub fn main() {
    println!("{}", GLOBAL_VARRIABLE);

    who_am_i();
}
