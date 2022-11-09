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
