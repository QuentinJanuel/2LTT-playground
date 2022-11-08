#[macro_use] extern crate lalrpop_util;

mod term;
mod stage;
mod val;
mod env;
mod utils;

use term::Term;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn elab(input: &str) -> String {
    let t = Term::from(input);
    let t = t.elab();
    format!("{t}")
}

#[wasm_bindgen]
pub fn stage(input: &str) -> String {
    let t = Term::from(input);
    let t = t.elab();
    let t = t.stage();
    format!("{t}")
}
// let input = "
//     let a: Nat0 =
//         comp_let id: (_: Nat0) -> Nat0 = (x: Nat0) => x in
//         comp_let b: Nat0 = zero0 in
//         id b
//     in
//     a
// ";
// let input = "
//     comp_let a: Nat1 = zero1 in
//     let b: Nat0 = zero0 in
//     comp_let c: Nat0 = zero0 in
//     let d: Nat0 = zero0 in
//     b
// ";
// let input = "
//     comp_let id: (A: U0) -> (_: ^A) -> ^A = (A: U0) => (x: ^A) => x in
//     id Nat0 zero0
// ";
