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
    c_custom_types::{
        ca_structures::ca_structures,
        cb_enums::cb_enums,
        cba_use::cba_use,
        cbb_c_like::cbb_c_like,
        cbc_testcase_linked_list::cbc_testcase_linked_list,
        cc_constants::cc_constants,
    }
};

fn main() {
    // formatted_display();
    // a_literals_and_operators()
    // bb_tuples()
    // bc_arrays_and_slices()
    // ca_structures()
    // cb_enums()
    // cba_use()
    // cbb_c_like()
    // cbc_testcase_linked_list()
    cc_constants()
}
