#[derive(Clone, PartialEq)]
pub enum Stage { S0, S1 }

impl Stage {
    // fn get(&self) -> usize {
    //     match self {
    //         Self::S0 => 0,
    //         Self::S1 => 1,
    //     }
    // }
    pub fn get_str(&self) -> &str {
        match self {
            Self::S0 => "₀",
            Self::S1 => "₁",
        }
    }
}
