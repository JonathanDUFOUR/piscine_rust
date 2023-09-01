use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq)]
pub struct Vector<T> {
	x: T,
	y: T,
}

impl<T> Vector<T> {
	/// Create a new Vector instance and initialize its attributes.
	///
	/// # Arguments
	///
	/// * `x` - The x direction of the vector to create.
	/// * `y` - The y direction of the vector to create.
	///
	/// # Example
	/// ```
	/// use ex05::Vector;
	///
	/// let vector = Vector::new(1, 2);
	/// ```
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T: Add<Output = T>> Add for Vector<T> {
	type Output = Self;

	#[inline(always)]
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
	type Output = Self;

	#[inline(always)]
	fn sub(self: Self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector<T> {
	type Output = Self;

	#[inline(always)]
	fn mul(self: Self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector<T> {
	type Output = Self;

	#[inline(always)]
	fn div(self: Self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl<T: AddAssign> AddAssign for Vector<T> {
	#[inline(always)]
	fn add_assign(self: &mut Self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T: SubAssign> SubAssign for Vector<T> {
	#[inline(always)]
	fn sub_assign(self: &mut Self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T: Copy + MulAssign> MulAssign<T> for Vector<T> {
	#[inline(always)]
	fn mul_assign(self: &mut Self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T: Copy + DivAssign> DivAssign<T> for Vector<T> {
	#[inline(always)]
	fn div_assign(self: &mut Self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_00() {
		const X: u8 = 0;
		const Y: u8 = 0;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_01() {
		const X: i8 = 0;
		const Y: i8 = -1;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_02() {
		const X: u16 = 42;
		const Y: u16 = 0;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_03() {
		const X: i16 = -1234;
		const Y: i16 = -4321;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_04() {
		const X: u32 = 134679;
		const Y: u32 = 946137;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_05() {
		const X: i32 = -123456789;
		const Y: i32 = -987654321;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_06() {
		const X: u64 = 555554444333221;
		const Y: u64 = 122333444455555;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_07() {
		const X: i64 = -7654321234567;
		const Y: i64 = -1234567654321;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_08() {
		const X: u128 = 122333444455555666666777777788888888;
		const Y: u128 = 888888887777777666666555554444333221;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_09() {
		const X: i128 = -1234567654321234567654321234567654321;
		const Y: i128 = -7654321234567654321234567654321234567;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_10() {
		const X: f32 = -4.2;
		const Y: f32 = 3.14;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_11() {
		const X: f64 = 0.00000000000000000000000000000000000000000000000000001;
		const Y: f64 = -1111111111111111111111111111111111111111111111111111.2;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_12() {
		const X: char = 'a';
		const Y: char = 'b';

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_13() {
		const X: bool = true;
		const Y: bool = false;

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn new_14() {
		const X: &str = "Hello";
		const Y: &str = "World";

		assert_eq!(Vector::new(X, Y), Vector { x: X, y: Y });
	}

	#[test]
	fn operator_equivalent_00() {
		const LHS: Vector<u8> = Vector::new(0, 0);
		const RHS: Vector<u8> = Vector::new(0, 0);

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_equivalent_01() {
		const LHS: Vector<i8> = Vector::new(0, 0);
		const RHS: Vector<i8> = Vector::new(0, 1);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_02() {
		const LHS: Vector<u16> = Vector::new(0, 0);
		const RHS: Vector<u16> = Vector::new(1, 0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_03() {
		const LHS: Vector<i16> = Vector::new(0, 0);
		const RHS: Vector<i16> = Vector::new(1, 1);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_04() {
		const LHS: Vector<u32> = Vector::new(0, 1);
		const RHS: Vector<u32> = Vector::new(0, 0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_05() {
		const LHS: Vector<i32> = Vector::new(0, 1);
		const RHS: Vector<i32> = Vector::new(0, 1);

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_equivalent_06() {
		const LHS: Vector<u64> = Vector::new(0, 1);
		const RHS: Vector<u64> = Vector::new(1, 0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_07() {
		const LHS: Vector<i64> = Vector::new(0, 1);
		const RHS: Vector<i64> = Vector::new(1, 1);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_08() {
		const LHS: Vector<u128> = Vector::new(1, 0);
		const RHS: Vector<u128> = Vector::new(0, 0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_09() {
		const LHS: Vector<i128> = Vector::new(1, 0);
		const RHS: Vector<i128> = Vector::new(0, 1);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_10() {
		const LHS: Vector<usize> = Vector::new(1, 0);
		const RHS: Vector<usize> = Vector::new(1, 0);

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_equivalent_11() {
		const LHS: Vector<isize> = Vector::new(1, 0);
		const RHS: Vector<isize> = Vector::new(1, 1);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_12() {
		const LHS: Vector<f32> = Vector::new(1.0, 1.0);
		const RHS: Vector<f32> = Vector::new(0.0, 0.0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_13() {
		const LHS: Vector<f64> = Vector::new(1.0, 1.0);
		const RHS: Vector<f64> = Vector::new(0.0, 1.0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_14() {
		const LHS: Vector<char> = Vector::new('1', '1');
		const RHS: Vector<char> = Vector::new('1', '0');

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_15() {
		const LHS: Vector<&str> = Vector::new("1", "1");
		const RHS: Vector<&str> = Vector::new("1", "1");

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_equivalent_16() {
		const LHS: Vector<bool> = Vector::new(false, false);
		const RHS: Vector<bool> = Vector::new(false, false);

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_different_00() {
		const LHS: Vector<u8> = Vector::new(0, 0);
		const RHS: Vector<u8> = Vector::new(0, 0);

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_different_01() {
		const LHS: Vector<i8> = Vector::new(0, 0);
		const RHS: Vector<i8> = Vector::new(0, 1);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_02() {
		const LHS: Vector<u16> = Vector::new(0, 0);
		const RHS: Vector<u16> = Vector::new(1, 0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_03() {
		const LHS: Vector<i16> = Vector::new(0, 0);
		const RHS: Vector<i16> = Vector::new(1, 1);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_04() {
		const LHS: Vector<u32> = Vector::new(0, 1);
		const RHS: Vector<u32> = Vector::new(0, 0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_05() {
		const LHS: Vector<i32> = Vector::new(0, 1);
		const RHS: Vector<i32> = Vector::new(0, 1);

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_different_06() {
		const LHS: Vector<u64> = Vector::new(0, 1);
		const RHS: Vector<u64> = Vector::new(1, 0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_07() {
		const LHS: Vector<i64> = Vector::new(0, 1);
		const RHS: Vector<i64> = Vector::new(1, 1);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_08() {
		const LHS: Vector<u128> = Vector::new(1, 0);
		const RHS: Vector<u128> = Vector::new(0, 0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_09() {
		const LHS: Vector<i128> = Vector::new(1, 0);
		const RHS: Vector<i128> = Vector::new(0, 1);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_10() {
		const LHS: Vector<usize> = Vector::new(1, 0);
		const RHS: Vector<usize> = Vector::new(1, 0);

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_different_11() {
		const LHS: Vector<isize> = Vector::new(1, 0);
		const RHS: Vector<isize> = Vector::new(1, 1);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_12() {
		const LHS: Vector<f32> = Vector::new(1.0, 1.0);
		const RHS: Vector<f32> = Vector::new(0.0, 0.0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_13() {
		const LHS: Vector<f64> = Vector::new(1.0, 1.0);
		const RHS: Vector<f64> = Vector::new(0.0, 1.0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_14() {
		const LHS: Vector<char> = Vector::new('1', '1');
		const RHS: Vector<char> = Vector::new('1', '0');

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_15() {
		const LHS: Vector<&str> = Vector::new("1", "1");
		const RHS: Vector<&str> = Vector::new("1", "1");

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_different_16() {
		const LHS: Vector<bool> = Vector::new(false, false);
		const RHS: Vector<bool> = Vector::new(false, false);

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_add_00() {
		const LHS: Vector<u8> = Vector::new(42, 37);
		const RHS: Vector<u8> = Vector::new(24, 13);
		const EXPECTED: Vector<u8> = Vector::new(66, 50);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_01() {
		const LHS: Vector<i8> = Vector::new(-42, 37);
		const RHS: Vector<i8> = Vector::new(24, -13);
		const EXPECTED: Vector<i8> = Vector::new(-18, 24);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_02() {
		const LHS: Vector<u16> = Vector::new(579, 666);
		const RHS: Vector<u16> = Vector::new(223, 333);
		const EXPECTED: Vector<u16> = Vector::new(802, 999);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_03() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<i16> = Vector::new(0, 0);
		const RHS: Vector<i16> = Vector::new(1, 1);
		const EXPECTED: Vector<i16> = Vector::new(1, 1);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_04() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<u32> = Vector::new(0, 1);
		const RHS: Vector<u32> = Vector::new(0, 0);
		const EXPECTED: Vector<u32> = Vector::new(0, 1);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_05() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<i32> = Vector::new(0, 1);
		const RHS: Vector<i32> = Vector::new(0, 1);
		const EXPECTED: Vector<i32> = Vector::new(0, 2);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_06() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<u64> = Vector::new(0, 1);
		const RHS: Vector<u64> = Vector::new(1, 0);
		const EXPECTED: Vector<u64> = Vector::new(1, 1);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_07() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<i64> = Vector::new(0, 1);
		const RHS: Vector<i64> = Vector::new(1, 1);
		const EXPECTED: Vector<i64> = Vector::new(1, 2);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_08() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<u128> = Vector::new(1, 0);
		const RHS: Vector<u128> = Vector::new(0, 0);
		const EXPECTED: Vector<u128> = Vector::new(1, 0);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_09() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<i128> = Vector::new(1, 0);
		const RHS: Vector<i128> = Vector::new(0, 1);
		const EXPECTED: Vector<i128> = Vector::new(1, 1);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_10() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<usize> = Vector::new(1, 0);
		const RHS: Vector<usize> = Vector::new(1, 0);
		const EXPECTED: Vector<usize> = Vector::new(2, 0);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_11() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<isize> = Vector::new(1, 0);
		const RHS: Vector<isize> = Vector::new(1, 1);
		const EXPECTED: Vector<isize> = Vector::new(2, 1);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_12() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<f32> = Vector::new(1.0, 1.0);
		const RHS: Vector<f32> = Vector::new(0.0, 0.0);
		const EXPECTED: Vector<f32> = Vector::new(1.0, 1.0);

		assert_eq!(LHS + RHS, EXPECTED);
	}

	#[test]
	fn operator_add_13() {
		// TODO: Change LHS, RHS, and EXPECTED values.
		const LHS: Vector<f64> = Vector::new(1.0, 1.0);
		const RHS: Vector<f64> = Vector::new(0.0, 1.0);
		const EXPECTED: Vector<f64> = Vector::new(1.0, 2.0);

		assert_eq!(LHS + RHS, EXPECTED);
	}
}
