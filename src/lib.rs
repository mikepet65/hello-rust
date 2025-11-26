#![allow(dead_code)]
#![allow(unused_variables)]

mod common_collections;
mod control_flow;
mod enumerations;
mod error_handling;
mod generics;
mod guessing_game;
mod iterators_and_closures;
mod ownership;
mod statements_and_expressions;
mod structs;
mod unit_tests;
mod variables_and_datatypes;

use crate::iterators_and_closures as iter_clos;
use crate::statements_and_expressions as stmt_expr;
use crate::variables_and_datatypes as var_types;

pub fn try_variables_and_datatypes() {
    var_types::variables();
    var_types::data_types();
    var_types::print_labeled_measurement(5, 'h');
}

pub fn try_statements_and_expressions() {
    stmt_expr::statements_and_expressions();
    let x = stmt_expr::five();
    let y = stmt_expr::plus_one(1);
}

pub fn try_control_flow() {
    control_flow::control_flow();
}

pub fn try_ownership() {
    ownership::ownership();
}

pub fn try_structs() {
    structs::structs();
}

pub fn try_enumerations() {
    enumerations::enumerations();
    enumerations::options();
}

pub fn try_error_handling() {
    error_handling::open_or_create_file();
    error_handling::open_file();
    error_handling::open_file_or_panic();

    let b = error_handling::write_to_file("content").unwrap();
    let c = error_handling::long_read_from_file().unwrap();
    let d = error_handling::short_read_from_file().unwrap();
    let e = error_handling::shortest_read_from_file().unwrap();
    let f = error_handling::read_from_file().unwrap();
    let g = error_handling::delete_the_file().unwrap();
}

pub fn try_generics() {
    generics::generics();
    generics::traits();
    generics::lifetimes();
}

pub fn try_iterators_and_closures() {
    iter_clos::closures();
    iter_clos::iterators();
}

pub fn try_common_collections() {
    common_collections::collections();
}

pub fn try_unit_tests() {
    let greeting = unit_tests::greeting("Michael");
    let guess = unit_tests::guess_number(5);

    match unit_tests::guess_text("Guess!") {
        Ok(result) => println!("Guess result = {result}"),
        Err(error) => println!("Guess error = {error}"),
    }
}

pub fn try_guessing_game() {
    guessing_game::guessing_game();
}
