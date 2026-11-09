//! The text of a float, the way AutoHotkey writes it.
//!
//! A float of the v2 is written with seventeen significant digits, and the
//! trailing zeros of that shape are dropped; a float that is a whole number
//! keeps one point, so `2.0` is not `2`. The rule was measured on the binary
//! of v2.0.19 on 2026-11-09, and the cases are in the tests below and in
//! `egm-review/keysharp-audit/differential/`.
//!
//! The tree used to write the shortest text that reads back as the same
//! float -- the shape Rust gives -- and that is one digit short of what v2
//! writes for a number such as `1/3`. The difference is visible to a script
//! that prints, compares or hashes the text of a float, so it is a defect of
//! the dialect and not a matter of taste.
//!
//! The measured rule is the shape of the C `%.17g` exactly:
//!
//! - seventeen significant digits, and not the shortest text that reads back
//!   as the same float: `1/3` is "0.33333333333333331" and `0.1` is
//!   "0.10000000000000001", while the zeros that the last digit carries are
//!   not written, so `1.5` is "1.5" and `1e-300` is "1e-300";
//! - the plain form while the decimal exponent is from -4 to 16, and the
//!   exponent form outside it: `1e16` is "10000000000000000.0" and `1e17` is
//!   "1e+17", `1e-4` is "0.0001" and `1e-5` is "1.0000000000000001e-05";
//! - the exponent form carries a lower-case `e`, a sign and at least two
//!   digits: "1.5e-05" and "1e+300";
//! - a whole number keeps one point in the plain form -- `2.0` is not `2`,
//!   and `1e16` ends with ".0" -- while the exponent form has no point:
//!   `1e+17` is not "1.0e+17";
//! - an infinity is "inf" or "-inf", and a NaN is "-nan(ind)" or "nan(ind)",
//!   which is what the C library of the binary writes.

use alloc::format;
use alloc::string::{String, ToString};

/// The decimal exponent of the last number that is written in the plain
/// form.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `"" 1e16` is
/// "10000000000000000.0" and `"" 1e17` is "1e+17". The `%g` of the C gives
/// the exponent form from the seventeenth significant digit, and this is
/// the exponent of the digit that is still plain.
const THE_LAST_PLAIN_EXPONENT: i32 = 16;

/// The decimal exponent of the first number that is written in the
/// exponent form.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `"" 1e-4` is "0.0001"
/// and `"" 1e-5` is "1.0000000000000001e-05".
const THE_FIRST_EXPONENT_EXPONENT: i32 = -5;

