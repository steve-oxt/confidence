pub mod results;
pub mod percent;
use results::Results;
use percent::Percent;
use chrono::{Datelike, Local, NaiveDate, DateTime};
use rand::random;
use std::mem::swap;

pub struct Confidence {
    rate: f64,
    start: f64,
    previous: f64,
    current: f64,
    // end: f64,
    interval: f64,
    tests: i32,
    percents: Vec<Percent>,
    pub results: Results,
}

impl Confidence {
    pub fn new(ticks: f64, seconds: f64, time: i64, end_time: i64, rate: f64, start: f64, previous: f64, interval: f64, end: f64) -> Confidence {
        let ticks: f64 = ticks;
        let seconds: f64 = seconds;
        let t: i64 = {
            if time == 0 {
                Local::now().timestamp()
            } else {
                time
            }
        };
        let et: i64 = {
            if end_time == 0 {
                NaiveDate::from_ymd_opt(
                    DateTime::from_timestamp(t, 0)
                        .unwrap()
                        .year(),
                    DateTime::from_timestamp(t, 0)
                        .unwrap()
                        .month(),
                    DateTime::from_timestamp(t, 0)
                        .unwrap()
                        .day(),
                )
                .unwrap()
                .and_hms_opt(20, 00, 00)
                .unwrap()
                .and_utc()
                .timestamp()
            } else {
                end_time
            }
        };

        Confidence {
            rate: rate,
            start: start,
            previous: previous,
            current: start,
            interval: interval,
            //end: end,
            tests: ((et as f64 - t as f64) / seconds * ticks) as i32,
            percents: default_percents(1000),
            results: Results::new(time, end),
        }
    }

    pub fn calculate(&mut self) {
        for _ in 0..1000 {
            for _ in 0..self.tests {
                let y = random::<f64>();
                if y > self.rate {
                    if self.current > self.previous {
                        let new_value: f64 = self.current + self.interval;
                        self.previous = self.current;
                        self.current = new_value;
                    } else if self.current < self.previous {
                        let new_value: f64 = self.current - self.interval;
                        self.previous = self.current;
                        self.current = new_value;
                    }
                } else {
                    swap(&mut self.current, &mut self.previous);
                }
            }
            let change = ((self.current - self.start).abs()) / self.start;
            for percent in 0..self.percents.len() {
                if change > self.percents[percent].value {
                    self.percents[percent].update();
                }
            }
            self.current = self.start;
            self.previous = self.start - self.interval;
        }
        self.results.percents = self.percents.clone();
    }

    
}

fn default_percents(tests: i64) -> Vec<Percent> {
    let mut deafults: Vec<Percent> = Vec::new();
    deafults.push(Percent::new(String::from("one_percent"), 0.01, tests));
    deafults.push(Percent::new(String::from("two_percent"), 0.025, tests));
    deafults.push(Percent::new(String::from("five_percent"), 0.05, tests));
    deafults.push(Percent::new(String::from("ten_percent"), 0.10, tests));
    deafults.push(Percent::new(String::from("one_hundred_percent"), 1.00, tests));
    deafults.push(Percent::new(String::from("two_hundred_percent"), 2.00, tests));
    deafults.push(Percent::new(String::from("four_hundred_percent"), 4.00, tests));
    deafults
}