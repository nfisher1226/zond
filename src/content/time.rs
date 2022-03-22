use {
    chrono::{
        ParseError,
        prelude::*,
    },
    serde::{ Deserialize, Serialize },
    std::fmt,
};

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Time {
    pub year: i32,
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
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }
}

impl Time {
    pub fn now() -> Self {
        let utc = Utc::now();
        Self {
            year: utc.date().year(),
            month: utc.date().month(),
            day: utc.date().day(),
            hour: utc.time().hour(),
            minute: utc.time().minute(),
            second: utc.time().second(),
        }
    }

    fn to_rfc_3339(&self) -> String {
        format!(
            "{}-{}-{}T{}:{}:{}Z",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }

    pub fn to_date_time(&self) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_rfc3339(&self.to_rfc_3339())
    }

    pub fn timestamp(&self) -> Result<i64, ParseError> {
        Ok(self.to_date_time()?.timestamp())
    }
}
