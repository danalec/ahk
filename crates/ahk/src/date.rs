//! The date and the time of the machine.
//!
//! The dialect of this tree is AutoHotkey v2.0.19, and the date is the
//! one of that binary: the number of a date is `YYYYMMDDHH24MISS`, the
//! formats are the letters of v2, and the failures say what v2 says.
//!
//! The measurements that pin this module were made on the binary of
//! v2.0.19 on 2026-10-28, and they are the ones the notes of the tree
//! carry:
//!
//! - `DateAdd` and `DateDiff` take the date first and the unit last:
//!   `DateAdd(20240101, 1, "d")` answers `20240102000000`. The unit is
//!   one of `d`, `h`, `m` and `s`; v2 has no unit of years nor of
//!   months, and it refuses the others with "Parameter #3 of DateAdd is
//!   invalid.".
//! - `FormatTime` takes the date first as well:
//!   `FormatTime(20240102, "yyyy-MM-dd")` answers `2024-01-02`.
//! - The date of v2 carries no millisecond: the number
//!   `20240102030405.678` is refused, because the point is not a date
//!   that v2 can read.
//! - `DateDiff` answers the first date minus the second, and it
//!   truncates towards zero.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{invalid_parameter, requires, Runtime, RuntimeError, TypeName, Value};

/// The reason the date and the time are not answered.
///
/// The clock of a run counts the milliseconds of a run from an arbitrary
/// origin: a host that knows the date says so through the clock, and a
/// host that does not leaves the names of the date with this reason.
pub const NO_DATE: &str = "the clock of this runtime does not know the date of the machine.";

/// The format of v2 when the script does not say one.
///
/// Measured on the binary of v2.0.19 on 2026-10-28:
/// `FormatTime(20240102)` answers `12:00 AM Tuesday, January 2, 2024`.
pub const DEFAULT_FORMAT: &str = "h:mm tt dddd, MMMM d, yyyy";

/// The months of the year, in the two spellings of v2.
const SHORT_MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const LONG_MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The days of the week, in the two spellings of v2.
///
/// The Sunday is the first, the way v2 counts it.
const SHORT_DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const LONG_DAYS: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// A moment of the date and the time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    /// The year.
    pub year: i64,
    /// The month, 1 being January.
    pub month: i64,
    /// The day of the month, 1 being the first.
    pub day: i64,
    /// The hour, 0 being midnight.
    pub hour: i64,
    /// The minute.
    pub minute: i64,
    /// The second.
    pub second: i64,
    /// The millisecond.
    pub millisecond: i64,
    /// The day of the week, 1 being the Sunday, the way v2 counts it.
    pub weekday: i64,
    /// The day of the year, 1 being the first of January.
    pub yday: i64,
}

/// The date and the time of the machine, when the platform knows them.
///
/// The clock of a run counts from an arbitrary origin, and the date is
/// not in it: a host that has no `std` says that it does not know the
/// date, and the names of the date answer with the reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WallClock {
    /// The local time of the machine.
    pub local: DateTime,
    /// The same moment, in the UTC.
    pub utc: DateTime,
    /// The offset of the local time in minutes, east of the UTC.
    pub offset_minutes: i64,
    /// Whether the machine knows its own timezone.
    pub knows_the_timezone: bool,
}

/// The date and the time of the machine, when the platform knows them.
///
/// The UTC comes from the clock of the system. The local time needs the
/// timezone of the machine, and `std` does not have it: the `TZ` of the
/// environment is read first when it is a plain offset, and the file of
/// the zone comes next. A machine that says neither does not know its
/// timezone, and the tree says so: the offset is nothing and the local
/// time is the UTC.
#[cfg(feature = "std")]
pub fn system_wall_clock() -> Option<WallClock> {
    let millis = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(since) => since.as_millis() as i64,
        // A machine whose clock is before the epoch has a date that the
        // arithmetic of the tree cannot hold.
        Err(_) => return None,
    };
    let utc = from_unix_millis(millis);
    let offset = the_offset_of_the_machine(millis / 1000);
    let local = match offset {
        Some(minutes) => from_unix_millis(millis + minutes * 60_000),
        None => utc,
    };
    Some(WallClock {
        local,
        utc,
        offset_minutes: offset.unwrap_or(0),
        knows_the_timezone: offset.is_some(),
    })
}

/// Whether the year has a day of February that is not in every year.
pub fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// The days of a month, with the leap years.
///
/// A month that is not one of the twelve has no days: the tree says so
/// rather than guessing one.
pub fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// The days of a year.
pub fn days_in_year(year: i64) -> i64 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

/// The day of the week, 1 being the Sunday, the way v2 counts it.
pub fn day_of_week(year: i64, month: i64, day: i64) -> i64 {
    // The first of January of 1970 was a Thursday, and the epoch is the
    // day zero of the count.
    let days = days_from_civil(year, month, day);
    (days + 4).rem_euclid(7) + 1
}

/// The day of the year, 1 being the first of January.
pub fn day_of_year(year: i64, month: i64, day: i64) -> i64 {
    let mut days = day;
    for month in 1..month {
        days += days_in_month(year, month);
    }
    days
}

/// The days since the Unix epoch, the way the calendar counts them.
///
/// The arithmetic is the one of Howard Hinnant: it holds every year of
/// the calendar, and it needs no table of the months.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146097 + day_of_era - 719468
}

/// The year, the month and the day of a number of days since the epoch.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let day_of_era = days - era * 146097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month + 2) / 5 + 1;
    let month = month + if month < 10 { 3 } else { -9 };
    (year + i64::from(month <= 2), month, day)
}

/// The moment of a number of milliseconds since the Unix epoch, in the UTC.
pub fn from_unix_millis(millis: i64) -> DateTime {
    let day = millis.div_euclid(86_400_000);
    let rest = millis.rem_euclid(86_400_000);
    let (year, month, day) = civil_from_days(day);
    let hour = rest / 3_600_000;
    let minute = rest % 3_600_000 / 60_000;
    let second = rest % 60_000 / 1000;
    let millisecond = rest % 1000;
    DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
        weekday: day_of_week(year, month, day),
        yday: day_of_year(year, month, day),
    }
}

