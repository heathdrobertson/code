fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}


pub fn main(){
    let mut a =  2+3*4;  // +=*/
    a += 1;
    println!("{}", a);
    a -= 2;

    println!("a now = {}", a);
    println!("Modulo of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{}",a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // f64 has functions and constants like PI

    println!("b={}, b_cubed={}, b_to_pi={}", b, b_cubed, b_to_pi);

    //bitwise
    let c = 1 | 2; // |=OR, &=AND, ^=XOR, !=NOR
    println!("1|2 ={}", c);

    let two_to_10 = 1 << 10; // >>

    println!("2^10 = {}", two_to_10);

    // logical
    let _pi_less_4 = std::f64::consts::PI < 4.0; // true
    let x = 5;
    let x_is_5 = x ==5; // true
    println!("x == 5 is {}", x_is_5);

    who_am_i();
}
