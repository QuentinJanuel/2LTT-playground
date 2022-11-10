use super::{Val, Val0};
use crate::env::Env;
use crate::term::Term;
use std::ops::Deref;
use std::rc::Rc;
use std::fmt::{
    Debug,
    Formatter,
    Result as FmtResult,
};

#[derive(Clone)]
pub enum Val1 {
    // x -> y
    Abs(Rc<dyn Fn(Self) -> Self>),
    // <x>
    Quote(Val0),
    // dummy value
    Any,
    // Zero1
    Zero,
    // Succ1 x
    Succ(Box<Self>),
}

impl Val1 {
    fn var(env: &Env<Val>, name: &str) -> Self {
        let v = env
            .get(name)
            .expect(&format!("Unknown meta variable {name}"));
        match v {
            Val::Val1(v) => v.clone(),
            _ => panic!("{} is not a meta variable", name),
        }
    }
    fn app(a: Self, b: Self) -> Self {
        match a {
            Self::Abs(a) => a(b),
            _ => panic!("Cannot apply a non function"),
        }
    }
    fn nat_elim(z: Self, s: Self, t: Self) -> Self {
        match t {
            Self::Zero => z,
            Self::Succ(t) => {
                Self::app(
                    s.clone(),
                    Self::nat_elim(z, s, t.deref().clone()),
                )
            },
            _ => panic!("Cannot eliminate a non natural number"),
        }
    }
    pub fn eval(env: &Env<Val>, term: &Term) -> Self {
        match term {
            Term::Var(name) => Self::var(env, name),
            Term::Abs(name, _, t) => {
                let env = env.clone();
                let t = t.as_ref().clone();
                let name = name.clone();
                let f = move |v: Self| Self::eval_bind(&env, &t, &name, v);
                Self::Abs(Rc::new(f))
            }
            Term::App(f, x) => Self::app(Self::eval(env, f), Self::eval(env, x)),
            Term::Prod(_, _, _) => Self::Any,
            Term::Let(_, a, _, b, c) => {
                let v = Self::eval(env, b);
                let v = Val::Val1(v);
                let env = env.insert(&a, v);
                Self::eval(&env, c)
            }
            Term::Quote(t) => Self::Quote(Val0::eval(env, t)),
            Term::Lift(_) => Self::Any,
            Term::U(_) => Self::Any,
            Term::Nat(_) => Self::Any,
            Term::Zero(_) => Self::Zero,
            Term::Succ(_) => Self::Abs(Rc::new(|v: Self| Self::Succ(Box::new(v)))),
            Term::NatElim(_) => {
                Self::Abs(Rc::new(|_: Self| {
                    Self::Abs(Rc::new(|z: Self| {
                        Self::Abs(Rc::new(move |s: Self| {
                            let z = z.clone();
                            Self::Abs(Rc::new(move |t: Self| {
                                Self::nat_elim(z.clone(), s.clone(), t)
                            }))
                        }))
                    }))
                }))
            },
            Term::Splice(_) => panic!("Cannot evaluate a splice as a meta value"),
        }
    }
    pub fn eval_bind(env: &Env<Val>, term: &Term, name: &str, v: Self) -> Self {
        let val = Val::Val1(v);
        let env = env.insert(name, val);
        Self::eval(&env, term)
    }
}

impl Debug for Val1 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Abs(_) => write!(f, "abs"),
            Self::Quote(v) => write!(f, "<{v:?}>"),
            Self::Any => write!(f, "any"),
            Self::Zero => write!(f, "zero1"),
            Self::Succ(v) => write!(f, "(succ1 {v:?})"),
        }
    }
}