/// The milliseconds since the Unix epoch of a moment, in the UTC.
pub fn to_unix_millis(date: &DateTime) -> i64 {
    days_from_civil(date.year, date.month, date.day) * 86_400_000
        + date.hour * 3_600_000
        + date.minute * 60_000
        + date.second * 1000
        + date.millisecond
}

/// The number of v2 for a moment: `YYYYMMDDHH24MISS`.
///
/// The date of v2 carries no millisecond: the binary of v2.0.19 refuses
/// the number with a point, so the tree refuses it as well.
pub fn to_number(date: &DateTime) -> f64 {
    the_number_text(date).parse().unwrap_or(0.0)
}

/// The number of v2 of a moment, as the text of it.
pub fn the_number_text(date: &DateTime) -> String {
    format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        date.year, date.month, date.day, date.hour, date.minute, date.second
    )
}

/// A moment, with the two groups that v2 may leave out.
///
/// The date of v2 is the year, the month and the day, or it is nothing;
/// the time of v2 is the hour, the minute and the second, or it is
/// nothing. A group that is not a real one writes nothing at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moment {
    /// The date and the time.
    pub date: DateTime,
    /// Whether the year, the month and the day are a real one.
    pub date_ok: bool,
    /// Whether the hour, the minute and the second are a real one.
    pub time_ok: bool,
}

/// What the text of a date carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parsed {
    /// The text carries no date at all: v2 takes the date of the machine.
    Nothing,
    /// The text is not a date that v2 can read.
    Malformed,
    /// The text carries a date.
    Date(Moment),
}

/// The date of a text, the way v2 reads it.
///
/// The text is the number of v2, `YYYYMMDDHH24MISS`, and the shorter
/// forms of it: the year alone, the year and the month, and so on. The
/// month and the day that are not there are the first of January, and
/// the hour, the minute and the second that are not there are zero.
///
/// A year before 1601, a month that is not one, a day that the month
/// does not have, an hour above 23 and a minute above 59 are not a date:
/// the group they are in is empty, and the codes of that group write
/// nothing. The second may be sixty, because v2 keeps the leap second.
pub fn parse(text: &str) -> Parsed {
    let text = text.trim();
    if !text.starts_with(|letter: char| letter.is_ascii_digit()) {
        return Parsed::Nothing;
    }
    let digits: String = text.chars().take_while(|letter| letter.is_ascii_digit()).collect();
    // What follows the digits is only a space and what the space
    // carries: the digits of the text are the date, and the rest of it
    // is not read. Something else right after the digits is something
    // that v2 cannot read.
    let rest = &text[digits.len()..];
    if !rest.is_empty() && !rest.starts_with(' ') {
        return Parsed::Malformed;
    }
    let take = |at: usize, len: usize| -> i64 { digits[at..at + len].parse().unwrap_or(0) };
    let (year, month, day, hour, minute, second) = match digits.len() {
        4 => (take(0, 4), 1, 1, 0, 0, 0),
        6 => (take(0, 4), take(4, 2), 1, 0, 0, 0),
        8 => (take(0, 4), take(4, 2), take(6, 2), 0, 0, 0),
        10 => (take(0, 4), take(4, 2), take(6, 2), take(8, 2), 0, 0),
        12 => (take(0, 4), take(4, 2), take(6, 2), take(8, 2), take(10, 2), 0),
        14 => (take(0, 4), take(4, 2), take(6, 2), take(8, 2), take(10, 2), take(12, 2)),
        // The other lengths are not a date that v2 can read.
        _ => return Parsed::Malformed,
    };
    let date_ok = (1601..=9999).contains(&year)
        && (1..=12).contains(&month)
        && day >= 1
        && day <= days_in_month(year, month);
    let time_ok = (0..=23).contains(&hour) && (0..=59).contains(&minute) && (0..=60).contains(&second);
    let date = DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond: 0,
        weekday: if date_ok { day_of_week(year, month, day) } else { 0 },
        yday: if date_ok { day_of_year(year, month, day) } else { 0 },
    };
    Parsed::Date(Moment { date, date_ok, time_ok })
}

/// A code of the format of v2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Code {
    /// The year: the last two digits, or the four of it.
    Year(bool),
    /// The month: the number, the number in two digits, the short name or
    /// the long one.
    Month(u8),
    /// The day of the month or the day of the week: the number, the
    /// number in two digits, the short name or the long one.
    Day(u8),
    /// The hour, in the 24-hour form or in the 12-hour one, and whether
    /// it is written with two digits.
    Hour(bool, bool),
    /// The minute, and whether it is written with two digits.
    Minute(bool),
    /// The second, and whether it is written with two digits.
    Second(bool),
    /// The meridiem: "A", "AM" or "a".
    AmPm(u8),
    /// The era: "A.D.".
    Era,
}

/// One piece of a format of v2.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Piece {
    /// Text that is written as it is.
    Literal(String),
    /// A code of the date.
    Date(Code),
    /// A code of the time.
    Time(Code),
}

/// The pieces of a format of v2.
///
/// The quotes hold a text that is written as it is, and a code is a run
/// of the same letter: `yyyy` is one code and not four.
fn pieces(format: &str) -> Vec<Piece> {
    let mut out = Vec::new();
    let mut letters = format.chars().peekable();
    while let Some(letter) = letters.next() {
        if letter == '\'' {
            let mut text = String::new();
            for next in letters.by_ref() {
                if next == '\'' {
                    break;
                }
                text.push(next);
            }
            out.push(Piece::Literal(text));
            continue;
        }
        let mut run = String::new();
        run.push(letter);
        while let Some(next) = letters.peek() {
            if *next != letter {
                break;
            }
            run.push(*next);
            letters.next();
        }
        match the_code(&run) {
            Some(code) if is_time(&code) => out.push(Piece::Time(code)),
            Some(code) => out.push(Piece::Date(code)),
            None => out.push(Piece::Literal(run)),
        }
    }
    out
}

