use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Percent{
    pub name: String,
    pub value: f64,
    tests: i64,
    count: i64,
    pub success: f64,
}

impl Percent{
    pub fn new(name: String, value: f64, tests: i64) -> Percent{
        Percent{
            name: name,
            value: value,
            tests: tests,
            count: 0,
            success: (value * 0 as f64) / tests as f64,
        }
    }

    pub fn update(&mut self){
        self.count += 1;
        self.success = (self.value * self.count as f64) / self.tests as f64;
    }
}
