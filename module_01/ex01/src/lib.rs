/// Compares two numbers and returns the lowest one.
///
/// ### Parameters
/// * `a` - The first number to compare.
/// * `b` - The second number to compare.
///
/// ### Return
/// The lowest number between `a` and `b`.
///
/// ### Example
/// ```
/// use::ex01::min;
///
/// const A: i32 = 1;
/// const B: i32 = 2;
///
/// assert_eq!(min(&A, &B), &A);
/// assert_eq!(min(&B, &A), &A);
/// ```
pub fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
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
		const A: i32 = 0;
		const B: i32 = 0;

		assert_eq!(min(&A, &B), &A);
		assert_eq!(min(&A, &B), &B);
	}

	#[test]
	fn min_01() {
		const A: i32 = 0;
		const B: i32 = 1;

		assert_eq!(min(&A, &B), &A);
	}

	#[test]
	fn min_02() {
		const A: i32 = 1;
		const B: i32 = 0;

		assert_eq!(min(&A, &B), &B);
	}

	#[test]
	fn min_03() {
		const A: i32 = -42;
		const B: i32 = 42;

		assert_eq!(min(&A, &B), &A);
	}

	#[test]
	fn min_04() {
		const A: i32 = 42;
		const B: i32 = -42;

		assert_eq!(min(&A, &B), &B);
	}

	#[test]
	fn min_05() {
		const A: i32 = i32::MIN;
		const B: i32 = i32::MAX;

		assert_eq!(min(&A, &B), &A);
	}
}
