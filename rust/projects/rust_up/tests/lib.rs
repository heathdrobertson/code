#[cfg(test)]
extern crate rust_up;

use rust_up::odds_ends::consuming_crates::glue;

#[test]

fn glue_correct() {
    assert_eq!(true, glue());
}
