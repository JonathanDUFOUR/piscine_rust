/// Compares two things and returns the lowest one.
///
/// # Type parameters
/// * `T` - The type of the two values to compare.
///
/// # Parameters
/// * `a` - The first thing to compare.
/// * `b` - The second thing to compare.
///
/// # Return
/// The lowest thing between `a` and `b`.
///
/// # Example
/// ```
/// use::ex01::min;
///
/// const A: &str = "baba";
/// const B: &str = "bababoï";
///
/// assert_eq!(min(A, B), A);
/// assert_eq!(min(B, A), A);
/// ```
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// region: min_00
	#[test]
	fn min_00() {
		assert_eq!(min(0u8, 0u8), 0u8);
	}
	// endregion

	// region: min_01
	#[test]
	fn min_01() {
		assert_eq!(min(0i8, -1i8), -1i8);
	}
	// endregion

	// region: min_02
	#[test]
	fn min_02() {
		assert_eq!(min(2u16, 1u16), 1u16);
	}
	// endregion

	// region: min_03
	#[test]
	fn min_03() {
		assert_eq!(min(-123i16, -321i16), -321i16);
	}
	// endregion

	// region: min_04
	#[test]
	fn min_04() {
		assert_eq!(min(42u32, 21u32), 21u32);
	}
	// endregion

	// region: min_05
	#[test]
	fn min_05() {
		assert_eq!(min(i32::MAX, -1i32), -1i32);
	}
	// endregion

	// region: min_06
	#[test]
	fn min_06() {
		assert_eq!(min(0u64, u64::MAX), 0u64);
	}
	// endregion

	// region: min_07
	#[test]
	fn min_07() {
		assert_eq!(min(i64::MIN, i64::MAX), i64::MIN);
	}
	// endregion

	// region: min_08
	#[test]
	fn min_08() {
		assert_eq!(min(123456789u128, 987654321u128), 123456789u128);
	}
	// endregion

	// region: min_09
	#[test]
	fn min_09() {
		assert_eq!(min(i128::MAX, i128::MIN), i128::MIN);
	}
	// endregion

	// region: min_10
	#[test]
	fn min_10() {
		assert_eq!(min(0usize, 42usize), 0usize);
	}
	// endregion

	// region: min_11
	#[test]
	fn min_11() {
		assert_eq!(min(-1isize, 1isize), -1isize);
	}
	// endregion

	// region: min_12
	#[test]
	fn min_12() {
		assert_eq!(min(f32::INFINITY, f32::EPSILON), f32::EPSILON);
	}
	// endregion

	// region: min_13
	#[test]
	fn min_13() {
		assert_eq!(min(f64::NEG_INFINITY, f64::MIN), f64::NEG_INFINITY);
	}
	// endregion

	// region: min_14
	#[test]
	fn min_14() {
		assert_eq!(min(true, false), false);
	}
	// endregion

	// region: min_15
	#[test]
	fn min_15() {
		assert_eq!(min('a', 'z'), 'a');
	}
	// endregion

	// region: min_16
	#[test]
	fn min_16() {
		assert_eq!(min("Hello", "World!"), "Hello");
	}
	// endregion

	// region: min_17
	#[test]
	fn min_17() {
		assert_eq!(min("abc".to_string(), "abcd".to_string()), "abc".to_string());
	}
	// endregion
}