/// The code of a run of letters, when the run is one.
fn the_code(run: &str) -> Option<Code> {
    let letter = run.chars().next()?;
    let count = run.chars().count() as u8;
    match letter {
        'y' => Some(Code::Year(count >= 3)),
        'M' => Some(Code::Month(count.min(4))),
        'd' => Some(Code::Day(count.min(4))),
        'H' => Some(Code::Hour(false, count >= 2)),
        'h' => Some(Code::Hour(true, count >= 2)),
        'm' => Some(Code::Minute(count >= 2)),
        's' => Some(Code::Second(count >= 2)),
        't' => Some(Code::AmPm(count)),
        'g' => Some(Code::Era),
        _ => None,
    }
}

/// Whether a code is one of the time.
fn is_time(code: &Code) -> bool {
    matches!(
        code,
        Code::Hour(..) | Code::Minute(..) | Code::Second(..) | Code::AmPm(..)
    )
}

/// The text of a code, for a moment.
fn the_value(code: &Code, moment: &Moment) -> String {
    let date = &moment.date;
    match code {
        Code::Year(long) => {
            if *long {
                format!("{:04}", date.year)
            } else {
                format!("{:02}", date.year % 100)
            }
        }
        Code::Month(1) => date.month.to_string(),
        Code::Month(2) => format!("{:02}", date.month),
        Code::Month(3) => SHORT_MONTHS[(date.month - 1).clamp(0, 11) as usize].to_string(),
        Code::Month(_) => LONG_MONTHS[(date.month - 1).clamp(0, 11) as usize].to_string(),
        Code::Day(1) => date.day.to_string(),
        Code::Day(2) => format!("{:02}", date.day),
        Code::Day(3) => SHORT_DAYS[(date.weekday - 1).clamp(0, 6) as usize].to_string(),
        Code::Day(_) => LONG_DAYS[(date.weekday - 1).clamp(0, 6) as usize].to_string(),
        Code::Hour(twelve, padded) => {
            let mut hour = date.hour;
            if *twelve {
                hour %= 12;
                if hour == 0 {
                    hour = 12;
                }
            }
            if *padded {
                format!("{:02}", hour)
            } else {
                hour.to_string()
            }
        }
        Code::Minute(padded) => {
            if *padded {
                format!("{:02}", date.minute)
            } else {
                date.minute.to_string()
            }
        }
        Code::Second(padded) => {
            if *padded {
                format!("{:02}", date.second)
            } else {
                date.second.to_string()
            }
        }
        Code::AmPm(1) => (if date.hour < 12 { "A" } else { "P" }).to_string(),
        Code::AmPm(2) => (if date.hour < 12 { "AM" } else { "PM" }).to_string(),
        Code::AmPm(_) => (if date.hour < 12 { "a" } else { "p" }).to_string(),
        Code::Era => "A.D.".to_string(),
    }
}

/// The format of v2 applied to a moment.
///
/// The format is split at the first code of the time: the codes of the
/// date and the text in front of them are the date, and the rest is the
/// time. A group that is not a real one writes nothing at all, the
/// codes of it and the text in between.
pub fn format_moment(moment: &Moment, format: &str) -> String {
    let format = if format.is_empty() { DEFAULT_FORMAT } else { format };
    let mut out = String::new();
    let mut in_time = false;
    for piece in pieces(format) {
        match piece {
            Piece::Literal(text) => {
                // The text belongs to the group of the code in front of it.
                let written = if in_time { moment.time_ok } else { moment.date_ok };
                if written {
                    out.push_str(&text);
                }
            }
            Piece::Date(code) => {
                let written = if in_time { moment.time_ok } else { moment.date_ok };
                if written {
                    out.push_str(&the_value(&code, moment));
                }
            }
            Piece::Time(code) => {
                in_time = true;
                if moment.time_ok {
                    out.push_str(&the_value(&code, moment));
                }
            }
        }
    }
    out
}

/// The unit of a date, the way v2 reads it.
///
/// v2 has no unit of years nor of months: the letters that are left are
/// the days, the hours, the minutes and the seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Day,
    Hour,
    Minute,
    Second,
}

impl Unit {
    /// The milliseconds of the unit.
    fn millis(self) -> i64 {
        match self {
            Unit::Day => 86_400_000,
            Unit::Hour => 3_600_000,
            Unit::Minute => 60_000,
            Unit::Second => 1000,
        }
    }
}

/// The moment of an argument, the way v2 reads it.
///
/// A date that v2 cannot read is not a date: the parameter is invalid.
fn the_moment(values: &[Value], index: usize, name: &str, line: u32) -> Result<Moment, RuntimeError> {
    let text = values.get(index).map(|value| value.to_text()).unwrap_or_default();
    match parse(&text) {
        Parsed::Date(moment) if moment.date_ok && moment.time_ok => Ok(moment),
        _ => Err(RuntimeError { message: invalid_parameter(index + 1, name), line }),
    }
}

/// The number of an argument, the way v2 wants it.
///
/// v2 does not take a text for a number: it says what it got.
fn the_number(values: &[Value], index: usize, name: &str, line: u32) -> Result<f64, RuntimeError> {
    match values.get(index) {
        Some(Value::Int(number)) => Ok(*number as f64),
        Some(Value::Float(number)) => Ok(*number),
        Some(other) => Err(RuntimeError {
            message: requires(index + 1, name, TypeName::Number, other),
            line,
        }),
        None => Err(RuntimeError { message: invalid_parameter(index + 1, name), line }),
    }
}

/// The unit of an argument, the way v2 reads it.
fn the_unit(values: &[Value], index: usize, name: &str, line: u32) -> Result<Unit, RuntimeError> {
    let text = values.get(index).map(|value| value.to_text()).unwrap_or_default();
    match text.as_str() {
        "d" | "D" => Ok(Unit::Day),
        "h" | "H" => Ok(Unit::Hour),
        "m" | "M" => Ok(Unit::Minute),
        "s" | "S" => Ok(Unit::Second),
        _ => Err(RuntimeError { message: invalid_parameter(index + 1, name), line }),
    }
}

