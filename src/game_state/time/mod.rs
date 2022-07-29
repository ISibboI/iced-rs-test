use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct GameTime {
    time: i128,
}

pub const SECONDS_PER_MINUTE: i128 = 60;
pub const MINUTES_PER_HOUR: i128 = 60;
pub const HOURS_PER_DAY: i128 = 24;
pub const DAYS_PER_WEEK: i128 = 7;
pub const WEEKS_PER_MONTH: i128 = 4;
pub const MONTHS_PER_YEAR: i128 = 12;

pub const SECONDS_PER_HOUR: i128 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;
pub const SECONDS_PER_DAY: i128 = SECONDS_PER_HOUR * HOURS_PER_DAY;
pub const SECONDS_PER_WEEK: i128 = SECONDS_PER_DAY * DAYS_PER_WEEK;
pub const SECONDS_PER_MONTH: i128 = SECONDS_PER_WEEK * WEEKS_PER_MONTH;
pub const SECONDS_PER_YEAR: i128 = SECONDS_PER_MONTH * MONTHS_PER_YEAR;

impl GameTime {
    pub const fn seconds(&self) -> i128 {
        self.time
    }

    pub const fn minutes(&self) -> i128 {
        self.time / SECONDS_PER_MINUTE
    }

    pub const fn hours(&self) -> i128 {
        self.time / SECONDS_PER_HOUR
    }

    pub const fn days(&self) -> i128 {
        self.time / SECONDS_PER_DAY
    }

    pub const fn weeks(&self) -> i128 {
        self.time / SECONDS_PER_WEEK
    }

    pub const fn months(&self) -> i128 {
        self.time / SECONDS_PER_MONTH
    }

    pub const fn years(&self) -> i128 {
        self.time / SECONDS_PER_YEAR
    }

    pub const fn from_seconds(seconds: i128) -> Self {
        Self { time: seconds }
    }

    pub const fn from_minutes(minutes: i128) -> Self {
        Self {
            time: minutes * SECONDS_PER_MINUTE,
        }
    }

    pub const fn from_hours(hours: i128) -> Self {
        Self {
            time: hours * SECONDS_PER_HOUR,
        }
    }

    pub const fn from_days(days: i128) -> Self {
        Self {
            time: days * SECONDS_PER_DAY,
        }
    }

    pub const fn from_weeks(weeks: i128) -> Self {
        Self {
            time: weeks * SECONDS_PER_WEEK,
        }
    }

    pub const fn from_months(months: i128) -> Self {
        Self {
            time: months * SECONDS_PER_MONTH,
        }
    }

    pub const fn from_years(years: i128) -> Self {
        Self {
            time: years * SECONDS_PER_YEAR,
        }
    }

    pub const fn second_of_minute(&self) -> i8 {
        (self.seconds() % SECONDS_PER_MINUTE) as i8
    }

    pub const fn minute_of_hour(&self) -> i8 {
        (self.minutes() % MINUTES_PER_HOUR) as i8
    }

    pub const fn hour_of_day(&self) -> i8 {
        (self.hours() % HOURS_PER_DAY) as i8
    }

    pub const fn day_of_week(&self) -> i8 {
        (self.days() % DAYS_PER_WEEK) as i8
    }

    pub const fn week_of_month(&self) -> i8 {
        (self.weeks() % WEEKS_PER_MONTH) as i8
    }

    pub const fn month_of_year(&self) -> i8 {
        (self.months() % MONTHS_PER_YEAR) as i8
    }

    pub const fn floor_day(&self) -> Self {
        Self {
            time: self.days() * SECONDS_PER_DAY,
        }
    }

    pub const fn ceil_day(&self) -> Self {
        Self {
            time: ((self.time - 1) / SECONDS_PER_DAY + 1) * SECONDS_PER_DAY,
        }
    }

    /// Modulo the length of a day, return the time as a clock would show it.
    pub const fn time_of_day(&self) -> Self {
        Self {
            time: self.time % SECONDS_PER_DAY,
        }
    }
}

impl ops::Add for GameTime {
    type Output = GameTime;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            time: self.time + rhs.time,
        }
    }
}

impl ops::AddAssign for GameTime {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for GameTime {
    type Output = GameTime;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            time: self.time - rhs.time,
        }
    }
}

impl ops::SubAssign for GameTime {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
