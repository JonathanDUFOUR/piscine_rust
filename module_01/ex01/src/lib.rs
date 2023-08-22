/// Get the smallest value among two numbers.
///
/// # Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Examples
/// ```
/// use::ex01::min;
///
/// const A: i32 = 1;
/// const B: i32 = 2;
///
/// assert_eq!(min(&A, &B), &A);
/// assert_eq!(min(&B, &A), &A);
/// ```
#[allow(dead_code)]
pub fn min<'lifetime>(a: &'lifetime i32, b: &'lifetime i32) -> &'lifetime i32 {
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
