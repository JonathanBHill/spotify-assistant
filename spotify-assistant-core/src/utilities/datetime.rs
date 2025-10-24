use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, Timelike, Weekday};
use tracing::{debug, Level};

use crate::enums::validation::TimeOfDay;

/// A structure representing checks related to the current date and time.
///
/// The `Checks` struct stores information about the current date, day of the week,
/// and a specific time of day for use in applications where such checks are necessary.
///
/// # Fields
///
/// * `now` - The current date and time with timezone information.
///   It is represented as a `DateTime<Local>` to handle local time.
/// * `day_of_week` - The current day of the week.
///   This is represented using the `Weekday` enum, which provides variants for each day (e.g., Monday, Tuesday).
/// * `time_of_day` - A representation of a specific time of day.
///   This is typically used to store details like hours, minutes, or seconds
///   and is encapsulated in the `TimeOfDay` struct or equivalent.
///
/// # Usage
/// This structure can be utilized for tasks like scheduling, validations,
/// or any functionality that depends on the current date or time-related attributes in a local timezone.
pub struct Checks {
    now: DateTime<Local>,
    day_of_week: Weekday,
    time_of_day: TimeOfDay,
}

impl Default for Checks {
    fn default() -> Self {
        let now = Local::now();
        let day_of_week = now.weekday();
        let time_of_day = TimeOfDay::from_hour(now.hour());
        Checks {
            now,
            day_of_week,
            time_of_day,
        }
    }
}

impl Checks {
    /// Returns the current date as a `NaiveDate` instance.
    ///
    /// This method extracts the `date_naive` component from the `now` field,
    /// providing the current date without any associated timezone or time information.
    ///
    /// # Returns
    ///
    /// * `NaiveDate` - A date object that represents the current date.
    ///
    /// # Notes
    ///
    /// - This method assumes that the `now` field is a valid `DateTime` object
    ///   and generates a naive date without timezone considerations.
    pub fn date(&self) -> NaiveDate {
        self.now.date_naive()
    }

    /// Retrieves the time component from the current datetime.
    ///
    /// # Returns
    ///
    /// This method returns a `NaiveTime` object, representing the time portion
    /// (hours, minutes, seconds, and milliseconds) of the `DateTime` value
    /// stored within the instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::{NaiveTime, NaiveDate, NaiveDateTime};
    /// use spotify_assistant_core::utilities::datetime::Checks;
    ///
    /// let dt = NaiveDateTime::new(
    ///     NaiveDate::from_ymd_opt(2023, 10, 24).unwrap(),
    ///     NaiveTime::from_hms_opt(14, 30, 45).unwrap(),
    /// );
    /// let instance = Checks::default();
    ///
    /// let time = instance.time();
    /// assert_eq!(time, NaiveTime::from_hms_opt(14, 30, 45).unwrap());
    /// ```
    ///
    /// # Note
    ///
    /// This method assumes that the `now` field within the structure contains
    /// a valid `DateTime` value.
    pub fn time(&self) -> NaiveTime {
        self.now.time()
    }

    /// Checks if the current day is a weekend.
    ///
    /// # Returns
    ///
    /// * `true` - If the day of the week is Saturday (`Weekday::Sat`) or Sunday (`Weekday::Sun`).
    /// * `false` - For all other days of the week.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::utilities::datetime::Checks;
    ///
    /// let saturday = Checks::default();
    /// println!("{}" ,saturday.is_weekend());
    /// ```
    pub fn is_weekend(&self) -> bool {
        self.day_of_week == Weekday::Sat || self.day_of_week == Weekday::Sun
    }

    pub fn time_of_day(&self) -> TimeOfDay {
        self.time_of_day.clone()
    }

