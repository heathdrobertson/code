/* The key property of unions is that all fields of a union share common
 * storage. As a result writes to one field of a union can overwrite its other
 * fields, and size of a union is determined by the size of its largest field.
 *
 * This example is primarily used to re alocate values to the same memory slot.
 */

fn who_am_i() {
    println!("\nWho Am I >> {}", module_path!());
}

union IntOrFloar {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloar) {
    unsafe {
        match iof {
            IntOrFloar {i: 42} => {println!("The meaning of life");}
            IntOrFloar {f} => {println!("f32 = {}", f);}
        }
    }
}
pub fn main() {

    let mut iof = IntOrFloar {i: 123};
    iof.i = 42;
    let _value = unsafe {iof.i};

    process_value(iof);
    process_value(IntOrFloar {f: 1.23});

    let doc_url = "https://doc.rust-lang.org/reference/items/unions.html";
    println!("Rust Unions Doc: {}", doc_url);

    who_am_i();
}