/// The date and the time of the machine, in the local time.
fn the_now(rt: &Runtime<'_>, line: u32) -> Result<Moment, RuntimeError> {
    let Some(clock) = rt.wall_clock() else {
        return Err(RuntimeError { message: NO_DATE.to_string(), line });
    };
    Ok(Moment { date: clock.local, date_ok: true, time_ok: true })
}

/// `DateAdd`: a moment, plus an amount of time.
///
/// The unit of v2 is the day, the hour, the minute or the second, and
/// the answer is the number of v2 without a millisecond.
pub fn date_add(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let moment = the_moment(values, 0, "DateAdd", line)?;
    let amount = the_number(values, 1, "DateAdd", line)?;
    let unit = the_unit(values, 2, "DateAdd", line)?;
    // The amount may have a fraction of the unit, and the fraction is
    // kept as far as the second.
    let millis = (amount * unit.millis() as f64) as i64;
    let moved = to_unix_millis(&moment.date) + millis;
    Ok(Value::Text(the_number_text(&from_unix_millis(moved))))
}

/// `DateDiff`: the difference between two moments.
///
/// The answer is the first date minus the second, and it is truncated
/// towards zero.
pub fn date_diff(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let first = the_moment(values, 0, "DateDiff", line)?;
    let second = the_moment(values, 1, "DateDiff", line)?;
    let unit = the_unit(values, 2, "DateDiff", line)?;
    let millis = to_unix_millis(&first.date) - to_unix_millis(&second.date);
    Ok(Value::Int(millis / unit.millis()))
}

/// `FormatTime`: a moment, written the way the format says.
///
/// The date comes first and the format comes last. A date that v2
/// cannot read is an invalid parameter, and a text that carries no date
/// at all is the date of the machine.
pub fn format_time(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let moment = match values.first() {
        None => the_now(rt, line)?,
        Some(value) => match parse(&value.to_text()) {
            Parsed::Nothing => the_now(rt, line)?,
            Parsed::Malformed => {
                return Err(RuntimeError { message: invalid_parameter(1, "FormatTime"), line })
            }
            Parsed::Date(moment) => moment,
        },
    };
    let format = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    Ok(Value::Text(format_moment(&moment, &format)))
}

/// The value of a built-in variable of the date.
///
/// The name is the one the table of the variables carries: the name of
/// v2 without the `A_` prefix, in lower case.
///
/// Every one of them is a text, the way v2 answers them.
pub fn variable_value(name: &str, rt: &Runtime<'_>) -> Result<Value, RuntimeError> {
    let Some(clock) = rt.wall_clock() else {
        return Err(RuntimeError { message: NO_DATE.to_string(), line: 0 });
    };
    let local = &clock.local;
    let text = match name {
        "now" => the_number_text(local),
        "nowutc" => the_number_text(&clock.utc),
        "year" | "yyyy" => format!("{:04}", local.year),
        "mon" | "mm" => format!("{:02}", local.month),
        "mday" | "dd" => format!("{:02}", local.day),
        "hour" => format!("{:02}", local.hour),
        "min" => format!("{:02}", local.minute),
        "sec" => format!("{:02}", local.second),
        "msec" => format!("{:03}", local.millisecond),
        "wday" => local.weekday.to_string(),
        "yday" => local.yday.to_string(),
        "yweek" => format!("{:04}{:02}", local.year, the_week(local)),
        "mmm" => SHORT_MONTHS[(local.month - 1).clamp(0, 11) as usize].to_string(),
        "mmmm" => LONG_MONTHS[(local.month - 1).clamp(0, 11) as usize].to_string(),
        "ddd" => SHORT_DAYS[(local.weekday - 1).clamp(0, 6) as usize].to_string(),
        "dddd" => LONG_DAYS[(local.weekday - 1).clamp(0, 6) as usize].to_string(),
        other => {
            return Err(RuntimeError {
                message: format!("this runtime does not carry the built-in variable {other}."),
                line: 0,
            })
        }
    };
    Ok(Value::Text(text))
}

/// The week of the year, the way v2 counts it.
///
/// The weeks begin on a Sunday, and the first week is the one that holds
/// the first of January.
fn the_week(date: &DateTime) -> i64 {
    let first = day_of_week(date.year, 1, 1);
    (date.yday - 1 + first - 1) / 7 + 1
}

/// The offset of the local time of the machine, in minutes east of the
/// UTC.
///
/// The `TZ` of the environment comes first when it is a plain offset, and
/// the file of the zone comes next. A machine that says neither does
/// not know its timezone.
#[cfg(feature = "std")]
fn the_offset_of_the_machine(unix_seconds: i64) -> Option<i64> {
    if let Ok(tz) = std::env::var("TZ") {
        if let Some(minutes) = the_offset_of_a_name(&tz) {
            return Some(minutes);
        }
    }
    the_offset_of_the_file("/etc/localtime", unix_seconds)
}

/// The offset of a plain offset, the way the `TZ` of the environment
/// says it.
///
/// The name is the zone and the sign is the offset east of the UTC: the
/// `BRT-3` of a machine that is three hours behind the UTC is minus
/// three hours. A name without an offset says nothing about the hours,
/// and the file of the zone is read instead.
#[cfg(feature = "std")]
fn the_offset_of_a_name(tz: &str) -> Option<i64> {
    let text = tz.trim();
    if text.is_empty() {
        return None;
    }
    let at = text.find(['+', '-']);
    let (name, rest) = match at {
        Some(at) => (&text[..at], &text[at..]),
        None => (text, ""),
    };
    if name.is_empty() || !name.chars().all(|letter| letter.is_ascii_alphabetic() || letter == '_') {
        return None;
    }
    if rest.is_empty() {
        return match name.to_ascii_uppercase().as_str() {
            "UTC" | "GMT" | "Z" => Some(0),
            _ => None,
        };
    }
    let sign = match rest.as_bytes()[0] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let rest = &rest[1..];
    let (hours, minutes) = match rest.split_once(':') {
        Some((hours, minutes)) => (hours, minutes),
        None => (rest, "0"),
    };
    if hours.is_empty() || hours.len() > 2 || !hours.chars().all(|letter| letter.is_ascii_digit()) {
        return None;
    }
    if minutes.len() > 2 || !minutes.chars().all(|letter| letter.is_ascii_digit()) {
        return None;
    }
    let hours: i64 = hours.parse().ok()?;
    let minutes: i64 = minutes.parse().ok()?;
    if hours > 24 {
        return None;
    }
    Some(sign * (hours * 60 + minutes))
}

