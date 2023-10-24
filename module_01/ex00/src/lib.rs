/// Adds two numbers together.
///
/// # Parameters
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
/// The sum of `a` and `b`.
///
/// # Example
/// ```
/// use ex00::add;
///
/// assert_eq!(add(&2, 3), 5);
/// ```
pub fn add(a: &i32, b: i32) -> i32 {
	return a + b;
}

/// Adds two numbers together, and store the result in the first given argument.
///
/// # Parameters
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Examples
/// ```
/// use ex00::add_assign;
///
/// let mut a: i32 = 2;
///
/// add_assign(&mut a, 3);
/// assert_eq!(a, 5);
/// ```
pub fn add_assign(a: &mut i32, b: i32) {
	*a += b;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn add_00() {
		assert_eq!(add(&0, 0), 0);
	}

	#[test]
	fn add_01() {
		assert_eq!(add(&0, 1), 1);
	}

	#[test]
	fn add_02() {
		assert_eq!(add(&1, 0), 1);
	}

	#[test]
	fn add_03() {
		assert_eq!(add(&1, 1), 2);
	}

	#[test]
	fn add_04() {
		assert_eq!(add(&-1, 1), 0);
	}

	#[test]
	fn add_05() {
		assert_eq!(add(&1, -1), 0);
	}

	#[test]
	fn add_06() {
		assert_eq!(add(&-1, -1), -2);
	}

	#[test]
	fn add_07() {
		assert_eq!(add(&i32::MAX, 0), i32::MAX);
	}

	#[test]
	fn add_08() {
		assert_eq!(add(&i32::MIN, 0), i32::MIN);
	}

	#[test]
	fn add_09() {
		assert_eq!(add(&i32::MAX, i32::MIN), -1);
	}

	#[test]
	fn add_10() {
		assert_eq!(add(&i32::MIN, i32::MAX), -1);
	}

	#[test]
	fn add_assign_00() {
		let mut a: i32 = 0;

		add_assign(&mut a, 0);
		assert_eq!(a, 0);
	}

	#[test]
	fn add_assign_01() {
		let mut a: i32 = 0;

		add_assign(&mut a, 1);
		assert_eq!(a, 1);
	}

	#[test]
	fn add_assign_02() {
		let mut a: i32 = 1;

		add_assign(&mut a, 0);
		assert_eq!(a, 1);
	}

	#[test]
	fn add_assign_03() {
		let mut a: i32 = 1;

		add_assign(&mut a, 1);
		assert_eq!(a, 2);
	}

	#[test]
	fn add_assign_04() {
		let mut a: i32 = -1;

		add_assign(&mut a, 1);
		assert_eq!(a, 0);
	}

	#[test]
	fn add_assign_05() {
		let mut a: i32 = 1;

		add_assign(&mut a, -1);
		assert_eq!(a, 0);
	}

	#[test]
	fn add_assign_06() {
		let mut a: i32 = -1;

		add_assign(&mut a, -1);
		assert_eq!(a, -2);
	}

	#[test]
	fn add_assign_07() {
		let mut a: i32 = i32::MAX;

		add_assign(&mut a, 0);
		assert_eq!(a, i32::MAX);
	}

	#[test]
	fn add_assign_08() {
		let mut a: i32 = i32::MIN;

		add_assign(&mut a, 0);
		assert_eq!(a, i32::MIN);
	}

	#[test]
	fn add_assign_09() {
		let mut a: i32 = i32::MAX;

		add_assign(&mut a, i32::MIN);
		assert_eq!(a, -1);
	}

	#[test]
	fn add_assign_10() {
		let mut a: i32 = i32::MIN;

		add_assign(&mut a, i32::MAX);
		assert_eq!(a, -1);
	}
}
