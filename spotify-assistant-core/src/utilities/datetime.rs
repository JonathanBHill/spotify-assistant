use chrono::{Datelike, DateTime, Duration, Local, NaiveDate, NaiveTime, Timelike, Weekday};
use tracing::{debug, Level};

use crate::enums::validation::TimeOfDay;

pub struct Checks {
    now: DateTime<Local>,
    day_of_week: Weekday,
    time_of_day: TimeOfDay,
}

impl Checks {
    pub fn new() -> Self {
        let now = Local::now();
        let day_of_week = now.weekday();
        let time_of_day = TimeOfDay::from_hour(now.hour());
        Checks {
            now,
            day_of_week,
            time_of_day,
        }
    }
    pub fn date(&self) -> NaiveDate {
        self.now.date_naive()
    }
    pub fn time(&self) -> NaiveTime {
        self.now.time()
    }
    pub fn is_weekend(&self) -> bool {
        self.day_of_week == Weekday::Sat || self.day_of_week == Weekday::Sun
    }
    pub fn is_outdated(&self, input: &str, threshold: Duration) -> bool {
        let span = tracing::span!(Level::DEBUG, "Checks.is_outdated");
        let _enter = span.enter();

        let formats = ["%m-%d-%Y", "%Y-%m-%d"];
        let correct_format = formats.iter().find(|format| {
            NaiveDate::parse_from_str(input, format).is_ok()
        }).unwrap_or_else(|| &"%m-%d-%Y");
        let last_updated = NaiveDate::parse_from_str(input, correct_format).unwrap_or_default();
        let cutoff = self.now.date_naive() - threshold;
        debug!("Last updated: {} | Cutoff: {}", last_updated, cutoff);
        last_updated < cutoff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checks() {
        let checks = Checks::new();
        assert!(!checks.is_weekend());
        assert_eq!(checks.time_of_day, TimeOfDay::Morning);
    }

    #[test]
    fn test_is_outdated() {
        let checks = Checks::new();
        let input_outdated = "11-08-2023";
        let input_outdated_2 = "2023-11-08";
        let input_not_outdated = "11-10-2024";
        let threshold = Duration::weeks(52);
        let threshold_2 = Duration::weeks(60);
        assert!(checks.is_outdated(input_outdated, threshold));
        assert!(!checks.is_outdated(input_outdated, threshold_2));
        assert!(checks.is_outdated(input_outdated_2, threshold));
        assert!(!checks.is_outdated(input_not_outdated, threshold));
    }
}