/// The offset of the local time out of the file of the zone.
///
/// The file is the TZif of the machine: its transitions say where the
/// offset changed, and the type of a moment is the type of the last
/// transition before it.
#[cfg(feature = "std")]
fn the_offset_of_the_file(path: &str, unix_seconds: i64) -> Option<i64> {
    let bytes = std::fs::read(path).ok()?;
    let zone = Zone::read(&bytes)?;
    zone.offset_at(unix_seconds)
}

/// The zone of a TZif file.
#[cfg(feature = "std")]
struct Zone {
    /// The offset of each type, in seconds east of the UTC.
    offsets: Vec<i64>,
    /// Whether each type is a summer time.
    daylight: Vec<bool>,
    /// The moments where the type changed.
    transitions: Vec<i64>,
    /// The type of each moment.
    kinds: Vec<u8>,
}

#[cfg(feature = "std")]
impl Zone {
    /// The zone of a TZif file.
    ///
    /// The file of the version two and of the version three carries two
    /// blocks: the first one has the moments in 32 bits, and the second
    /// one has them in 64. The second is the one that holds every year.
    fn read(bytes: &[u8]) -> Option<Zone> {
        if bytes.len() < 44 || &bytes[0..4] != b"TZif" {
            return None;
        }
        let version = bytes[4];
        let counts = the_counts(bytes, 20)?;
        if version == b'2' || version == b'3' {
            let at = 44 + the_size_of_the_block(counts, 4);
            let counts = the_counts(bytes, at)?;
            return the_block(bytes, at + 44, counts, 8);
        }
        the_block(bytes, 44, counts, 4)
    }

    /// The offset of a moment, in minutes east of the UTC.
    fn offset_at(&self, unix_seconds: i64) -> Option<i64> {
        // The type of a moment is the type of the last change before it.
        let mut kind: Option<u8> = None;
        for (at, transition) in self.transitions.iter().enumerate() {
            if *transition > unix_seconds {
                break;
            }
            kind = self.kinds.get(at).copied();
        }
        let kind = match kind {
            Some(kind) => kind as usize,
            // Before the first change: the first type that is not a
            // summer time.
            None => self.daylight.iter().position(|summer| !summer).unwrap_or(0),
        };
        self.offsets.get(kind).map(|offset| offset / 60)
    }
}

/// The counts of what a TZif file holds.
#[cfg(feature = "std")]
type Counts = (usize, usize, usize, usize, usize, usize);

/// The counts of the header of a TZif file.
#[cfg(feature = "std")]
fn the_counts(bytes: &[u8], at: usize) -> Option<Counts> {
    let read = |index: usize| -> Option<usize> {
        let at = at + index * 4;
        let four = bytes.get(at..at + 4)?;
        Some(u32::from_be_bytes([four[0], four[1], four[2], four[3]]) as usize)
    };
    Some((read(0)?, read(1)?, read(2)?, read(3)?, read(4)?, read(5)?))
}

/// The size of the block of data of a TZif file.
#[cfg(feature = "std")]
fn the_size_of_the_block(counts: Counts, time_size: usize) -> usize {
    let (isutcnt, isstdcnt, leapcnt, timecnt, typecnt, charcnt) = counts;
    timecnt * time_size + timecnt + typecnt * 6 + charcnt + leapcnt * (time_size + 4) + isstdcnt + isutcnt
}

