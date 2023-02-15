use {
    chrono::{prelude::*, ParseError},
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
/// Conversion middleman because `chrono::DateTime` does not support serde
pub struct Time {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}T{}:{}:{}Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second,
        )
    }
}

impl Time {
    /// Saves the current time as UTC
    pub fn now() -> Self {
        let utc = Utc::now();
        Self {
            year: utc.date_naive().year(),
            month: utc.date_naive().month(),
            day: utc.date_naive().day(),
            hour: utc.time().hour(),
            minute: utc.time().minute(),
            second: utc.time().second(),
        }
    }

    /// Retreives the year stored in this struct
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns a `String` representing rfc3339 dat/time format
    fn to_rfc_3339(&self) -> String {
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second,
        )
    }

    /// Converts to chrono's `DateTime` format
    pub fn to_date_time(&self) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_rfc3339(&self.to_rfc_3339())
    }

    /// Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”).
    pub fn timestamp(&self) -> Result<i64, crate::Error> {
        Ok(self.to_date_time()?.timestamp())
    }

    /// Returns a string representing just the date protion (American format)
    pub fn date_string(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
