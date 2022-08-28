#![allow(dead_code)]

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
pub const DAYS_PER_MONTH: [i128; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const FIRST_DAY_OF_MONTH: [i128; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
pub const DAYS_PER_YEAR: i128 = 365;
pub const MONTHS_PER_YEAR: i128 = 12;
pub const YEARS_PER_FINISHED_ERA: [i128; 2] = [2344, 1698];
pub const FIRST_YEAR_OF_ERA: [i128; 3] = [0, 2344, 4042];

pub const MILLISECONDS_PER_MINUTE: i128 = MILLISECONDS_PER_SECOND * SECONDS_PER_MINUTE;
pub const MILLISECONDS_PER_HOUR: i128 = MILLISECONDS_PER_MINUTE * MINUTES_PER_HOUR;
pub const MILLISECONDS_PER_DAY: i128 = MILLISECONDS_PER_HOUR * HOURS_PER_DAY;
pub const MILLISECONDS_PER_WEEK: i128 = MILLISECONDS_PER_DAY * DAYS_PER_WEEK;
pub const MILLISECONDS_PER_MONTH: [i128; 12] = [
    DAYS_PER_MONTH[0] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[1] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[2] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[3] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[4] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[5] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[6] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[7] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[8] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[9] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[10] * MILLISECONDS_PER_DAY,
    DAYS_PER_MONTH[11] * MILLISECONDS_PER_DAY,
];
pub const MILLISECONDS_PER_YEAR: i128 = MILLISECONDS_PER_DAY * DAYS_PER_YEAR;

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

    pub const fn years(&self) -> i128 {
        self.time / MILLISECONDS_PER_YEAR
    }

    pub const fn eras(&self) -> i8 {
        let years = self.years();
        let mut era = FIRST_YEAR_OF_ERA.len();
        loop {
            era -= 1;
            assert!(era < FIRST_YEAR_OF_ERA.len());
            if years >= FIRST_YEAR_OF_ERA[era] {
                return era as i8;
            }
        }
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

    pub const fn from_years(years: i128) -> Self {
        Self {
            time: years * MILLISECONDS_PER_YEAR,
        }
    }

    pub const fn from_eras(eras: i128) -> Self {
        Self {
            time: FIRST_YEAR_OF_ERA[eras as usize] * MILLISECONDS_PER_YEAR,
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
        (self.days() - self.floor_month().days()) as i8
    }

    pub const fn day_of_year(&self) -> i16 {
        (self.days() - self.floor_year().days()) as i16
    }

    pub const fn month_of_year(&self) -> i8 {
        let days = self.day_of_year();
        let mut month = FIRST_DAY_OF_MONTH.len();
        loop {
            month -= 1;
            assert!(month < FIRST_DAY_OF_MONTH.len());
            if days as i128 >= FIRST_DAY_OF_MONTH[month] {
                return month as i8;
            }
        }
    }

    pub const fn year_of_era(&self) -> i16 {
        (self.years() - self.floor_era().years()) as i16
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

    pub const fn floor_month(&self) -> Self {
        assert!(self.time >= 0);
        Self {
            time: self.years() * MILLISECONDS_PER_YEAR
                + FIRST_DAY_OF_MONTH[self.month_of_year() as usize] * MILLISECONDS_PER_DAY,
        }
    }

    pub const fn floor_year(&self) -> Self {
        Self {
            time: self.years() * MILLISECONDS_PER_YEAR,
        }
    }

    pub const fn floor_era(&self) -> Self {
        assert!(self.time >= 0);
        Self {
            time: FIRST_YEAR_OF_ERA[self.eras() as usize] * MILLISECONDS_PER_YEAR,
        }
    }

    /// Modulo the length of a day, return the time as a clock would show it.
    pub const fn time_of_day(&self) -> Self {
        Self {
            time: self.time % MILLISECONDS_PER_DAY,
        }
    }

    pub const fn day_of_week_ord(&self) -> i8 {
        self.day_of_week() + 1
    }

    pub const fn day_of_month_ord(&self) -> i8 {
        self.day_of_month() + 1
    }

    pub const fn month_of_year_ord(&self) -> i8 {
        self.month_of_year() + 1
    }

    pub const fn era_ord(&self) -> i8 {
        self.eras() + 1
    }

    pub const fn day_of_week_str(&self) -> &'static str {
        match self.day_of_week_ord() {
            1 => "Mandas",
            2 => "Tirdas",
            3 => "Kemdas",
            4 => "Tordas",
            5 => "Perdas",
            6 => "Landas",
            7 => "Sondas",
            _ => unreachable!(),
        }
    }

    pub const fn month_of_year_str_old(&self) -> &'static str {
        match self.month_of_year_ord() {
            1 => "Ismon",   // ice month
            2 => "Tiinmon", // thaw month
            3 => "Saadmon", // seed month
            4 => "Renmon",  // rain month
            5 => "Blomon",  // flower month
            6 => "Lirtmon", // light month
            7 => "Tysmon",  // calm month
            8 => "Skormon", // harvest month
            9 => "Hostmon", // fall month
            10 => "Mutmon", // mud month
            11 => "Murmon", // dark month
            12 => "Jolmon", // winter solstice month
            _ => unreachable!(),
        }
    }

    pub const fn day_of_month_str_ord(&self) -> &'static str {
        match self.day_of_month_ord() {
            1 => "1st",
            2 => "2nd",
            3 => "3rd",
            4 => "4th",
            5 => "5th",
            6 => "6th",
            7 => "7th",
            8 => "8th",
            9 => "9th",
            10 => "10th",
            11 => "11th",
            12 => "12th",
            13 => "13th",
            14 => "14th",
            15 => "15th",
            16 => "16th",
            17 => "17th",
            18 => "18th",
            19 => "19th",
            20 => "20th",
            21 => "21st",
            22 => "22nd",
            23 => "23rd",
            24 => "24th",
            25 => "25th",
            26 => "26th",
            27 => "27th",
            28 => "28th",
            29 => "29th",
            30 => "30th",
            31 => "31st",
            _ => unreachable!(),
        }
    }

    pub const fn month_of_year_str_common(&self) -> &'static str {
        match self.month_of_year_ord() {
            1 => "White Earth",    // ice month
            2 => "Sun's Hope",     // thaw month
            3 => "First Seed",     // seed month
            4 => "Cloud Break",    // rain month
            5 => "Flowery Fields", // flower month
            6 => "Eternal Light",  // light month
            7 => "Calm Dreams",    // calm month
            8 => "Harvest",        // harvest month
            9 => "Leaves' Fall",   // fall month
            10 => "Wet Mud",       // mud month
            11 => "Dark Skies",    // dark month
            12 => "Frost Fire",    // winter solstice month
            _ => unreachable!(),
        }
    }

    pub const fn era_str(&self) -> &'static str {
        match self.era_ord() {
            1 => "1st",
            2 => "2nd",
            3 => "3rd",
            4 => "4th",
            5 => "5th",
            6 => "6th",
            7 => "7th",
            8 => "8th",
            _ => unreachable!(),
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

#[cfg(test)]
mod tests {
    use crate::game_state::time::{
        GameTime, DAYS_PER_MONTH, FIRST_DAY_OF_MONTH, FIRST_YEAR_OF_ERA, YEARS_PER_FINISHED_ERA,
    };

    #[test]
    fn test_first_days_of_months() {
        let first_day_of_month: Vec<_> = DAYS_PER_MONTH
            .iter()
            .scan(0, |acc, &days_per_month| {
                let result = *acc;
                *acc += days_per_month;
                Some(result)
            })
            .collect();
        assert_eq!(first_day_of_month.as_slice(), FIRST_DAY_OF_MONTH);
    }

    #[test]
    fn test_month_of_year() {
        assert_eq!(GameTime::from_days(30).month_of_year(), 0);
        assert_eq!(
            (GameTime::from_days(31) - GameTime::from_milliseconds(1)).month_of_year(),
            0
        );
        assert_eq!(GameTime::from_days(31).month_of_year(), 1);
        assert_eq!(GameTime::from_days(32).month_of_year(), 1);

        assert_eq!(
            (GameTime::from_days(30) + GameTime::from_years(2)).month_of_year(),
            0
        );
        assert_eq!(
            (GameTime::from_days(31) - GameTime::from_milliseconds(1) + GameTime::from_years(4))
                .month_of_year(),
            0
        );
        assert_eq!(
            (GameTime::from_days(31) + GameTime::from_years(5)).month_of_year(),
            1
        );
        assert_eq!(
            (GameTime::from_days(32) + GameTime::from_years(6)).month_of_year(),
            1
        );

        assert_eq!(
            (GameTime::from_days(89) + GameTime::from_years(2)).month_of_year(),
            2
        );
        assert_eq!(
            (GameTime::from_days(90) - GameTime::from_milliseconds(1) + GameTime::from_years(4))
                .month_of_year(),
            2
        );
        assert_eq!(
            (GameTime::from_days(90) + GameTime::from_years(5)).month_of_year(),
            3
        );
        assert_eq!(
            (GameTime::from_days(91) + GameTime::from_years(6)).month_of_year(),
            3
        );
    }

    #[test]
    fn test_floor_month() {
        assert_eq!(
            GameTime::from_days(30).floor_month(),
            GameTime::from_days(0)
        );
        assert_eq!(
            (GameTime::from_days(31) - GameTime::from_milliseconds(1)).floor_month(),
            GameTime::from_days(0)
        );
        assert_eq!(
            GameTime::from_days(31).floor_month(),
            GameTime::from_days(31)
        );
        assert_eq!(
            GameTime::from_days(32).floor_month(),
            GameTime::from_days(31)
        );

        assert_eq!(
            (GameTime::from_days(30) + GameTime::from_years(2)).floor_month(),
            GameTime::from_days(0) + GameTime::from_years(2)
        );
        assert_eq!(
            (GameTime::from_days(31) - GameTime::from_milliseconds(1) + GameTime::from_years(4))
                .floor_month(),
            GameTime::from_days(0) + GameTime::from_years(4)
        );
        assert_eq!(
            (GameTime::from_days(31) + GameTime::from_years(5)).floor_month(),
            GameTime::from_days(31) + GameTime::from_years(5)
        );
        assert_eq!(
            (GameTime::from_days(32) + GameTime::from_years(6)).floor_month(),
            GameTime::from_days(31) + GameTime::from_years(6)
        );

        assert_eq!(
            (GameTime::from_days(89) + GameTime::from_years(2)).floor_month(),
            GameTime::from_days(59) + GameTime::from_years(2)
        );
        assert_eq!(
            (GameTime::from_days(90) - GameTime::from_milliseconds(1) + GameTime::from_years(4))
                .floor_month(),
            GameTime::from_days(59) + GameTime::from_years(4)
        );
        assert_eq!(
            (GameTime::from_days(90) + GameTime::from_years(5)).floor_month(),
            GameTime::from_days(90) + GameTime::from_years(5)
        );
        assert_eq!(
            (GameTime::from_days(91) + GameTime::from_years(6)).floor_month(),
            GameTime::from_days(90) + GameTime::from_years(6)
        );
    }

    #[test]
    fn test_first_years_of_eras() {
        let first_year_of_era: Vec<_> = YEARS_PER_FINISHED_ERA
            .iter()
            .scan(0, |acc, &days_per_month| {
                *acc += days_per_month;
                Some(*acc)
            })
            .collect();
        assert_eq!(0, FIRST_YEAR_OF_ERA[0]);
        assert_eq!(first_year_of_era.as_slice(), &FIRST_YEAR_OF_ERA[1..]);
    }

    #[test]
    fn test_eras() {
        assert_eq!(GameTime::from_years(3).eras(), 0);
        assert_eq!(
            (GameTime::from_years(FIRST_YEAR_OF_ERA[1]) - GameTime::from_milliseconds(1)).eras(),
            0
        );
        assert_eq!(GameTime::from_years(FIRST_YEAR_OF_ERA[1]).eras(), 1);
        assert_eq!(
            (GameTime::from_years(FIRST_YEAR_OF_ERA[1]) + GameTime::from_years(10)).eras(),
            1
        );
    }

    #[test]
    fn test_year_of_era() {
        assert_eq!(GameTime::from_years(3).year_of_era(), 3);
        assert_eq!(
            (GameTime::from_years(FIRST_YEAR_OF_ERA[1]) - GameTime::from_milliseconds(1))
                .year_of_era() as i128,
            FIRST_YEAR_OF_ERA[1] - 1
        );
        assert_eq!(GameTime::from_years(FIRST_YEAR_OF_ERA[1]).year_of_era(), 0);
        assert_eq!(
            (GameTime::from_years(FIRST_YEAR_OF_ERA[1]) + GameTime::from_years(10)).year_of_era(),
            10
        );
    }
}
