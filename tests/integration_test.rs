// Cargo will compile each of the files in the `tests` directory as an individual crate
mod common;
use hello_rust::*;

#[test]
fn integration_test_1() {
    common::setup();

    try_variables_and_datatypes();
    try_statements_and_expressions();

    common::teardown();
}
