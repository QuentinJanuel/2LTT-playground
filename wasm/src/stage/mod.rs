#[derive(Clone, PartialEq)]
pub enum Stage { S0, S1 }

impl Stage {
    pub fn get(&self) -> usize {
        match self {
            Self::S0 => 0,
            Self::S1 => 1,
        }
    }
}
