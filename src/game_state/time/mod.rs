use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct GameTime {
    time: i128,
}

pub const MILLISECONDS_PER_SECOND: i128 = 1000;
pub const SECONDS_PER_MINUTE: i128 = 60;
pub const MINUTES_PER_HOUR: i128 = 60;
pub const HOURS_PER_DAY: i128 = 24;
pub const DAYS_PER_WEEK: i128 = 7;
pub const WEEKS_PER_MONTH: i128 = 4;
pub const DAYS_PER_MONTH: i128 = DAYS_PER_WEEK * WEEKS_PER_MONTH;
pub const MONTHS_PER_YEAR: i128 = 12;

pub const MILLISECONDS_PER_MINUTE: i128 = MILLISECONDS_PER_SECOND * SECONDS_PER_MINUTE;
pub const MILLISECONDS_PER_HOUR: i128 = MILLISECONDS_PER_MINUTE * MINUTES_PER_HOUR;
pub const MILLISECONDS_PER_DAY: i128 = MILLISECONDS_PER_HOUR * HOURS_PER_DAY;
pub const MILLISECONDS_PER_WEEK: i128 = MILLISECONDS_PER_DAY * DAYS_PER_WEEK;
pub const MILLISECONDS_PER_MONTH: i128 = MILLISECONDS_PER_WEEK * WEEKS_PER_MONTH;
pub const MILLISECONDS_PER_YEAR: i128 = MILLISECONDS_PER_MONTH * MONTHS_PER_YEAR;

#[allow(dead_code)]
impl GameTime {
    pub const fn milliseconds(&self) -> i128 {
        self.time
    }

    pub const fn seconds(&self) -> i128 {
        self.time / MILLISECONDS_PER_SECOND
    }

    pub const fn minutes(&self) -> i128 {
        self.time / MILLISECONDS_PER_MINUTE
    }

    pub const fn hours(&self) -> i128 {
        self.time / MILLISECONDS_PER_HOUR
    }

    pub const fn days(&self) -> i128 {
        self.time / MILLISECONDS_PER_DAY
    }

    pub const fn weeks(&self) -> i128 {
        self.time / MILLISECONDS_PER_WEEK
    }

    pub const fn months(&self) -> i128 {
        self.time / MILLISECONDS_PER_MONTH
    }

    pub const fn years(&self) -> i128 {
        self.time / MILLISECONDS_PER_YEAR
    }

    pub const fn from_milliseconds(milliseconds: i128) -> Self {
        Self { time: milliseconds }
    }

    pub const fn from_seconds(seconds: i128) -> Self {
        Self {
            time: seconds * MILLISECONDS_PER_SECOND,
        }
    }

    pub const fn from_minutes(minutes: i128) -> Self {
        Self {
            time: minutes * MILLISECONDS_PER_MINUTE,
        }
    }

    pub const fn from_hours(hours: i128) -> Self {
        Self {
            time: hours * MILLISECONDS_PER_HOUR,
        }
    }

    pub const fn from_days(days: i128) -> Self {
        Self {
            time: days * MILLISECONDS_PER_DAY,
        }
    }

    pub const fn from_weeks(weeks: i128) -> Self {
        Self {
            time: weeks * MILLISECONDS_PER_WEEK,
        }
    }

    pub const fn from_months(months: i128) -> Self {
        Self {
            time: months * MILLISECONDS_PER_MONTH,
        }
    }

    pub const fn from_years(years: i128) -> Self {
        Self {
            time: years * MILLISECONDS_PER_YEAR,
        }
    }

    pub const fn millisecond_of_second(&self) -> i16 {
        (self.milliseconds() % MILLISECONDS_PER_SECOND) as i16
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

    pub const fn day_of_month(&self) -> i8 {
        (self.days() % DAYS_PER_MONTH) as i8
    }

    pub const fn week_of_month(&self) -> i8 {
        (self.weeks() % WEEKS_PER_MONTH) as i8
    }

    pub const fn month_of_year(&self) -> i8 {
        (self.months() % MONTHS_PER_YEAR) as i8
    }

    pub const fn day_of_week_ord(&self) -> i8 {
        self.day_of_week() + 1
    }

    pub const fn month_of_year_ord(&self) -> i8 {
        self.month_of_year() + 1
    }

    pub const fn day_of_week_str(&self) -> &'static str {
        match self.day_of_week() {
            0 => "mandas",
            1 => "tirdas",
            2 => "kemdas",
            3 => "tordas",
            4 => "perdas",
            5 => "landas",
            6 => "sondas",
            _ => unreachable!(),
        }
    }

    pub const fn month_of_year_str_old(&self) -> &'static str {
        match self.month_of_year() {
            0 => "ismon",   // ice month
            1 => "tiinmon", // thaw month
            2 => "saadmon", // seed month
            3 => "renmon",  // rain month
            4 => "blomon",  // flower month
            5 => "lirtmon", // light month
            6 => "tysmon",  // calm month
            7 => "skormon", // harvest month
            8 => "hostmon", // fall month
            9 => "mutmon",  // mud month
            10 => "murmon", // dark month
            11 => "jolmon", // winter solstice month
            _ => unreachable!(),
        }
    }

    pub const fn month_of_year_str_common(&self) -> &'static str {
        match self.month_of_year() {
            0 => "white earth",    // ice month
            1 => "sun's hope",     // thaw month
            2 => "first seed",     // seed month
            3 => "cloud break",    // rain month
            4 => "flowery fields", // flower month
            5 => "eternal light",  // light month
            6 => "calm dreams",    // calm month
            7 => "harvest",        // harvest month
            8 => "leaves' fall",   // fall month
            9 => "wet mud",        // mud month
            10 => "dark skies",    // dark month
            11 => "frost fire",    // winter solstice month
            _ => unreachable!(),
        }
    }

    pub const fn floor_day(&self) -> Self {
        Self {
            time: self.days() * MILLISECONDS_PER_DAY,
        }
    }

    pub const fn ceil_day(&self) -> Self {
        Self {
            time: ((self.time - 1) / MILLISECONDS_PER_DAY + 1) * MILLISECONDS_PER_DAY,
        }
    }

    /// Modulo the length of a day, return the time as a clock would show it.
    pub const fn time_of_day(&self) -> Self {
        Self {
            time: self.time % MILLISECONDS_PER_DAY,
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

impl ops::Mul<i64> for GameTime {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            time: self.time * i128::from(rhs),
        }
    }
}

impl ops::Mul<GameTime> for i64 {
    type Output = GameTime;

    fn mul(self, rhs: GameTime) -> Self::Output {
        GameTime {
            time: rhs.time * i128::from(self),
        }
    }
}
