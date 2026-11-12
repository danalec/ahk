//! The float functions of the language, for a host with no `std`.
//!
//! The functions a script asks for -- `Sqrt`, `Floor`, `Ceil`, `Round`,
//! `Exp`, `Log`, `Sqrt`, the trigonometry, and the power -- live in `std`,
//! where they call the platform's library. A target with no `std` has no
//! library to call, and `core` carries only `abs`, so this module is the
//! answer for that target.
//!
//! The implementations are the ones every libm carries, written from their
//! definitions rather than from the source of a particular one: a reduction
//! that brings the argument into a small range, a polynomial that is accurate
//! on that range, and the assembly of the answer out of the two. The numbers
//! are the same as the ones `std` gives for the arguments a script uses, and
//! the cases at the bottom of this file pin the ones that matter.
//!
//! Nothing here is compiled when the crate has `std`: the helpers of
//! `runtime.rs` call the platform there, which is both faster and exact, and
//! this module is the fallback of a host that has nothing to call.
/// The square root of `value`.
///
/// The answer is the Newton iteration on the estimate the exponent of the
/// number gives: the first guess takes the exponent in half, and three turns
/// of `x = (x + value / x) / 2` take it to the last bit.
pub fn sqrt(value: f64) -> f64 {
    // The square root of something that is not positive is not a number the
    // platform answers with a value.
    if value.is_nan() || value < 0.0 {
        return f64::NAN;
    }
    if value == 0.0 || value.is_infinite() {
        return value;
    }
    // The guess from the exponent, and the turns of the recurrence. A value
    // of zero is done, and a value that is infinite answers itself.
    let mut guess = f64::from_bits((value.to_bits() >> 1) + (1023u64 << 51));
    for _ in 0..5 {
        guess = 0.5 * (guess + value / guess);
    }
    guess
}

/// The largest whole number that is not greater than `value`.
pub fn floor(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    let whole = trunc(value);
    // A value that is exactly a whole number is its own floor, and one that
    // is below the number it would have gone to goes one down.
    if whole > value { whole - 1.0 } else { whole }
}

/// The smallest whole number that is not less than `value`.
pub fn ceil(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    let whole = trunc(value);
    if whole < value { whole + 1.0 } else { whole }
}

/// The whole part of `value`: the number without its fraction.
pub fn trunc(value: f64) -> f64 {
    if !value.is_finite() || value.abs() >= 4503599627370496.0 {
        // A number that large has no fraction to take away, and infinity and
        // a not-a-number answer themselves.
        return value;
    }
    // The cast to an integer is exact for a value this small, and it is the
    // truncation the function asks for.
    let whole = value as i64;
    whole as f64
}

/// The fraction of `value`, with the sign of the value.
pub fn fract(value: f64) -> f64 {
    if !value.is_finite() {
        return f64::NAN;
    }
    value - trunc(value)
}

/// The nearest whole number, with a half going away from zero.
///
/// AutoHotkey rounds a half up, and a negative half away from zero, which is
/// what the reference implementation does.
pub fn round(value: f64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    let floor = floor(value);
    let difference = value - floor;
    // Half goes away from zero: above the half the whole number above, below
    // it the one below, and at the half the one that is further from zero.
    if difference > 0.5 || (difference == 0.5 && value > 0.0) {
        floor + 1.0
    } else {
        floor
    }
}

/// The constant pi, to the last bit of a double.
///
/// It is `core`'s, and not a literal here: the value a script's `Sqrt` and
/// `Sin` are built on is the one the platform uses, and a literal would be a
/// second copy of it to keep in step.
const PI: f64 = core::f64::consts::PI;

/// The natural logarithm of two.
const LN_2: f64 = core::f64::consts::LN_2;

/// The inverse of the natural logarithm of ten, for `log10`.
const LOG10_E: f64 = core::f64::consts::LOG10_E;

/// The square root of two, for the range of the logarithm.
const SQRT_2: f64 = core::f64::consts::SQRT_2;

/// The result of `value` raised to `exponent`.
///
/// The whole exponents are the repeated multiplication, which is exact for
/// the ones a script uses, and the others go through the logarithm and the
/// exponential, which is what the platform does as well.
pub fn pow(base: f64, exponent: f64) -> f64 {
    if exponent == 0.0 {
        return 1.0;
    }
    if base == 0.0 {
        return if exponent > 0.0 { 0.0 } else { f64::INFINITY };
    }
    if base < 0.0 {
        // A negative base with a fractional exponent has no real answer.
        if exponent != trunc(exponent) {
            return f64::NAN;
        }
        let magnitude = exp(exponent * ln(-base));
        let odd = (exponent % 2.0) != 0.0;
        return if odd { -magnitude } else { magnitude };
    }
    exp(exponent * ln(base))
}

