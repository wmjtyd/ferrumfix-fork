use crate::Buffer;
use crate::FixValue;

const LEN_IN_BYTES: usize = 8;

const ERR_GENERIC: &str = "Invalid day or week format.";

/// Canonical data field (DTF) for
/// [`FixDatatype::MonthYear`](crate::dict::FixDatatype::MonthYear).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MonthYear {
    year: u32,
    month: u32,
    day_or_week: DayOrWeek,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DayOrWeek {
    Day(u32),
    Week(u32),
}

impl MonthYear {
    /// Converts `self` to a byte array.
    pub fn to_yyyymmww(&self) -> [u8; LEN_IN_BYTES] {
        let day_or_week_1 = match self.day_or_week {
            DayOrWeek::Day(day) => (day / 10) as u8 + b'0',
            DayOrWeek::Week(_) => b'w',
        };
        let day_or_week_2 = match self.day_or_week {
            DayOrWeek::Day(day) => (day % 10) as u8 + b'0',
            DayOrWeek::Week(week) => week as u8 + b'0',
        };
        [
            (self.year() / 1000) as u8 + b'0',
            ((self.year() / 100) % 10) as u8 + b'0',
            ((self.year() / 10) % 10) as u8 + b'0',
            (self.year() % 10) as u8 + b'0',
            (self.month() / 10) as u8 + b'0',
            (self.month() % 10) as u8 + b'0',
            day_or_week_1,
            day_or_week_2,
        ]
    }

    /// Returns the year of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"19390901").unwrap();
    /// assert_eq!(dtf.year(), 1939)
    /// ```
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the month of `self`, starting from January as 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"20000101").unwrap();
    /// assert_eq!(dtf.month(), 1)
    /// ```
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day of `self`, if defined.
    ///
    /// # Examples
    ///
    /// Day included in the definition:
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"20191225").unwrap();
    /// assert_eq!(dtf.day(), Some(25))
    /// ```
    ///
    /// Day not included:
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"201801w3").unwrap();
    /// assert_eq!(dtf.day(), None)
    /// ```
    pub fn day(&self) -> Option<u32> {
        if let DayOrWeek::Day(day) = self.day_or_week {
            Some(day)
        } else {
            None
        }
    }

    /// Returns the intra-month week code of `self`, if defined. Note that it is
    /// 1-indexed.
    ///
    /// # Examples
    ///
    /// Present week code:
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"201912w1").unwrap();
    /// assert_eq!(dtf.week(), Some(1))
    /// ```
    ///
    /// Absent week code:
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_value::MonthYear;
    ///
    /// let dtf = MonthYear::deserialize(b"20191225").unwrap();
    /// assert_eq!(dtf.week(), None)
    /// ```
    pub fn week(&self) -> Option<u32> {
        if let DayOrWeek::Week(week) = self.day_or_week {
            Some(week)
        } else {
            None
        }
    }
}

impl<'a> FixValue<'a> for MonthYear {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let bytes = self.to_yyyymmww();
        buffer.extend_from_slice(&bytes[..]);
        bytes.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if validate(data) {
            Self::deserialize_lossy(data)
        } else {
            Err(ERR_GENERIC)
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let year = from_digit(data[0]) as u32 * 1000
            + from_digit(data[1]) as u32 * 100
            + from_digit(data[2]) as u32 * 10
            + from_digit(data[3]) as u32;
        let month = from_digit(data[4]) as u32 * 10 + from_digit(data[5]) as u32;
        let day_or_week = if data[6] == b'w' {
            DayOrWeek::Week(from_digit(data[7]) as u32)
        } else {
            DayOrWeek::Day(from_digit(data[6]) as u32 * 10 + from_digit(data[7]) as u32)
        };
        Ok(Self {
            year,
            month,
            day_or_week,
        })
    }
}

fn is_digit(byte: u8, min_digit: u8, max_digit: u8) -> bool {
    byte >= (min_digit + b'0') && byte <= (max_digit + b'0')
}

fn from_digit(digit: u8) -> u8 {
    digit.wrapping_sub(b'0')
}

fn validate(data: &[u8]) -> bool {
    if data.len() != 8 {
        return false;
    }
    if !validate_year(data) || !validate_month(data) {
        return false;
    }
    validate_week(data) || validate_day(data)
}

fn validate_year(data: &[u8]) -> bool {
    is_digit(data[0], 0, 9)
        && is_digit(data[1], 0, 9)
        && is_digit(data[2], 0, 9)
        && is_digit(data[3], 0, 9)
}

fn validate_month(data: &[u8]) -> bool {
    ((data[4] == b'0' && data[5] <= b'9') || (data[4] == b'1' && data[5] <= b'2'))
        && data[5] >= b'0'
}

fn validate_week(data: &[u8]) -> bool {
    data[6] == b'w' && is_digit(data[7], 1, 5)
}

fn validate_day(data: &[u8]) -> bool {
    ([b'0', b'1', b'2'].contains(&data[6]) && data[7] >= b'0' && data[7] <= b'9')
        || (data[6] == b'3' && data[7] >= b'0' && data[7] <= b'1')
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for MonthYear {
        fn arbitrary(g: &mut Gen) -> Self {
            let year = u32::arbitrary(g) % 10000;
            let month = (u32::arbitrary(g) % 12) + 1;
            let day_or_week = if bool::arbitrary(g) {
                format!("{:02}", (u32::arbitrary(g) % 31) + 1)
            } else {
                format!("w{}", (u32::arbitrary(g) % 5) + 1)
            };
            let s = format!("{:04}{:02}{}", year, month, day_or_week);
            MonthYear::deserialize(s.as_bytes()).unwrap()
        }
    }

    #[quickcheck]
    fn verify_serialization_behavior(my: MonthYear) -> bool {
        super::super::test_utility_verify_serialization_behavior(my)
    }

    #[quickcheck]
    fn can_deserialize_after_serializing(my: MonthYear) -> bool {
        let serialized = my.to_bytes();
        let deserialized = MonthYear::deserialize(&serialized[..]).unwrap();
        let deserialized_lossy = MonthYear::deserialize_lossy(&serialized[..]).unwrap();
        deserialized == my && deserialized_lossy == my
    }
}
