//! The date of the v1.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the date
//! of the v1 is the number of the year, the month, the day, the hour, the
//! minute and the second -- the `YYYYMMDDHH24MISS` -- and the groups that
//! are not there are not read: the "20200101" is the first of January of
//! 2020, and the "202001011" is the first of January of 2020 at one in the
//! morning.
//!
//! The date that is not one is refused: the year is before 1601 or after
//! 9999, the month is not one of the twelve, the day is not one that the
//! month has, the hour is above 23, and the minute and the second are above
//! 59. The text that is not a number is not a date, and the v1 does not
//! refuse it: the `EnvSub` lets the "abc" through, and the answer of it is
//! zero.

use crate::date::{days_in_month, day_of_week, day_of_year, DateTime};

/// The date of the v1 that the text holds.
///
/// The text that is not a number of days is not a date, and the answer of
/// the caller is then zero.
pub(crate) fn the_date_of_the_v1(text: &str) -> Option<DateTime> {
    let text = text.trim();
    if text.is_empty() || !text.chars().all(|letter| letter.is_ascii_digit()) {
        return None;
    }
    let digits = text.as_bytes();
    // The date has at least the year, the month and the day.
    if digits.len() < 8 {
        return None;
    }
    let year = the_number_of_the_digits(digits, 0, 4);
    let month = the_number_of_the_digits(digits, 4, 2);
    let day = the_number_of_the_digits(digits, 6, 2);
    // The groups that are not there are not read, and the group that is
    // half there is the one that is written: the "202001011" is the hour one.
    let hour = the_number_of_the_digits(digits, 8, digits.len().saturating_sub(8).min(2));
    let minute = the_number_of_the_digits(digits, 10, digits.len().saturating_sub(10).min(2));
    let second = the_number_of_the_digits(digits, 12, digits.len().saturating_sub(12).min(2));
    if !(1601..=9999).contains(&year) || !(1..=12).contains(&month) {
        return None;
    }
    if day < 1 || day > days_in_month(year, month) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    Some(DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond: 0,
        weekday: day_of_week(year, month, day),
        yday: day_of_year(year, month, day),
    })
}

/// The number of the digits of a place of the text.
fn the_number_of_the_digits(digits: &[u8], at: usize, count: usize) -> i64 {
    let mut number = 0;
    for digit in digits.iter().skip(at).take(count) {
        number = number * 10 + (digit - b'0') as i64;
    }
    number
}

/// Whether the text is a number of days that the v1 would read as a date.
///
/// The text that is not a number is not a date, and the v1 does not refuse
/// it: only the text that is a number and is not a date is refused.
pub(crate) fn the_text_is_a_number_of_days(text: &str) -> bool {
    let text = text.trim();
    !text.is_empty() && text.chars().all(|letter| letter.is_ascii_digit())
}
