use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Double(f32),
}

pub struct ValuePool {
    values: Vec<Value>,
}

impl ValuePool {
    pub fn new() -> ValuePool {
        ValuePool { values: Vec::new() }
    }

    pub fn get(&self, idx: u16) -> &Value {
        self.values
            .get(usize::from(idx))
            .unwrap_or_else(|| panic!("constants pool does not contain constant of idx {}", idx))
    }

    pub fn add(&mut self, value: Value) -> u16 {
        self.values.push(value);
        let usize_idx = self.values.len() - 1;
        usize_idx
            .try_into()
            .unwrap_or_else(|_| panic!("exceeded constants pool limit of {}", usize_idx))
    }
}
