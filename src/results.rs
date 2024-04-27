use serde::{Serialize, Deserialize};
use crate::percent::Percent;

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