/// The result of `value` raised to a whole `exponent`, by squaring.
pub fn powi(value: f64, exponent: i32) -> f64 {
    let mut result = 1.0f64;
    let mut base = value;
    let mut power = exponent.unsigned_abs();
    while power > 0 {
        if power & 1 == 1 {
            result *= base;
        }
        base *= base;
        power >>= 1;
    }
    if exponent < 0 { 1.0 / result } else { result }
}

/// The exponential of `value`: e to the power of it.
///
/// The argument is halved until it is small, the polynomial of the small
/// argument is evaluated, and the answer is squared back up: the same shape
/// every libm uses, because it is the one that keeps the accuracy.
pub fn exp(value: f64) -> f64 {
    if value.is_nan() {
        return f64::NAN;
    }
    if value > 709.782712893384 {
        return f64::INFINITY;
    }
    if value < -745.1332191019411 {
        return 0.0;
    }
    // The whole part of the argument, which is the power of two the answer
    // is scaled by, and the fraction, which the polynomial answers.
    let turns = round(value / LN_2);
    let rest = value - turns * LN_2;
    // The Taylor series of the exponential of a small argument, to the term
    // where the terms are smaller than the last bit of the answer.
    let mut term = 1.0f64;
    let mut sum = 1.0f64;
    for step in 1..=22 {
        term *= rest / step as f64;
        sum += term;
    }
    // Scale by two to the power of the turns, which is exact.
    sum * f64::from_bits(((1023 + turns as i64) as u64) << 52)
}

/// The natural logarithm of `value`.
pub fn ln(value: f64) -> f64 {
    if value.is_nan() || value < 0.0 {
        return f64::NAN;
    }
    if value == 0.0 {
        return f64::NEG_INFINITY;
    }
    if value.is_infinite() {
        return f64::INFINITY;
    }
    // The exponent of the number, which is the logarithm of the power of two
    // in it, and the mantissa, which is brought to the range of the series.
    let bits = value.to_bits();
    let mut exponent = ((bits >> 52) & 0x7FF) as i64 - 1023;
    let mut mantissa = f64::from_bits((bits & 0x000F_FFFF_FFFF_FFFF) | (1023u64 << 52));
    if mantissa > SQRT_2 {
        mantissa /= 2.0;
        exponent += 1;
    }
    // The series of `ln(1 + t)` with `t` the mantissa less one, which the
    // range above keeps small enough to converge in a few dozen terms.
    let t = (mantissa - 1.0) / (mantissa + 1.0);
    let square = t * t;
    let mut term = t;
    let mut sum = 0.0f64;
    let mut divisor = 1.0f64;
    for _ in 0..30 {
        sum += term / divisor;
        term *= square;
        divisor += 2.0;
    }
    2.0 * sum + exponent as f64 * LN_2
}

/// The logarithm of `value` in base ten.
pub fn log10(value: f64) -> f64 {
    // The constant is `1 / ln(10)`, to the last bit of a double.
    ln(value) * LOG10_E
}

/// The sine of `value`, in radians.
pub fn sin(value: f64) -> f64 {
    if !value.is_finite() {
        return f64::NAN;
    }
    let (rest, quadrant) = the_quarter(value);
    let sine = the_sine_of_small(rest);
    let cosine = the_cosine_of_small(rest);
    match quadrant {
        0 => sine,
        1 => cosine,
        2 => -sine,
        _ => -cosine,
    }
}

/// The cosine of `value`, in radians.
pub fn cos(value: f64) -> f64 {
    if !value.is_finite() {
        return f64::NAN;
    }
    let (rest, quadrant) = the_quarter(value);
    let sine = the_sine_of_small(rest);
    let cosine = the_cosine_of_small(rest);
    match quadrant {
        0 => cosine,
        1 => -sine,
        2 => -cosine,
        _ => sine,
    }
}

