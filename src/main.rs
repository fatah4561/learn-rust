#![allow(clippy::approx_constant)]

mod modules;

use modules::h_flow_of_control::hd_for_and_range;
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
    },
    d_variable_bindings::{
        da_variable_bindings::da_variable_bindings,
        db_mutability::db_mutability,
        dc_scope_and_shadowing::dc_scope_and_shadowing,
        dd_declare_first::dd_declare_first,
        de_freezing::de_freezing,
    },
    e_types::{
        ea_casting::ea_casting,
        eb_literals::eb_literals,
        ec_inference::ec_inference,
        ed_aliasing::ed_aliasing,
    },
    f_conversions::{
        fa_from_and_into::fa_from_and_into,
        fb_tryfrom_and_tryinto::fb_tryfrom_and_tryinto,
        fc_to_and_from_strings::fc_to_and_from_strings,
    },
    g_expressions::g_expressions::g_expressions,
    h_flow_of_control::{
        ha_if_else::ha_if_else,
        hb_loop::hb_loop,
        hba_nesting_and_labels::hba_nesting_and_labels,
        hbb_returning_from_loops::hbb_returning_from_loops,
        hc_while::hc_while,
        hd_for_and_range::{
            for_and_range,
            for_and_iterators,
        }
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
    // cc_constants()
    // da_variable_bindings()
    // db_mutability()
    // dc_scope_and_shadowing()
    // dd_declare_first()
    // de_freezing()
    // ea_casting()
    // eb_literals()
    // ec_inference()
    // ed_aliasing()
    // fa_from_and_into()
    // fb_tryfrom_and_tryinto()
    // fc_to_and_from_strings()
    // g_expressions()
    // ha_if_else()
    // hb_loop()
    // hba_nesting_and_labels()
    // hbb_returning_from_loops()
    // hc_while()
    // for_and_range()
    for_and_iterators()
}
