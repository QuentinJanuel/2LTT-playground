use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};
use crate::stage::Stage;
use crate::val::Val0;
use crate::env::Env;


lalrpop_mod!(pub grammar, "/term/grammar.rs");

#[derive(Clone)]
pub enum Term {
    // x
    Var(String),
    // (x: A) => y
    Abs(String, Box<Self>, Box<Self>),
    // f x
    App(Box<Self>, Box<Self>),
    // Universe at a given stage
    U(Stage),
    // (x: A) -> B
    Prod(String, Box<Self>, Box<Self>),
    // let a: A = b in c at a given stage
    Let(Stage, String, Box<Self>, Box<Self>, Box<Self>),
    // ^A
    Lift(Box<Self>),
    // <t>
    Quote(Box<Self>),
    // ~t
    Splice(Box<Self>),
    // Nat, Zero and Succ at given stages
    Nat(Stage),
    Zero(Stage),
    Succ(Stage),
    NatElim(Stage),
}

impl Term {
    pub fn stage(&self) -> Self {
        Val0::eval(&Env::new(), self).quote()
    }
    fn take(&mut self) -> Box<Self> {
        Box::new(std::mem::take(self))
    }
    fn splice(&mut self) {
        *self = Self::Splice(self.take());
    }
    fn quote(&mut self) {
        *self = Self::Quote(self.take());
    }
    fn lift(&mut self) {
        *self = Self::Lift(self.take());
    }
    fn fix(&mut self, target: &Stage, as_type: bool) {
        match (target, as_type) {
            (Stage::S1, false) => self.quote(),
            (Stage::S1, true) => self.lift(),
            (Stage::S0, _) => self.splice(),
        }
    }
    fn elab_with(&mut self, env: &Env<Stage>, target: &Stage, as_type: bool) {
        match self {
            Self::Var(x) => {
                let s = env.get(x)
                    .expect(&format!("Unknown variable {x}"));
                if target != s { self.fix(target, as_type) }
            }
            Self::Abs(name, ty, body) => {
                ty.elab_with(env, target, true);
                let env = env.insert(name, target.clone());
                body.elab_with(&env, target, false);
            }
            Self::App(f, x) => {
                f.elab_with(env, target, false);
                x.elab_with(env, target, false);
            }
            Self::Prod(name, a, b) => {
                a.elab_with(env, target, true);
                let env = env.insert(name, target.clone());
                b.elab_with(&env, target, true);
            }
            Self::Let(s, name, ty, b, c) => {
                if target == s {
                    b.elab_with(env, target, false);
                    ty.elab_with(env, target, true);
                    let env = env.insert(name, target.clone());
                    c.elab_with(&env, target, false);
                } else {
                    let s = s.clone();
                    self.elab_with(env, &s, false);
                    self.fix(target, false);
                }
            }
            Self::Lift(t) => t.elab_with(env, &Stage::S0, true),
            Self::Quote(t) => t.elab_with(env, &Stage::S0, false),
            Self::Splice(t) => t.elab_with(env, &Stage::S1, false),
            Self::U(s)
            | Self::Nat(s)
            | Self::Zero(s)
            | Self::Succ(s)
            | Self::NatElim(s) => {
                if target != s { self.fix(target, as_type) }
            }
        }
    }
    pub fn elab(mut self) -> Self {
        self.elab_with(&Env::new(), &Stage::S0, false);
        self
    }
    fn get_nat(&self) -> Option<u32> {
        match self {
            Term::App(a, b) => {
                match a.as_ref() {
                    Term::Succ(_) => b.get_nat().map(|b| b + 1),
                    _ => None
                }
            }
            Term::Zero(_) => Some(0),
            _ => None,
        }
    }
    fn get_nat_str(&self, s: &Stage) -> Option<String> {
        self.get_nat().map(|n| {
            format!("{}{}", n, s.get_str())
        })
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        grammar::TermParser::new().parse(s).unwrap()
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Var(x) => write!(f, "{}", x),
            Self::Abs(n, ty, body) => write!(f, "(({n}: {ty}) => {body})"),
            Self::App(a, b) => {
                match a.as_ref() {
                    Term::Succ(s) => write!(f, "{}", self.get_nat_str(s).unwrap_or(format!("{a} {b}"))),
                    _ => write!(f, "{a} {b}"),
                }
            },
            Self::U(s) => write!(f, "U{}", s.get_str()),
            Self::Prod(n, a, b) => write!(f, "({n}: {a}) -> {b}"),
            Self::Let(s, n, ty, a, b) => write!(
                f,
                "{} {n}: {ty} = {a} in {b}",
                if s == &Stage::S0 { "let" } else { "const" },
            ),
            Self::Lift(t) => write!(f, "⇑{t}"),
            Self::Quote(t) => write!(f, "<{t}>"),
            Self::Splice(t) => write!(f, "~({t})"),
            Self::Nat(s) => write!(f, "Nat{}", s.get_str()),
            Self::Zero(s) => write!(f, "{}", self.get_nat_str(s).unwrap()),
            Self::Succ(s) => write!(f, "succ{}", s.get_str()),
            Self::NatElim(s) => write!(f, "nat_elim{}", s.get_str()),
        }
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::U(Stage::S0)
    }
}