/// Bring an angle into a quarter turn, and say which quarter it was in.
///
/// The answer is a value in `[0, pi/2]` and the number of the quarter, so
/// that the caller can turn the sine or the cosine back with the same value.
fn the_quarter(value: f64) -> (f64, u8) {
    // How many quarter turns the angle holds, rounded to the nearest. The
    // remainder is in `[-pi/4, pi/4]`, and it is folded into `[0, pi/2]` by
    // going one quarter further when it is behind.
    let quarters = round(value / (PI / 2.0));
    let rest = value - quarters * (PI / 2.0);
    let base = (quarters as i64).rem_euclid(4);
    if rest < 0.0 {
        // The angle is the quarter before, plus what is missing from this
        // one: `q - pi/2 + rest` is the same angle as `(q - 1) + (pi/2 + rest)`.
        (PI / 2.0 + rest, ((base + 3).rem_euclid(4)) as u8)
    } else {
        (rest, base as u8)
    }
}

/// The sine of an argument that is already small.
///
/// The polynomial is the one of the sine on the range that a quarter turn
/// gives, and the terms run to the one that is smaller than the last bit of a
/// double.
fn the_sine_of_small(value: f64) -> f64 {
    let square = value * value;
    let mut term = value;
    let mut sum = value;
    for step in 1..=14 {
        term *= -square / (((2 * step) * (2 * step + 1)) as f64);
        sum += term;
    }
    sum
}

/// The cosine of an argument that is already small.
fn the_cosine_of_small(value: f64) -> f64 {
    let square = value * value;
    let mut term = 1.0f64;
    let mut sum = 1.0f64;
    for step in 1..=14 {
        term *= -square / (((2 * step - 1) * (2 * step)) as f64);
        sum += term;
    }
    sum
}

/// The tangent of `value`, in radians.
pub fn tan(value: f64) -> f64 {
    let sine = sin(value);
    let cosine = cos(value);
    if cosine == 0.0 {
        // The tangent has no value at a quarter of a turn, and the platform
        // answers with a very large number rather than a failure.
        return if sine > 0.0 {
            f64::INFINITY
        } else {
            f64::NEG_INFINITY
        };
    }
    sine / cosine
}

/// The arc whose sine is `value`, in radians.
pub fn asin(value: f64) -> f64 {
    if value.is_nan() || value.abs() > 1.0 {
        return f64::NAN;
    }
    if value == 1.0 {
        return PI / 2.0;
    }
    if value == -1.0 {
        return -PI / 2.0;
    }
    // The identity that turns the arcsine into the arctangent, which is the
    // one that has a series converging on this range.
    atan(value / sqrt(1.0 - value * value))
}

/// The arc whose cosine is `value`, in radians.
pub fn acos(value: f64) -> f64 {
    if value.is_nan() || value.abs() > 1.0 {
        return f64::NAN;
    }
    // The arccosine is the arcsine turned round.
    PI / 2.0 - asin(value)
}

/// The arc whose tangent is `value`, in radians.
pub fn atan(value: f64) -> f64 {
    if value.is_nan() {
        return f64::NAN;
    }
    if value == 0.0 {
        return value;
    }
    if value.is_infinite() {
        return if value > 0.0 { PI / 2.0 } else { -PI / 2.0 };
    }
    // A very large argument is the arctangent of its reciprocal, turned round.
    if value.abs() > 1.0 {
        let turned = atan(1.0 / value.abs());
        return if value > 0.0 {
            PI / 2.0 - turned
        } else {
            turned - PI / 2.0
        };
    }
    // The argument is now in `[0, 1]`. The identity below halves it again, so
    // that the series converges quickly and accurately on what is left.
    let sign = if value < 0.0 { -1.0 } else { 1.0 };
    let mut magnitude = value.abs();
    let mut double = 0u32;
    while magnitude > 0.2679491924311227 {
        // `tan(a/2) = tan(a) / (1 + sqrt(1 + tan(a)^2))`, and the series below
        // is accurate on the small argument it leaves.
        magnitude = magnitude / (1.0 + sqrt(1.0 + magnitude * magnitude));
        double += 1;
    }
    let square = magnitude * magnitude;
    let mut term = magnitude;
    let mut sum = magnitude;
    for step in 1..=20 {
        term *= -square;
        sum += term / (2 * step + 1) as f64;
    }
    sign * sum * (1u32 << double) as f64
}

