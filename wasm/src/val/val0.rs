use super::{Val, Val1};
use crate::env::Env;
use crate::stage::Stage;
use crate::term::Term;
use std::rc::Rc;
use std::fmt::{
    Debug,
    Formatter,
    Result as FmtResult,
};

#[derive(Clone)]
pub enum Val0 {
    Var(String),
    App(Box<Self>, Box<Self>),
    Pi(String, Box<Self>, Rc<dyn Fn(Self) -> Self>),
    Abs(String, Box<Self>, Rc<dyn Fn(Self) -> Self>),
    Let(String, Box<Self>, Box<Self>, Rc<dyn Fn(Self) -> Self>),
    U,
    Nat,
    Zero,
    Succ,
    NatElim,
}

impl Val0 {
    fn var(env: &Env<Val>, name: &str) -> Self {
        let v = env
            .get(name)
            .expect(&format!("Unknown object variable {name}"));
        match v {
            Val::Val0(v) => v.clone(),
            _ => panic!("{} is not an object variable", name),
        }
    }
    fn splice(v: Val1) -> Self {
        match v {
            Val1::Quote(v) => v,
            _ => {
                println!("Attempted to splice {v:?}");
                panic!("Cannot splice a non quote");
            },
        }
    }
    pub fn eval(env: &Env<Val>, term: &Term) -> Self {
        match term {
            Term::Var(name) => Self::var(env, name),
            Term::Abs(x, a, t) => {
                let env = env.clone();
                let a = a.as_ref().clone();
                let t = t.as_ref().clone();
                let a = Self::eval(&env, &a);
                let x2 = x.clone();
                let f = move |v: Self| Self::eval_bind(&env, &t, &x2, v);
                Self::Abs(x.clone(), Box::new(a), Rc::new(f))
            }
            Term::App(f, x) => {
                Self::App(Box::new(Self::eval(env, f)), Box::new(Self::eval(env, x)))
            }
            Term::Prod(x, a, b) => Self::Pi(x.clone(), Box::new(Self::eval(env, a)), {
                let env = env.clone();
                let b = b.as_ref().clone();
                let x = x.clone();
                Rc::new(move |v: Self| Self::eval_bind(&env, &b, &x, v))
            }),
            Term::Let(_, x, a, t, u) => Self::Let(
                x.clone(),
                Box::new(Self::eval(env, a)),
                Box::new(Self::eval(env, t)),
                {
                    let env = env.clone();
                    let u = u.as_ref().clone();
                    let x = x.clone();
                    Rc::new(move |v: Self| Self::eval_bind(&env, &u, &x, v))
                },
            ),
            Term::U(_) => Self::U,
            Term::Splice(t) => {
                let t = Val1::eval(env, t);
                Self::splice(t)
            }
            Term::Nat(_) => Self::Nat,
            Term::Zero(_) => Self::Zero,
            Term::Succ(_) => Self::Succ,
            Term::NatElim(_) => Self::NatElim,
            Term::Quote(_) => unreachable!(),
            Term::Lift(_) => unreachable!(),
        }
    }
    fn eval_bind(env: &Env<Val>, term: &Term, name: &str, v: Self) -> Self {
        let val = Val::Val0(v);
        let env = env.insert(name, val);
        Self::eval(&env, term)
    }
    pub fn quote(&self) -> Term {
        match self {
            Self::Var(x) => Term::Var(x.clone()),
            Self::App(t, u) => Term::App(Box::new(t.quote()), Box::new(u.quote())),
            Self::Pi(x, a, b) => Term::Prod(
                x.clone(),
                Box::new(a.quote()),
                Box::new(b(Self::Var(x.clone())).quote()),
            ),
            Self::Abs(x, a, t) => Term::Abs(
                x.clone(),
                Box::new(a.quote()),
                Box::new(t(Self::Var(x.clone())).quote()),
            ),
            Self::Let(x, a, t, u) => Term::Let(
                Stage::S0,
                x.clone(),
                Box::new(a.quote()),
                Box::new(t.quote()),
                Box::new(u(Self::Var(x.clone())).quote()),
            ),
            Self::U => Term::U(Stage::S0),
            Self::Nat => Term::Nat(Stage::S0),
            Self::Zero => Term::Zero(Stage::S0),
            Self::Succ => Term::Succ(Stage::S0),
            Self::NatElim => Term::NatElim(Stage::S0),
        }
    }
}

impl Debug for Val0 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Var(name) => write!(f, "{name}"),
            Self::App(a, b) => write!(f, "({a:?} {b:?})"),
            Self::Pi(name, a, _) => write!(f, "({name} : {a:?}) -> ..."),
            Self::Abs(name, a, _) => write!(f, "({name} : {a:?}) => ..."),
            Self::Let(name, a, b, _) => write!(f, "let0 {name} : {a:?} = {b:?} in ..."),
            Self::U => write!(f, "U0"),
            Self::Nat => write!(f, "Nat0"),
            Self::Zero => write!(f, "zero0"),
            Self::Succ => write!(f, "succ0"),
            Self::NatElim => write!(f, "nat_elim0"),
        }
    }
}