/// The block of data of a TZif file.
#[cfg(feature = "std")]
fn the_block(bytes: &[u8], at: usize, counts: Counts, time_size: usize) -> Option<Zone> {
    let (_, _, _, timecnt, typecnt, _) = counts;
    let mut at = at;
    // The moments where the type changed.
    let mut transitions = Vec::with_capacity(timecnt);
    for _ in 0..timecnt {
        let bytes = bytes.get(at..at + time_size)?;
        let mut value: i64 = 0;
        for byte in bytes {
            value = (value << 8) | i64::from(*byte);
        }
        if time_size == 4 {
            value = (value as i32) as i64;
        }
        transitions.push(value);
        at += time_size;
    }
    // The type of each of them.
    let mut kinds = Vec::with_capacity(timecnt);
    for _ in 0..timecnt {
        kinds.push(*bytes.get(at)?);
        at += 1;
    }
    // The types themselves.
    let mut offsets = Vec::with_capacity(typecnt);
    let mut daylight = Vec::with_capacity(typecnt);
    for _ in 0..typecnt {
        let four = bytes.get(at..at + 4)?;
        let offset = i32::from_be_bytes([four[0], four[1], four[2], four[3]]) as i64;
        offsets.push(offset);
        daylight.push(*bytes.get(at + 4)? != 0);
        at += 6;
    }
    Some(Zone { offsets, daylight, transitions, kinds })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{
        Clipboard, Desktop, Files, Hotkeys, Keyboard, Machine, Native, Pointer,
        Processes, Screen, Sound, Timers, Windows,
    };
    use crate::runtime::Clock;
    use alloc::string::String;
    use core::time::Duration;

    /// The moment of the tests: the second of January of 2024, at ten
    /// past three in the morning, in the UTC.
    const MOMENT: i64 = 1_704_164_645_678;

    /// A desktop that answers nothing: this module asks it no question.
    #[derive(Debug)]
    struct Fake;

    impl Desktop for Fake {}
    impl Sound for Fake {}
impl Machine for Fake {}

    impl Files for Fake {}

    impl Clipboard for Fake {}

    impl Timers for Fake {}

    impl Hotkeys for Fake {}

    impl Windows for Fake {}

    impl Native for Fake {}

    impl Keyboard for Fake {
        fn type_text(&mut self, _text: &str) -> Result<String, String> {
            Ok(String::new())
        }
        fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Pointer for Fake {
        fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Screen for Fake {
        fn capture(
            &mut self,
            _x: i64,
            _y: i64,
            _width: i64,
            _height: i64,
            _filename: &str,
        ) -> Result<String, String> {
            Ok(String::new())
        }
        fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Processes for Fake {
        fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
            Ok((0, String::new(), String::new()))
        }
    }

    /// The clock of the tests: it knows the date of a moment.
    #[derive(Debug)]
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            0
        }
        fn sleep_ms(&mut self, _millis: u64) {}

        fn wall_clock(&self) -> Option<WallClock> {
            Some(WallClock {
                local: from_unix_millis(MOMENT),
                utc: from_unix_millis(MOMENT),
                offset_minutes: 0,
                knows_the_timezone: true,
            })
        }
    }

    /// The clock of a runtime that does not know the date.
    #[derive(Debug)]
    struct TheBareClock;

    impl Clock for TheBareClock {
        fn now_ms(&self) -> u64 {
            0
        }
        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// The answer of a body, with a clock that knows the date.
    fn with_clock<T>(body: impl FnOnce(&mut Runtime<'_>) -> T) -> T {
        let mut desktop = Fake;
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        body(&mut rt)
    }

    /// The answer of a body, with a clock that does not know the date.
    fn with_bare_clock<T>(body: impl FnOnce(&mut Runtime<'_>) -> T) -> T {
        let mut desktop = Fake;
        let mut clock = TheBareClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        body(&mut rt)
    }

    /// A moment of the tests, with the day of the week and the day of
    /// the year of it.
    fn a_moment(
        year: i64,
        month: i64,
        day: i64,
        hour: i64,
        minute: i64,
        second: i64,
    ) -> Moment {
        let date = DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond: 0,
            weekday: day_of_week(year, month, day),
            yday: day_of_year(year, month, day),
        };
        Moment { date, date_ok: true, time_ok: true }
    }

    #[test]
    fn a_leap_year_has_a_day_of_february_that_is_not_in_every_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        // A year of a hundred is not a leap year, unless it is of four
        // hundred.
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
    }

    #[test]
    fn the_days_of_a_month_are_the_ones_of_the_calendar() {
        assert_eq!(days_in_month(2024, 1), 31);
        assert_eq!(days_in_month(2024, 4), 30);
        assert_eq!(days_in_month(2024, 9), 30);
        assert_eq!(days_in_month(2024, 12), 31);
        assert_eq!(days_in_year(2024), 366);
        assert_eq!(days_in_year(2023), 365);
        // A month that is not one of the twelve has no days.
        assert_eq!(days_in_month(2024, 13), 0);
    }

    #[test]
    fn the_day_of_the_week_is_the_one_of_the_calendar() {
        // Measured on the binary of v2.0.19 on 2026-10-28: the
        // twenty-eighth of October of 2026 was a Wednesday, and v2 counts
        // the Sunday as the first.
        assert_eq!(day_of_week(2026, 10, 28), 4);
        assert_eq!(day_of_week(2024, 1, 2), 3);
        // The first of January of 1970 was a Thursday.
        assert_eq!(day_of_week(1970, 1, 1), 5);
    }

    #[test]
    fn the_day_of_the_year_counts_from_the_first_of_january() {
        // Measured on the binary of v2.0.19 on 2026-10-28: the
        // twenty-eighth of October of 2026 was the three hundred and
        // first day of the year.
        assert_eq!(day_of_year(2026, 10, 28), 301);
        assert_eq!(day_of_year(2024, 1, 2), 2);
        assert_eq!(day_of_year(2024, 12, 31), 366);
    }

    #[test]
    fn the_moment_of_the_epoch_is_the_first_of_january_of_1970() {
        let moment = from_unix_millis(0);
        assert_eq!(moment.year, 1970);
        assert_eq!(moment.month, 1);
        assert_eq!(moment.day, 1);
        assert_eq!(moment.hour, 0);
        assert_eq!(moment.minute, 0);
        assert_eq!(moment.second, 0);
        assert_eq!(moment.weekday, 5);
    }

    #[test]
    fn a_moment_and_the_milliseconds_it_came_from_are_the_same() {
        let moment = from_unix_millis(MOMENT);
        assert_eq!(moment.year, 2024);
        assert_eq!(moment.month, 1);
        assert_eq!(moment.day, 2);
        assert_eq!(moment.hour, 3);
        assert_eq!(moment.minute, 4);
        assert_eq!(moment.second, 5);
        assert_eq!(moment.millisecond, 678);
        assert_eq!(to_unix_millis(&moment), MOMENT);
    }

    #[test]
    fn the_number_of_v2_is_the_date_and_the_time_without_a_separator() {
        let moment = from_unix_millis(MOMENT);
        assert_eq!(to_number(&moment), 20240102030405.0);
        assert_eq!(the_number_text(&moment), "20240102030405");
    }

    #[test]
    fn the_format_of_v2_writes_the_date_the_way_v2_writes_it() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let moment = a_moment(2024, 1, 2, 15, 4, 5);
        assert_eq!(format_moment(&moment, "yyyy-MM-dd HH:mm:ss"), "2024-01-02 15:04:05");
        assert_eq!(format_moment(&moment, "y"), "24");
        assert_eq!(format_moment(&moment, "yy"), "24");
        assert_eq!(format_moment(&moment, "yyy"), "2024");
        assert_eq!(format_moment(&moment, "M"), "1");
        assert_eq!(format_moment(&moment, "MM"), "01");
        assert_eq!(format_moment(&moment, "MMM"), "Jan");
        assert_eq!(format_moment(&moment, "MMMM"), "January");
        assert_eq!(format_moment(&moment, "d"), "2");
        assert_eq!(format_moment(&moment, "dd"), "02");
        assert_eq!(format_moment(&moment, "ddd"), "Tue");
        assert_eq!(format_moment(&moment, "dddd"), "Tuesday");
        assert_eq!(format_moment(&moment, "H"), "15");
        assert_eq!(format_moment(&moment, "HH"), "15");
        assert_eq!(format_moment(&moment, "h"), "3");
        assert_eq!(format_moment(&moment, "hh"), "03");
        assert_eq!(format_moment(&moment, "m"), "4");
        assert_eq!(format_moment(&moment, "mm"), "04");
        assert_eq!(format_moment(&moment, "s"), "5");
        assert_eq!(format_moment(&moment, "ss"), "05");
        assert_eq!(format_moment(&moment, "t"), "P");
        assert_eq!(format_moment(&moment, "tt"), "PM");
        assert_eq!(format_moment(&moment, "g"), "A.D.");
    }

    #[test]
    fn a_code_that_v2_does_not_know_is_written_as_it_is() {
        // Measured on the binary of v2.0.19 on 2026-10-28: the letters
        // that v2 has no code for are written as they are.
        let moment = a_moment(2024, 1, 2, 15, 4, 5);
        assert_eq!(format_moment(&moment, "EEEE"), "EEEE");
        assert_eq!(format_moment(&moment, "ww"), "ww");
        assert_eq!(format_moment(&moment, "yyyyQ"), "2024Q");
    }

    #[test]
    fn the_quotes_of_a_format_hold_the_text_that_is_written_as_it_is() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let moment = a_moment(2024, 1, 2, 15, 4, 5);
        assert_eq!(format_moment(&moment, "'literal' yyyy"), "literal 2024");
        assert_eq!(format_moment(&moment, "'yyyy' yyyy"), "yyyy 2024");
        assert_eq!(format_moment(&moment, "''"), "");
    }

    #[test]
    fn the_format_of_v2_that_is_empty_is_the_default_of_v2() {
        // Measured on the binary of v2.0.19 on 2026-10-28:
        // `FormatTime(20240102)` answers `12:00 AM Tuesday, January 2,
        // 2024`.
        let moment = a_moment(2024, 1, 2, 0, 0, 0);
        assert_eq!(format_moment(&moment, ""), "12:00 AM Tuesday, January 2, 2024");
    }

    #[test]
    fn a_text_that_v2_cannot_read_is_not_a_date() {
        assert!(matches!(parse("hello"), Parsed::Nothing));
        assert!(matches!(parse(""), Parsed::Nothing));
        // A date with something that is not a digit in it is malformed.
        assert!(matches!(parse("2024-01-02"), Parsed::Malformed));
        assert!(matches!(parse("20240102."), Parsed::Malformed));
        assert!(matches!(parse("20240102T030405"), Parsed::Malformed));
        // A number of digits that is not one of the forms of v2.
        assert!(matches!(parse("20240102030405678"), Parsed::Malformed));
        assert!(matches!(parse("20240102"), Parsed::Date(_)));
    }

    #[test]
    fn a_space_after_the_digits_ends_the_date_of_v2() {
        // Measured on the binary of v2.0.19 on 2026-10-28: the text
        // `2024 01 02` is the year of 2024, and the rest of it is not
        // read.
        let Parsed::Date(moment) = parse("2024 01 02") else {
            panic!("the text of a date is not a date");
        };
        assert_eq!(moment.date.year, 2024);
        assert_eq!(moment.date.month, 1);
        assert_eq!(moment.date.day, 1);
    }

    #[test]
    fn a_date_of_v2_carries_the_date_and_the_time_that_it_has() {
        let Parsed::Date(moment) = parse("20240102030405") else {
            panic!("the number of v2 is not a date");
        };
        assert_eq!(moment.date.year, 2024);
        assert_eq!(moment.date.month, 1);
        assert_eq!(moment.date.day, 2);
        assert_eq!(moment.date.hour, 3);
        assert_eq!(moment.date.minute, 4);
        assert_eq!(moment.date.second, 5);
        assert!(moment.date_ok);
        assert!(moment.time_ok);
    }

    #[test]
    fn a_group_of_a_date_that_is_not_a_real_one_writes_nothing() {
        // The year of 304 is not a year that v2 holds, so the date of the
        // text is empty and the time is not there either.
        let Parsed::Date(moment) = parse("030405") else {
            panic!("the number of v2 is not a date");
        };
        assert!(!moment.date_ok);
        assert!(moment.time_ok);
        assert_eq!(format_moment(&moment, "yyyy-MM-dd"), "");
        assert_eq!(format_moment(&moment, "HH:mm:ss"), "00:00:00");
        // The text of the group is not read either: it goes with the
        // group that is not a real one.
        assert_eq!(format_moment(&moment, "[yyyy]"), "");
        // And it is read when the group is a real one.
        let whole = a_moment(2024, 1, 2, 3, 4, 5);
        assert_eq!(format_moment(&whole, "[yyyy]"), "[2024]");
    }

    #[test]
    fn date_add_adds_the_days_the_way_v2_adds_them() {
        // Measured on the binary of v2.0.19 on 2026-10-28:
        // `DateAdd(20240101, 1, "d")` answers `20240102000000`.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240101), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20240102000000".into()));
    }

    #[test]
    fn date_add_goes_over_the_end_of_a_month_and_of_a_year() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240131), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20240201000000".into()));
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20241231), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20250101000000".into()));
    }

    #[test]
    fn date_add_counts_the_leap_year_the_way_v2_counts_it() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240228), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20240229000000".into()));
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20230228), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20230301000000".into()));
    }

    #[test]
    fn date_add_keeps_the_fraction_of_the_amount() {
        // Measured on the binary of v2.0.19 on 2026-10-28: one day and a
        // half is a day and twelve hours.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240101), Value::Float(1.5), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("20240102120000".into()));
    }

    #[test]
    fn a_date_that_v2_cannot_read_is_an_invalid_parameter() {
        // Measured on the binary of v2.0.19 on 2026-10-28: the message is
        // "Parameter #1 of DateAdd is invalid.".
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Text("hello".into()), Value::Int(1), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(
            answer.unwrap_err().message,
            "Parameter #1 of DateAdd is invalid.".to_string()
        );
    }

    #[test]
    fn a_unit_that_v2_does_not_have_is_an_invalid_parameter() {
        // Measured on the binary of v2.0.19 on 2026-10-28: v2 has no
        // unit of years nor of months.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240101), Value::Int(1), Value::Text("y".into())], 1, 0)
        });
        assert_eq!(
            answer.unwrap_err().message,
            "Parameter #3 of DateAdd is invalid.".to_string()
        );
    }

    #[test]
    fn an_amount_that_is_not_a_number_is_refused_the_way_v2_refuses_it() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let answer = with_clock(|rt| {
            date_add(rt, &[Value::Int(20240101), Value::Text("d".into()), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(
            answer.unwrap_err().message,
            "Parameter #2 of DateAdd requires a Number, but received a String.".to_string()
        );
    }

    #[test]
    fn date_diff_answers_the_first_date_minus_the_second() {
        // Measured on the binary of v2.0.19 on 2026-10-28:
        // `DateDiff(20240101, 20240110, "d")` answers `-9`.
        let answer = with_clock(|rt| {
            date_diff(rt, &[Value::Int(20240101), Value::Int(20240110), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Int(-9));
        let answer = with_clock(|rt| {
            date_diff(rt, &[Value::Int(20240110), Value::Int(20240101), Value::Text("d".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Int(9));
    }

    #[test]
    fn date_diff_truncates_towards_zero() {
        // Measured on the binary of v2.0.19 on 2026-10-28: half a day is
        // zero days, and an hour and a half is one hour.
        let answer = with_clock(|rt| {
            date_diff(
                rt,
                &[Value::Int(20240101000000), Value::Int(20240101120000), Value::Text("d".into())],
                1,
                0,
            )
        });
        assert_eq!(answer.unwrap(), Value::Int(0));
        let answer = with_clock(|rt| {
            date_diff(
                rt,
                &[Value::Int(20240101000000), Value::Int(20240101013000), Value::Text("h".into())],
                1,
                0,
            )
        });
        assert_eq!(answer.unwrap(), Value::Int(-1));
    }

    #[test]
    fn format_time_writes_the_moment_the_way_v2_writes_it() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let answer = with_clock(|rt| {
            format_time(rt, &[Value::Int(20240102), Value::Text("yyyy-MM-dd".into())], 1, 0)
        });
        assert_eq!(answer.unwrap(), Value::Text("2024-01-02".into()));
    }

    #[test]
    fn a_date_of_v2_with_a_point_is_not_a_date() {
        // Measured on the binary of v2.0.19 on 2026-10-28: v2 refuses
        // the number with a millisecond.
        let answer = with_clock(|rt| {
            format_time(rt, &[Value::Float(20240102030405.5), Value::Text("yyyy".into())], 1, 0)
        });
        assert_eq!(
            answer.unwrap_err().message,
            "Parameter #1 of FormatTime is invalid.".to_string()
        );
    }

    #[test]
    fn the_variables_of_the_date_answer_the_date_of_the_machine() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let answer = with_clock(|rt| variable_value("now", rt));
        assert_eq!(answer.unwrap(), Value::Text("20240102030405".into()));
        let answer = with_clock(|rt| variable_value("year", rt));
        assert_eq!(answer.unwrap(), Value::Text("2024".into()));
        let answer = with_clock(|rt| variable_value("mon", rt));
        assert_eq!(answer.unwrap(), Value::Text("01".into()));
        let answer = with_clock(|rt| variable_value("mday", rt));
        assert_eq!(answer.unwrap(), Value::Text("02".into()));
        let answer = with_clock(|rt| variable_value("hour", rt));
        assert_eq!(answer.unwrap(), Value::Text("03".into()));
        let answer = with_clock(|rt| variable_value("min", rt));
        assert_eq!(answer.unwrap(), Value::Text("04".into()));
        let answer = with_clock(|rt| variable_value("sec", rt));
        assert_eq!(answer.unwrap(), Value::Text("05".into()));
        let answer = with_clock(|rt| variable_value("msec", rt));
        assert_eq!(answer.unwrap(), Value::Text("678".into()));
        let answer = with_clock(|rt| variable_value("wday", rt));
        assert_eq!(answer.unwrap(), Value::Text("3".into()));
        let answer = with_clock(|rt| variable_value("yday", rt));
        assert_eq!(answer.unwrap(), Value::Text("2".into()));
        let answer = with_clock(|rt| variable_value("yweek", rt));
        assert_eq!(answer.unwrap(), Value::Text("202401".into()));
        let answer = with_clock(|rt| variable_value("mm", rt));
        assert_eq!(answer.unwrap(), Value::Text("01".into()));
        let answer = with_clock(|rt| variable_value("mmm", rt));
        assert_eq!(answer.unwrap(), Value::Text("Jan".into()));
        let answer = with_clock(|rt| variable_value("mmmm", rt));
        assert_eq!(answer.unwrap(), Value::Text("January".into()));
        let answer = with_clock(|rt| variable_value("ddd", rt));
        assert_eq!(answer.unwrap(), Value::Text("Tue".into()));
        let answer = with_clock(|rt| variable_value("dddd", rt));
        assert_eq!(answer.unwrap(), Value::Text("Tuesday".into()));
    }

    #[test]
    fn a_runtime_that_does_not_know_the_date_says_so() {
        let answer = with_bare_clock(|rt| variable_value("now", rt));
        assert_eq!(answer.unwrap_err().message, NO_DATE.to_string());
    }
}
