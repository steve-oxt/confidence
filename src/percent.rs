use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Percent {
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

impl Percent {
    pub fn new(name: String, value: f64, tests: i64) -> Percent {
        Percent {
            name: name,
            value: value,
            tests: tests,
            count: 0,
            success: 0.00,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
        self.success = (self.count as f64 / self.tests as f64) * 100.00;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_new() {
        let percent = Percent::new(String::from("test_percent"), 0.5, 100);
        assert_eq!(percent.name, "test_percent");
        assert_eq!(percent.value, 0.5);
        assert_eq!(percent.tests, 100);
        assert_eq!(percent.count, 0);
        assert_eq!(percent.success, 0.0);
    }

    #[test]
    fn test_percent_update() {
        let mut percent = Percent::new(String::from("test_percent"), 0.5, 100);
        percent.update();
        assert_eq!(percent.count, 1);
        assert_eq!(percent.success, 1.0);

        for _ in 0..99 {
            percent.update();
        }

        assert_eq!(percent.count, 100);
        assert_eq!(percent.success, 100.0);

        let mut percent2 = Percent::new(String::from("test_percent"), 0.5, 10);
        percent2.update();
        assert_eq!(percent2.count, 1);
        assert_eq!(percent2.success, 10.0);
    }
}
