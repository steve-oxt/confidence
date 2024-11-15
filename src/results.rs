use crate::percent::Percent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Results {
    pub time: i64,
    pub percents: Vec<Percent>,
    pub price: f64,
}

impl Results {
    pub fn new(time: i64, price: f64) -> Results {
        Results {
            time: time,
            percents: Vec::new(),
            price: price,
        }
    }

    pub fn update(&mut self, time: i64, success: Vec<Percent>, price: f64) {
        self.time = time;
        for p in success {
            self.percents.push(p);
        }
        self.price = price;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_new() {
        let time = 1678886400; // Example timestamp
        let price = 123.45;
        let results = Results::new(time, price);
        assert_eq!(results.time, time);
        assert!(results.percents.is_empty());
        assert_eq!(results.price, price);
    }

    #[test]
    fn test_results_update() {
        let mut results = Results::new(0, 0.0);
        let time = 1678886400;
        let price = 678.90;
        let percents = vec![
            Percent::new(String::from("one_percent"), 0.01, 1000),
            Percent::new(String::from("five_percent"), 0.05, 1000),
        ];

        results.update(time, percents.clone(), price);

        assert_eq!(results.time, time);
        assert_eq!(results.price, price);
        assert_eq!(results.percents.len(), percents.len());

        for (i, p) in results.percents.iter().enumerate() {
            assert_eq!(p.name, percents[i].name);
            assert_eq!(p.value, percents[i].value);
        }
    }
}