    /// Checks if a given date string is outdated based on a threshold duration.
    ///
    /// # Parameters
    /// - `input`: A string slice representing the input date to check. Expected formats are `%m-%d-%Y` or `%Y-%m-%d`.
    /// - `threshold`: A `Duration` representing the cutoff threshold. The date in `input` is compared to the current
    ///   date minus this threshold to determine if it is outdated.
    ///
    /// # Returns
    /// - `true` if the `input` date is earlier than the computed cutoff date based on the threshold.
    /// - `false` otherwise.
    ///
    /// # Behavior
    /// - The method attempts to parse the input date against predefined formats (`%m-%d-%Y`, `%Y-%m-%d`).
    /// - If no matching format is found, the first format (`%m-%d-%Y`) is used as a fallback.
    /// - The parsed `input` date is compared with the cutoff date (i.e., `self.now.date_naive() - threshold`).
    /// - Extensive debugging information is logged for tracing the computation process.
    ///
    /// # Panics
    /// - This function uses `unwrap_or_default` for parsing the date. If the fallback value of `NaiveDate` (i.e., `NaiveDate::default()`)
    ///   is not suitable for the intended logic, adjust the handling of invalid input dates.
    ///
    /// # Example
    /// ```
    /// use chrono::Duration; // Make sure to include the chrono crate
    /// use spotify_assistant_core::utilities::datetime::Checks;
    ///
    /// let checker = Checks::default();
    /// let is_outdated = checker.is_outdated("12-25-2020", Duration::days(365));
    /// assert!(is_outdated); // Assumes the current date is after 12-25-2021.
    /// ```
    pub fn is_outdated(&self, input: &str, threshold: Duration) -> bool {
        let span = tracing::span!(Level::DEBUG, "Checks.is_outdated");
        let _enter = span.enter();

        let formats = ["%m-%d-%Y", "%Y-%m-%d"];
        let correct_format = formats
            .iter()
            .find(|format| NaiveDate::parse_from_str(input, format).is_ok())
            .unwrap_or(&"%m-%d-%Y");
        let last_updated = NaiveDate::parse_from_str(input, correct_format).unwrap_or_default();
        let cutoff = self.now.date_naive() - threshold;
        debug!("Last updated: {} | Cutoff: {}", last_updated, cutoff);
        last_updated < cutoff
    }
}

#[cfg(test)]
impl Checks {
    pub(crate) fn from_datetime(now: DateTime<Local>) -> Self {
        let day_of_week = now.weekday();
        let time_of_day = TimeOfDay::from_hour(now.hour());
        Checks {
            now,
            day_of_week,
            time_of_day,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime, TimeZone};

    fn build_checks(year: i32, month: u32, day: u32, hour: u32) -> Checks {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("valid date");
        let time = NaiveTime::from_hms_opt(hour, 0, 0).expect("valid time");
        let naive = date.and_time(time);
        let now = Local.from_local_datetime(&naive).unwrap();
        Checks::from_datetime(now)
    }

    #[test]
    fn is_weekend_reports_true_for_saturday() {
        let checks = build_checks(2024, 11, 9, 9); // Saturday
        assert!(checks.is_weekend());
    }

    #[test]
    fn is_weekend_reports_false_for_weekday() {
        let checks = build_checks(2024, 11, 7, 9); // Thursday
        assert!(!checks.is_weekend());
    }

    #[test]
    fn is_outdated_respects_threshold() {
        let checks = build_checks(2024, 11, 10, 10);
        let threshold = Duration::weeks(52);
        assert!(checks.is_outdated("11-08-2023", threshold));
        assert!(checks.is_outdated("2023-11-08", threshold));
        assert!(!checks.is_outdated("11-11-2024", threshold));
    }

    #[test]
    fn time_of_day_transitions_cover_all_ranges() {
        let cases = vec![
            (0, TimeOfDay::Night),
            (6, TimeOfDay::Morning),
            (12, TimeOfDay::Afternoon),
            (18, TimeOfDay::Evening),
            (23, TimeOfDay::Evening),
            (5, TimeOfDay::Night),
            (11, TimeOfDay::Morning),
            (16, TimeOfDay::Afternoon),
        ];

        for (hour, expected) in cases {
            let checks = build_checks(2024, 11, 7, hour);
            assert_eq!(
                checks.time_of_day, expected,
                "hour {hour} should map correctly"
            );
        }
    }
}
