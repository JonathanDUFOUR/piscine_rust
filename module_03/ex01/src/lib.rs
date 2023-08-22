/// Get the smallest value among two of any type.
///
/// # Generics
///
/// * `T` - The type of the two values to compare.
///
/// # Arguments
///
/// * `a` - The first thing.
/// * `b` - The second thing.
///
/// # Examples
/// ```
/// use::ex01::min;
///
/// const A: &str = "baba";
/// const B: &str = "bababoï";
///
/// assert_eq!(min(A, B), A);
/// assert_eq!(min(B, A), A);
/// ```
#[allow(dead_code)]
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

	#[test]
	fn min_00() {
		assert_eq!(min(0u8, 0u8), 0u8);
	}

	#[test]
	fn min_01() {
		assert_eq!(min(0i8, -1i8), -1i8);
	}

	#[test]
	fn min_02() {
		assert_eq!(min(2u16, 1u16), 1u16);
	}

	#[test]
	fn min_03() {
		assert_eq!(min(-123i16, -321i16), -321i16);
	}

	#[test]
	fn min_04() {
		assert_eq!(min(42u32, 21u32), 21u32);
	}

	#[test]
	fn min_05() {
		assert_eq!(min(i32::MAX, -1i32), -1i32);
	}

	#[test]
	fn min_06() {
		assert_eq!(min(0u64, u64::MAX), 0u64);
	}

	#[test]
	fn min_07() {
		assert_eq!(min(i64::MIN, i64::MAX), i64::MIN);
	}

	#[test]
	fn min_08() {
		assert_eq!(min(123456789u128, 987654321u128), 123456789u128);
	}

	#[test]
	fn min_09() {
		assert_eq!(min(i128::MAX, i128::MIN), i128::MIN);
	}

	#[test]
	fn min_10() {
		assert_eq!(min(0usize, 42usize), 0usize);
	}

	#[test]
	fn min_11() {
		assert_eq!(min(-1isize, 1isize), -1isize);
	}

	#[test]
	fn min_12() {
		assert_eq!(min(f32::INFINITY, f32::EPSILON), f32::EPSILON);
	}

	#[test]
	fn min_13() {
		assert_eq!(min(f64::NEG_INFINITY, f64::MIN), f64::NEG_INFINITY);
	}

	#[test]
	fn min_14() {
		assert_eq!(min(true, false), false);
	}

	#[test]
	fn min_15() {
		assert_eq!(min('a', 'z'), 'a');
	}

	#[test]
	fn min_16() {
		assert_eq!(min("Hello", "World!"), "Hello");
	}

	#[test]
	fn min_17() {
		assert_eq!(
			min("abc".to_string(), "abcd".to_string()),
			"abc".to_string()
		);
	}
}
