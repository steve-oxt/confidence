pub mod percent;
pub mod results;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use percent::Percent;
use rand::random;
use results::Results;
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
    pub fn new(
        ticks: f64,
        seconds: f64,
        time: i64,
        end_time: i64,
        rate: f64,
        start: f64,
        previous: f64,
        interval: f64,
        end: f64,
        hour_test: bool,
    ) -> Confidence {
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
                    DateTime::from_timestamp(t, 0).unwrap().year(),
                    DateTime::from_timestamp(t, 0).unwrap().month(),
                    DateTime::from_timestamp(t, 0).unwrap().day(),
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
        let tests = if hour_test {
            (3600.00 / seconds * ticks) as i32
        } else {
            ((et as f64 - t as f64) / seconds * ticks) as i32
        };

        Confidence {
            rate: rate,
            start: start,
            previous: previous,
            current: start,
            interval: interval,
            tests: tests,
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
                        let new_value: f64 = if (self.current - self.interval) < 0.00 {
                            0.00
                        } else {
                            self.current - self.interval
                        };
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
    deafults.push(Percent::new(
        String::from("one_hundred_percent"),
        1.00,
        tests,
    ));
    deafults.push(Percent::new(
        String::from("two_hundred_percent"),
        2.00,
        tests,
    ));
    deafults.push(Percent::new(
        String::from("four_hundred_percent"),
        4.00,
        tests,
    ));
    deafults
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_new() {
        let ticks = 2.0;
        let seconds = 60.0;
        let time = 0; // Use current time
        let end_time = 0; // Use default end time
        let rate = 0.5;
        let start = 100.0;
        let previous = 90.0;
        let interval = 10.0;
        let end = 200.0;
        let hour_test = false; // Test for the whole day, not just an hour
        let current_time = Local::now().timestamp();

        let confidence = Confidence::new(
            ticks, seconds, time, end_time, rate, start, previous, interval, end, hour_test,
        );

        assert_eq!(confidence.rate, rate);
        assert_eq!(confidence.start, start);
        assert_eq!(confidence.previous, previous);
        assert_eq!(confidence.current, start);
        assert_eq!(confidence.interval, interval);

        let calculated_tests = ((NaiveDate::from_ymd_opt(
            DateTime::from_timestamp(current_time, 0).unwrap().year(),
            DateTime::from_timestamp(current_time, 0).unwrap().month(),
            DateTime::from_timestamp(current_time, 0).unwrap().day(),
        )
        .unwrap()
        .and_hms_opt(20, 00, 00)
        .unwrap()
        .and_utc()
        .timestamp() as f64
            - current_time as f64)
            / seconds
            * ticks) as i32;

        assert_eq!(confidence.tests, calculated_tests);
    }

    #[test]
    fn test_confidence_new_hour_test() {
        let ticks = 2.0;
        let seconds = 60.0;
        let time = 0; // Use current time
        let end_time = 0; // Use default end time
        let rate = 0.5;
        let start = 100.0;
        let previous = 90.0;
        let interval = 10.0;
        let end = 200.0;
        let hour_test = true;

        let confidence = Confidence::new(
            ticks, seconds, time, end_time, rate, start, previous, interval, end, hour_test,
        );

        assert_eq!(confidence.tests, (3600.0 / seconds * ticks) as i32);
    }

    #[test]
    fn test_confidence_calculate() {
        let ticks = 1.0;
        let seconds = 1.0;
        let time = 0;
        let end_time = 0;
        let rate = 1.0; // Always succeed in the random test
        let start = 100.0;
        let previous = 90.0;
        let interval = 10.0;
        let end = 0.0;
        let hour_test = true;

        let mut confidence = Confidence::new(
            ticks, seconds, time, end_time, rate, start, previous, interval, end, hour_test,
        );
        confidence.calculate();

        assert!(confidence.results.percents[0].success >= 0.0); // one_percent
    }
}
