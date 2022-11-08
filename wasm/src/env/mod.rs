use std::collections::HashMap;

#[derive(Clone)]
pub struct Env<T> {
    vals: HashMap<String, T>,
}

impl<T: Clone> Env<T> {
    pub fn new() -> Self {
        Self { vals: HashMap::new() }
    }
    pub fn get(&self, name: &str) -> Option<&T> {
        self.vals.get(name)
    }
    pub fn insert(&self, name: &str, val: T) -> Self {
        let mut s = self.clone();
        s.vals.insert(name.to_string(), val);
        s
    }
}