/// The text of a float, the way v2 writes it.
///
/// The number is written with seventeen significant digits, the zeros that
/// the last digit carries are dropped and a whole number keeps one `.0`, and
/// a magnitude that is too small or too large for the plain form is written
/// with an exponent. The exact rule is measured, and it is in the tests.
pub fn the_text_of(number: f64) -> String {
    // The NaN and the infinities are the texts of the C library of the
    // binary. Measured on 2026-11-09: `Log(0)` is "-inf", `Exp(1000)` is
    // "inf", and an infinity minus an infinity -- the only NaN a script
    // can make -- is "-nan(ind)". The sign of the NaN is the sign bit of
    // the double, and a NaN whose sign is clear is "nan(ind)" in the C
    // library of the machine; that one a script cannot reach, and it is
    // written the same way.
    if number.is_nan() {
        return if number.is_sign_negative() { "-nan(ind)" } else { "nan(ind)" }.to_string();
    }
    if number.is_infinite() {
        return if number < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    // The sign of the zero is part of the text: measured on 2026-11-09,
    // `"" -0.0` is "-0.0".
    let negative = number.is_sign_negative();
    // The seventeen significant digits of the double, and the place of the
    // point. Rust rounds the digits of the exponential form the way the C
    // library of the binary does, even a half that falls on the last of
    // them: measured on 2026-11-09, `"" (1/(2**25))` is
    // "2.9802322387695312e-08".
    let exact = format!("{:.16e}", number.abs());
    let (mantissa, exponent) = exact.split_once('e').expect("the exponential form has an exponent");
    let digits: String = mantissa.chars().filter(|character| character.is_ascii_digit()).collect();
    let exponent: i32 = exponent.parse().expect("the exponent is a number");
    let mut text = String::new();
    if negative {
        text.push('-');
    }
    if exponent <= THE_FIRST_EXPONENT_EXPONENT || exponent > THE_LAST_PLAIN_EXPONENT {
        // The exponent form: the mantissa without the zeros that its last
        // digit carries, then `e`, the sign and at least two digits.
        let mut digits = digits;
        while digits.len() > 1 && digits.ends_with('0') {
            digits.pop();
        }
        text.push_str(&digits[..1]);
        if digits.len() > 1 {
            text.push('.');
            text.push_str(&digits[1..]);
        }
        text.push('e');
        text.push(if exponent < 0 { '-' } else { '+' });
        text.push_str(&format!("{:02}", exponent.abs()));
    } else {
        // The plain form: the point goes after `exponent + 1` digits, and
        // the zeros at the end of the fraction are not written.
        let point = exponent + 1;
        let mut fraction = String::new();
        if point <= 0 {
            text.push('0');
            for _ in 0..-point {
                fraction.push('0');
            }
            fraction.push_str(&digits);
        } else {
            let point = point as usize;
            if point >= digits.len() {
                text.push_str(&digits);
                for _ in 0..point - digits.len() {
                    text.push('0');
                }
            } else {
                text.push_str(&digits[..point]);
                fraction.push_str(&digits[point..]);
            }
        }
        while fraction.ends_with('0') {
            fraction.pop();
        }
        if fraction.is_empty() {
            // A whole number keeps one point: `2.0` is not `2`, and the
            // text of `1e16` is "10000000000000000.0".
            text.push_str(".0");
        } else {
            text.push('.');
            text.push_str(&fraction);
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::the_text_of;

    /// The text that v2 wrote for the number on 2026-11-09, against the
    /// text the tree writes for the same double.
    fn writes(number: f64, expected: &str) {
        assert_eq!(the_text_of(number), expected, "the text of {number:e}");
    }

    #[test]
    fn the_seventeen_digits_are_not_the_shortest_text_that_reads_back() {
        // Measured on the binary of v2.0.19 on 2026-11-09. The shortest
        // text of `1/3` is "0.3333333333333333", and v2 writes one digit
        // more; the same for `0.1`, whose shortest text is "0.1".
        writes(1.0 / 3.0, "0.33333333333333331");
        writes(2.0 / 3.0, "0.66666666666666663");
        writes(1.0 - 0.9, "0.099999999999999978");
        writes(0.1, "0.10000000000000001");
        writes(0.30000000000000004, "0.30000000000000004");
        writes(0.3333333333333333, "0.33333333333333331");
        writes(1.2345678901234567, "1.2345678901234567");
        writes(1.0000000000000002, "1.0000000000000002");
        writes(0.00012345, "0.00012344999999999999");
        writes(0.1 + 0.7, "0.79999999999999993");
        writes(1234567890.1234567, "1234567890.1234567");
        writes(core::f64::consts::PI, "3.1415926535897931");
        writes(core::f64::consts::E, "2.7182818284590451");
        writes(0.1 * 3.0, "0.30000000000000004");
        writes(10.0 / 3.0, "3.3333333333333335");
        writes(7.0 / 11.0, "0.63636363636363635");
        writes(100.0 / 7.0, "14.285714285714286");
        writes(1.0 / 9.0, "0.1111111111111111");
        writes(1e-4 + 1e-20, "0.00010000000000000002");
        writes(1.0000000000000002e-4, "0.00010000000000000002");
        writes(1e-5 + 1e-21, "1.0000000000000003e-05");
        writes(2.0e-5, "2.0000000000000002e-05");
        writes(0.00009999999999999999, "9.9999999999999991e-05");
        writes(9.999999999999999e-05, "9.9999999999999991e-05");
        writes(0.9999999999999999e-4, "9.9999999999999991e-05");
        writes(1e-4 * 0.9999999999999999, "9.9999999999999991e-05");
        writes(1e-6, "9.9999999999999995e-07");
        writes(1e-7, "9.9999999999999995e-08");
        writes(1.0 / 67108864.0, "1.4901161193847656e-08");
        writes(1.0 / 1073741824.0, "9.3132257461547852e-10");
        writes(1.7e-5, "1.7e-05");
        writes(1.0e-4 / 3.0, "3.3333333333333335e-05");
        writes(123456789012345678.0, "1.2345678901234568e+17");
        writes(1e100 + 1e84, "1.0000000000000002e+100");
        writes(1e100 * 1e100, "9.9999999999999997e+199");
        writes(1e20 + 1e4, "1.0000000000000002e+20");
        writes(18446744073709551616.0, "1.8446744073709552e+19");
    }

    #[test]
    fn the_zeros_that_the_last_digit_carries_are_not_written() {
        // Measured on the binary of v2.0.19 on 2026-11-09: the seventeen
        // digits are the shape, and `%g` drops the zeros at the end of it.
        writes(1.5, "1.5");
        writes(-0.5, "-0.5");
        writes(0.5, "0.5");
        writes(2.5, "2.5");
        writes(1.5e-5, "1.5e-05");
        writes(1e-300, "1e-300");
        writes(1e-4, "0.0001");
        writes(1.0 / 1024.0, "0.0009765625");
        writes(1.0 / 16777216.0, "5.9604644775390625e-08");
        writes(1.0 / 1048576.0, "9.5367431640625e-07");
    }

    #[test]
    fn a_whole_number_keeps_one_point_in_the_plain_form() {
        // Measured on the binary of v2.0.19 on 2026-11-09: `"" 2.0` is
        // "2.0" and `"" 1e16` is "10000000000000000.0". The point is kept
        // to the last whole number the plain form holds, and the exponent
        // form has none: `"" 1e17` is "1e+17".
        writes(2.0, "2.0");
        writes(-2.0, "-2.0");
        writes(0.0, "0.0");
        writes(-0.0, "-0.0");
        writes(-0.0 * 1.0, "-0.0");
        writes(100.0 / 4.0, "25.0");
        writes(3.0, "3.0");
        writes(1e10, "10000000000.0");
        writes(-1e10, "-10000000000.0");
        writes(1e15, "1000000000000000.0");
        writes(1e16, "10000000000000000.0");
        writes(9.9e16, "99000000000000000.0");
        writes(1e16 + 2.0, "10000000000000002.0");
        writes(1e16 + 1.0, "10000000000000000.0");
        writes(9.999999999999999e16, "99999999999999984.0");
        writes(16777216.0, "16777216.0");
        writes(33554432.0, "33554432.0");
        writes(4503599627370496.0, "4503599627370496.0");
        writes(4503599627370497.0, "4503599627370497.0");
        writes(9007199254740990.0, "9007199254740990.0");
        writes(9007199254740992.0, "9007199254740992.0");
        writes(9007199254740994.0, "9007199254740994.0");
        writes(123456789012345.0, "123456789012345.0");
        writes(1234567890123456.0, "1234567890123456.0");
        writes(12345678901234568.0, "12345678901234568.0");
        writes(1e300 * 1e-300, "1.0");
        writes(100000000000000000000.0, "1e+20");
        writes(99999999999999999.0, "1e+17");
    }

    #[test]
    fn the_plain_form_holds_from_the_fourth_place_to_the_seventeenth_digit() {
        // Measured on the binary of v2.0.19 on 2026-11-09: the plain form
        // goes down to 1e-4 and up to 1e16; from 1e-5 and from 1e17 the
        // exponent form is the one used, and the exponent has a sign and
        // at least two digits.
        writes(1e17, "1e+17");
        writes(1e18, "1e+18");
        writes(1e19, "1e+19");
        writes(1e20, "1e+20");
        writes(1e21, "1e+21");
        writes(1e22, "1e+22");
        writes(1e22 + 1.0, "1e+22");
        writes(1.0000000000000002e17, "1.0000000000000002e+17");
        writes(1e300, "1.0000000000000001e+300");
        writes(-1e300, "-1.0000000000000001e+300");
        writes(1e-5, "1.0000000000000001e-05");
        writes(-1e-5, "-1.0000000000000001e-05");
        writes(4.9e-324, "4.9406564584124654e-324");
        writes(2.2250738585072014e-308, "2.2250738585072014e-308");
        writes(1.7976931348623157e308, "1.7976931348623157e+308");
        writes(1.7976931348623155e308, "1.7976931348623155e+308");
        writes(4611686018427387904.0, "4.6116860184273879e+18");
        writes(9223372036854775808.0, "9.2233720368547758e+18");
        writes(1e15 + 0.5, "1000000000000000.5");
        writes(1234567890123456.5, "1234567890123456.5");
    }

    #[test]
    fn a_tie_on_the_seventeenth_digit_rounds_the_way_the_c_library_does() {
        // Measured on the binary of v2.0.19 on 2026-11-09. The exact value
        // of `1/(2**25)` ends in "...3125", and the seventeenth digit is
        // the one the tie falls on: the C library rounds it to the even
        // digit, and so does Rust.
        writes(1.0 / 33554432.0, "2.9802322387695312e-08");
        writes(3.0 / 33554432.0, "8.9406967163085938e-08");
        writes(5.0 / 33554432.0, "1.4901161193847656e-07");
        writes(7.0 / 33554432.0, "2.0861625671386719e-07");
        writes(9.0 / 33554432.0, "2.6822090148925781e-07");
    }

    #[test]
    fn the_infinity_and_the_nan_are_the_texts_of_the_c_library() {
        // Measured on the binary of v2.0.19 on 2026-11-09: `Exp(1000)` is
        // "inf", `Log(0)` is "-inf", and an infinity minus an infinity is
        // the only NaN a script can make, and its text is "-nan(ind)". A
        // NaN whose sign is clear is the C library's "nan(ind)"; a script
        // cannot reach that one, and the tree writes it the same way.
        writes(f64::INFINITY, "inf");
        writes(f64::NEG_INFINITY, "-inf");
        writes(f64::NAN, "nan(ind)");
        writes(f64::from_bits(0xfff8_0000_0000_0000), "-nan(ind)");
    }
}