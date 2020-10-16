#![allow(unused_imports)]
// #![allow(unused_code)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

extern crate rust_up;

use rust_up::traits;
use rust_up::functions;
use rust_up::data_structures;
use rust_up::control_flow;
use rust_up::types_n_variables;
use rust_up::lifetime_memory;
use rust_up::odds_ends;

fn main() {
    odds_ends::consuming_crates::main(); 
    // lifetime_memory::using_mutex_thread_safe::main();
    // traits::why_dynamic_dispatch::main();
    // functions::high_order_fn::main();
    // data_structures::generics::main();
    // types_n_variables::boxes::main();
    // data_structures::vectors::main();
    // control_flow::while_loop::main();
}
