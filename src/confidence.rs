use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use rand::random;
use std::mem::swap;

pub struct Confidence {
    rate: f64,
    start: f64,
    previous: f64,
    current: f64,
    end: f64,
    interval: f64,
    tests: i32,
    positive: Vec<i32>,
    percentage: Vec<f64>,
    pretty_percentage: Vec<String>,
    pub results: String,
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
                    NaiveDateTime::from_timestamp_opt(t, 0)
                        .unwrap()
                        .year(),
                    NaiveDateTime::from_timestamp_opt(t, 0)
                        .unwrap()
                        .month(),
                    NaiveDateTime::from_timestamp_opt(t, 0)
                        .unwrap()
                        .day(),
                )
                .unwrap()
                .and_hms_opt(20, 00, 00)
                .unwrap()
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
            end: end,
            tests: ((et as f64 - t as f64) / seconds * ticks) as i32,
            positive: [0, 0, 0, 0, 0, 0, 0].to_vec(),
            percentage: [0.01, 0.025, 0.05, 0.10, 1.00, 2.00, 4.00].to_vec(),
            pretty_percentage: [String::from("one_percent"), String::from("two_percent"), String::from("five_percent"), String::from("ten_percent"), String::from("one_hundred_percent"), String::from("two_hundred_percent"), String::from("four_hundred_percent")].to_vec(),
            results: format!("{{\"time\":{}", time),
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
            for z in 0..7 {
                if change > self.percentage[z] {
                    self.positive[z] += 1;
                }
            }
            self.current = self.start;
            self.previous = self.start - self.interval;
        }
        for w in 0..7 {
            let success: f64 = (self.positive[w] as f64) / 1000.00 * 100.00;
            self.results +=
                format!(",\"{}\":{:.2}", self.pretty_percentage[w], success).as_str();
        }

        self.results += format!(",\"current_price\":{:.2}}}", self.end).as_str();
    }
}
