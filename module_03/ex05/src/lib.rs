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
		const LHS: Vector<f32> = Vector::new(1.0, 0.0);
		const RHS: Vector<f32> = Vector::new(1.0, 0.0);

		assert_eq!(LHS == RHS, true);
	}

	#[test]
	fn operator_equivalent_11() {
		const LHS: Vector<f64> = Vector::new(1.0, 0.0);
		const RHS: Vector<f64> = Vector::new(1.0, 1.0);

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_12() {
		const LHS: Vector<char> = Vector::new('1', '1');
		const RHS: Vector<char> = Vector::new('0', '0');

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_13() {
		const LHS: Vector<&str> = Vector::new("1", "1");
		const RHS: Vector<&str> = Vector::new("0", "1");

		assert_eq!(LHS == RHS, false);
	}

	#[test]
	fn operator_equivalent_14() {
		const LHS: Vector<bool> = Vector::new(true, true);
		const RHS: Vector<bool> = Vector::new(true, true);

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
		const LHS: Vector<f32> = Vector::new(1.0, 0.0);
		const RHS: Vector<f32> = Vector::new(1.0, 0.0);

		assert_eq!(LHS != RHS, false);
	}

	#[test]
	fn operator_different_11() {
		const LHS: Vector<f64> = Vector::new(1.0, 0.0);
		const RHS: Vector<f64> = Vector::new(1.0, 1.0);

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_12() {
		const LHS: Vector<char> = Vector::new('1', '1');
		const RHS: Vector<char> = Vector::new('0', '0');

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_13() {
		const LHS: Vector<&str> = Vector::new("1", "1");
		const RHS: Vector<&str> = Vector::new("0", "1");

		assert_eq!(LHS != RHS, true);
	}

	#[test]
	fn operator_different_14() {
		const LHS: Vector<bool> = Vector::new(true, true);
		const RHS: Vector<bool> = Vector::new(true, true);

		assert_eq!(LHS != RHS, false);
	}
}
