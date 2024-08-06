#![allow(clippy::approx_constant)]

mod modules;

#[allow(unused_imports)]
use modules::{
    a_hello_world::ab_formatted_print::formatted_display,
    b_primitives::{
        ba_literals_and_operators::a_literals_and_operators,
        bb_tuples::bb_tuples,
        bc_arrays_and_slices::bc_arrays_and_slices,
    },
};

fn main() {
    // formatted_display();
    // a_literals_and_operators()
    // bb_tuples()
    bc_arrays_and_slices()
}
