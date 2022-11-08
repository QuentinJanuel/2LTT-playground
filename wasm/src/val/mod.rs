mod val0;
mod val1;

pub use val0::Val0;
pub use val1::Val1;

#[derive(Clone)]
pub enum Val {
    Val0(Val0),
    Val1(Val1),
}