/// The arc whose tangent is `y / x`, in the quadrant the signs name.
///
/// The two axes and the two diagonals of the quadrant map are the cases the
/// library carries; the rest is the arctangent of the quotient, turned
/// round when the cosine of the quadrant is negative.
pub fn atan2(y: f64, x: f64) -> f64 {
    if y.is_nan() || x.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    if x == 0.0 {
        return if y > 0.0 { PI / 2.0 } else { -PI / 2.0 };
    }
    if y == 0.0 {
        return if x > 0.0 { 0.0 } else { PI };
    }
    let quotient = atan(y / x);
    if x > 0.0 {
        quotient
    } else if y > 0.0 {
        PI + quotient
    } else {
        quotient - PI
    }
}

/// The hyperbolic sine of `value`.
pub fn sinh(value: f64) -> f64 {
    let raised = exp(value);
    (raised - 1.0 / raised) / 2.0
}

/// The hyperbolic cosine of `value`.
pub fn cosh(value: f64) -> f64 {
    let raised = exp(value);
    (raised + 1.0 / raised) / 2.0
}

/// The hyperbolic tangent of `value`.
pub fn tanh(value: f64) -> f64 {
    let raised = exp(2.0 * value);
    (raised - 1.0) / (raised + 1.0)
}

#[cfg(test)]
mod tests {
    use super::{
        acos, asin, atan, ceil, cos, exp, floor, fract, ln, log10, pow, powi, round, sin, sqrt,
        tan, trunc,
    };

    /// A value and the answer of the reference, to the last few bits.
    fn close(answer: f64, expected: f64) {
        let tolerance = expected.abs() * 1e-12 + 1e-12;
        assert!(
            (answer - expected).abs() <= tolerance,
            "the answer {answer} is not {expected}"
        );
    }

    #[test]
    fn the_whole_part_is_the_number_without_its_fraction() {
        assert_eq!(trunc(2.7), 2.0);
        assert_eq!(trunc(-2.7), -2.0);
        assert_eq!(trunc(0.0), 0.0);
        assert_eq!(trunc(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn the_floor_and_the_ceiling_are_the_whole_numbers_beside_it() {
        assert_eq!(floor(2.7), 2.0);
        assert_eq!(floor(-2.1), -3.0);
        assert_eq!(ceil(2.1), 3.0);
        assert_eq!(ceil(-2.7), -2.0);
        assert_eq!(floor(3.0), 3.0);
        assert_eq!(ceil(3.0), 3.0);
    }

    #[test]
    fn the_round_takes_a_half_away_from_zero() {
        assert_eq!(round(2.4), 2.0);
        assert_eq!(round(2.5), 3.0);
        assert_eq!(round(-2.5), -3.0);
        assert_eq!(round(2.6), 3.0);
    }

    #[test]
    fn the_fraction_keeps_the_sign_of_the_value() {
        close(fract(2.75), 0.75);
        close(fract(-2.75), -0.75);
    }

    #[test]
    fn the_square_root_is_the_one_of_the_reference() {
        for value in [0.0, 1.0, 2.0, 4.0, 9.0, 0.25, 1e10, 1e-10] {
            close(sqrt(value), value.sqrt());
        }
        assert!(sqrt(-1.0).is_nan());
        assert_eq!(sqrt(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn the_exponential_and_the_logarithm_are_the_ones_of_the_reference() {
        for value in [0.0, 1.0, -1.0, 2.5, 10.0, -10.0] {
            close(exp(value), value.exp());
        }
        for value in [1.0, 2.0, 10.0, 0.5, 100.0] {
            close(ln(value), value.ln());
            close(log10(value), value.log10());
        }
    }

    #[test]
    fn the_trigonometry_is_the_one_of_the_reference() {
        for value in [0.0, 0.5, 1.0, -1.0, 3.0] {
            close(sin(value), value.sin());
            close(cos(value), value.cos());
            close(tan(value), value.tan());
        }
        for value in [0.0, 0.5, -0.5, 1.0, -1.0] {
            close(asin(value), value.asin());
            close(acos(value), value.acos());
        }
        for value in [0.0, 0.5, -0.5, 1.0, 2.0, 100.0, -100.0] {
            close(atan(value), value.atan());
        }
    }

    #[test]
    fn the_power_is_the_one_of_the_reference() {
        for (base, exponent) in [(2.0, 3.0), (2.0, 0.5), (9.0, 0.5), (2.0, -1.0), (2.0, 0.0)] {
            close(pow(base, exponent), base.powf(exponent));
        }
        assert!(pow(-2.0, 0.5).is_nan());
        close(pow(-2.0, 3.0), -8.0);
        close(powi(2.0, 10), 1024.0);
        close(powi(2.0, -2), 0.25);
    }
}
