use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Percent{
    pub name: String,
    pub value: f64,
    tests: i64,
    count: i64,
    pub success: f64,
}

impl fmt::Debug for Percent {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Percent")
            .field("name", &self.name)
            .field("value", &self.value)
            .field("success", &self.success)
            .finish()
    }
}

impl Percent{
    pub fn new(name: String, value: f64, tests: i64) -> Percent{
        Percent{
            name: name,
            value: value,
            tests: tests,
            count: 0,
            success: 0.00,
        }
    }

    pub fn update(&mut self){
        self.count += 1;
        self.success = (self.count as f64 / self.tests as f64) * 100.00;
    }
}
