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
		const A: Vector<u8> = Vector::new(0, 0);
		const B: Vector<u8> = Vector::new(0, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, true);
		assert_eq!(B == A, true);
	}

	#[test]
	fn operator_equivalent_01() {
		const A: Vector<i8> = Vector::new(0, 0);
		const B: Vector<i8> = Vector::new(0, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_02() {
		const A: Vector<u16> = Vector::new(0, 0);
		const B: Vector<u16> = Vector::new(1, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_03() {
		const A: Vector<i16> = Vector::new(0, 0);
		const B: Vector<i16> = Vector::new(1, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_04() {
		const A: Vector<u32> = Vector::new(0, 1);
		const B: Vector<u32> = Vector::new(0, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_05() {
		const A: Vector<i32> = Vector::new(0, 1);
		const B: Vector<i32> = Vector::new(0, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, true);
		assert_eq!(B == A, true);
	}

	#[test]
	fn operator_equivalent_06() {
		const A: Vector<u64> = Vector::new(0, 1);
		const B: Vector<u64> = Vector::new(1, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_07() {
		const A: Vector<i64> = Vector::new(0, 1);
		const B: Vector<i64> = Vector::new(1, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_08() {
		const A: Vector<u128> = Vector::new(1, 0);
		const B: Vector<u128> = Vector::new(0, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_09() {
		const A: Vector<i128> = Vector::new(1, 0);
		const B: Vector<i128> = Vector::new(0, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_10() {
		const A: Vector<usize> = Vector::new(1, 0);
		const B: Vector<usize> = Vector::new(1, 0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, true);
		assert_eq!(B == A, true);
	}

	#[test]
	fn operator_equivalent_11() {
		const A: Vector<isize> = Vector::new(1, 0);
		const B: Vector<isize> = Vector::new(1, 1);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_12() {
		const A: Vector<f32> = Vector::new(1.0, 1.0);
		const B: Vector<f32> = Vector::new(0.0, 0.0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_13() {
		const A: Vector<f64> = Vector::new(1.0, 1.0);
		const B: Vector<f64> = Vector::new(0.0, 1.0);

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_14() {
		const A: Vector<char> = Vector::new('1', '1');
		const B: Vector<char> = Vector::new('1', '0');

		assert_eq!(A == A, true);
		assert_eq!(A == B, false);
		assert_eq!(B == A, false);
	}

	#[test]
	fn operator_equivalent_15() {
		const A: Vector<&str> = Vector::new("1", "1");
		const B: Vector<&str> = Vector::new("1", "1");

		assert_eq!(A == A, true);
		assert_eq!(A == B, true);
		assert_eq!(B == A, true);
	}

	#[test]
	fn operator_equivalent_16() {
		const A: Vector<bool> = Vector::new(false, false);
		const B: Vector<bool> = Vector::new(false, false);

		assert_eq!(A == A, true);
		assert_eq!(A == B, true);
		assert_eq!(B == A, true);
	}

	#[test]
	fn operator_different_00() {
		const A: Vector<u8> = Vector::new(0, 0);
		const B: Vector<u8> = Vector::new(0, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, false);
		assert_eq!(B != A, false);
	}

	#[test]
	fn operator_different_01() {
		const A: Vector<i8> = Vector::new(0, 0);
		const B: Vector<i8> = Vector::new(0, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_02() {
		const A: Vector<u16> = Vector::new(0, 0);
		const B: Vector<u16> = Vector::new(1, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_03() {
		const A: Vector<i16> = Vector::new(0, 0);
		const B: Vector<i16> = Vector::new(1, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_04() {
		const A: Vector<u32> = Vector::new(0, 1);
		const B: Vector<u32> = Vector::new(0, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_05() {
		const A: Vector<i32> = Vector::new(0, 1);
		const B: Vector<i32> = Vector::new(0, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, false);
		assert_eq!(B != A, false);
	}

	#[test]
	fn operator_different_06() {
		const A: Vector<u64> = Vector::new(0, 1);
		const B: Vector<u64> = Vector::new(1, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_07() {
		const A: Vector<i64> = Vector::new(0, 1);
		const B: Vector<i64> = Vector::new(1, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_08() {
		const A: Vector<u128> = Vector::new(1, 0);
		const B: Vector<u128> = Vector::new(0, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_09() {
		const A: Vector<i128> = Vector::new(1, 0);
		const B: Vector<i128> = Vector::new(0, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_10() {
		const A: Vector<usize> = Vector::new(1, 0);
		const B: Vector<usize> = Vector::new(1, 0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, false);
		assert_eq!(B != A, false);
	}

	#[test]
	fn operator_different_11() {
		const A: Vector<isize> = Vector::new(1, 0);
		const B: Vector<isize> = Vector::new(1, 1);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_12() {
		const A: Vector<f32> = Vector::new(1.0, 1.0);
		const B: Vector<f32> = Vector::new(0.0, 0.0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_13() {
		const A: Vector<f64> = Vector::new(1.0, 1.0);
		const B: Vector<f64> = Vector::new(0.0, 1.0);

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_14() {
		const A: Vector<char> = Vector::new('1', '1');
		const B: Vector<char> = Vector::new('1', '0');

		assert_eq!(A != A, false);
		assert_eq!(A != B, true);
		assert_eq!(B != A, true);
	}

	#[test]
	fn operator_different_15() {
		const A: Vector<&str> = Vector::new("1", "1");
		const B: Vector<&str> = Vector::new("1", "1");

		assert_eq!(A != A, false);
		assert_eq!(A != B, false);
		assert_eq!(B != A, false);
	}

	#[test]
	fn operator_different_16() {
		const A: Vector<bool> = Vector::new(false, false);
		const B: Vector<bool> = Vector::new(false, false);

		assert_eq!(A != A, false);
		assert_eq!(A != B, false);
		assert_eq!(B != A, false);
	}

	#[test]
	fn operator_add_00() {
		const A_X: u8 = 42;
		const A_Y: u8 = 37;
		const RHS_X: u8 = 24;
		const RHS_Y: u8 = 13;
		const A: Vector<u8> = Vector::new(A_X, A_Y);
		const B: Vector<u8> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u8> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_01() {
		const A_X: i8 = -42;
		const A_Y: i8 = 37;
		const RHS_X: i8 = 24;
		const RHS_Y: i8 = -13;
		const A: Vector<i8> = Vector::new(A_X, A_Y);
		const B: Vector<i8> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i8> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_02() {
		const A_X: u16 = 579;
		const A_Y: u16 = 666;
		const RHS_X: u16 = 223;
		const RHS_Y: u16 = 333;
		const A: Vector<u16> = Vector::new(A_X, A_Y);
		const B: Vector<u16> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u16> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_03() {
		const A_X: i16 = -579;
		const A_Y: i16 = 666;
		const RHS_X: i16 = 233;
		const RHS_Y: i16 = -333;
		const A: Vector<i16> = Vector::new(A_X, A_Y);
		const B: Vector<i16> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i16> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_04() {
		const A_X: u32 = 99989796;
		const A_Y: u32 = 868462;
		const RHS_X: u32 = 11121314;
		const RHS_Y: u32 = 156735;
		const A: Vector<u32> = Vector::new(A_X, A_Y);
		const B: Vector<u32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u32> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_05() {
		const A_X: i32 = -99989796;
		const A_Y: i32 = 868462;
		const RHS_X: i32 = 11121314;
		const RHS_Y: i32 = -156735;
		const A: Vector<i32> = Vector::new(A_X, A_Y);
		const B: Vector<i32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i32> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_06() {
		const A_X: u64 = 34385221568451004;
		const A_Y: u64 = 775218960468323;
		const RHS_X: u64 = 10842267456381903;
		const RHS_Y: u64 = 513897651258763;
		const A: Vector<u64> = Vector::new(A_X, A_Y);
		const B: Vector<u64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u64> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_07() {
		const A_X: i64 = -34385221568451004;
		const A_Y: i64 = 775218960468323;
		const RHS_X: i64 = 10842267456381903;
		const RHS_Y: i64 = -513897651258763;
		const A: Vector<i64> = Vector::new(A_X, A_Y);
		const B: Vector<i64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i64> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_08() {
		const A_X: u128 = 753154682723179648101430;
		const A_Y: u128 = 253448910430876192408113;
		const RHS_X: u128 = 594837264587963254125793;
		const RHS_Y: u128 = 123456789012345678901234;
		const A: Vector<u128> = Vector::new(A_X, A_Y);
		const B: Vector<u128> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u128> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_09() {
		const A_X: i128 = -71808433846287399410816;
		const A_Y: i128 = 2710896536725620914729;
		const RHS_X: i128 = 673520165547328004521462;
		const RHS_Y: i128 = -1234567890123456789012;
		const A: Vector<i128> = Vector::new(A_X, A_Y);
		const B: Vector<i128> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i128> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_10() {
		const A_X: usize = 910229776674578740;
		const A_Y: usize = 775346328037744557;
		const RHS_X: usize = 890596211341745422;
		const RHS_Y: usize = 356629621899658002;
		const A: Vector<usize> = Vector::new(A_X, A_Y);
		const B: Vector<usize> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<usize> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_11() {
		const A_X: isize = -555144761072283375;
		const A_Y: isize = -618241292701782516;
		const RHS_X: isize = -630822953165765373;
		const RHS_Y: isize = -354852285947913161;
		const A: Vector<isize> = Vector::new(A_X, A_Y);
		const B: Vector<isize> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<isize> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_12() {
		const A_X: f32 = 2.3236801624298095703125;
		const A_Y: f32 = 6.095500469207763671875;
		const RHS_X: f32 = -7.7994251251220703125;
		const RHS_Y: f32 = -9.25922870635986328125;
		const A: Vector<f32> = Vector::new(A_X, A_Y);
		const B: Vector<f32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<f32> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_add_13() {
		const A_X: f64 = 9094924168448048317657520807834234640892181891518240986759168.0;
		const A_Y: f64 = 8443599126371414379654864980266240158403912962654371116482560.0;
		const RHS_X: f64 = -8606246334228050844468748003196119889206728317870082462056448.0;
		const RHS_Y: f64 = -6892461893904804034376935372196642757706266769942608267276516.0;
		const A: Vector<f64> = Vector::new(A_X, A_Y);
		const B: Vector<f64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<f64> = Vector::new(A_X + RHS_X, A_Y + RHS_Y);

		assert_eq!(A + B, EXPECTED);
		assert_eq!(B + A, EXPECTED);
	}

	#[test]
	fn operator_sub_00() {
		const A_X: u8 = 42;
		const A_Y: u8 = 37;
		const RHS_X: u8 = 24;
		const RHS_Y: u8 = 13;
		const A: Vector<u8> = Vector::new(A_X, A_Y);
		const B: Vector<u8> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u8> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_01() {
		const A_X: i8 = -42;
		const A_Y: i8 = 37;
		const RHS_X: i8 = 24;
		const RHS_Y: i8 = -13;
		const A: Vector<i8> = Vector::new(A_X, A_Y);
		const B: Vector<i8> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i8> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_02() {
		const A_X: u16 = 579;
		const A_Y: u16 = 666;
		const RHS_X: u16 = 223;
		const RHS_Y: u16 = 333;
		const A: Vector<u16> = Vector::new(A_X, A_Y);
		const B: Vector<u16> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u16> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_03() {
		const A_X: i16 = -579;
		const A_Y: i16 = 666;
		const RHS_X: i16 = 233;
		const RHS_Y: i16 = -333;
		const A: Vector<i16> = Vector::new(A_X, A_Y);
		const B: Vector<i16> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i16> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_04() {
		const A_X: u32 = 99989796;
		const A_Y: u32 = 868462;
		const RHS_X: u32 = 11121314;
		const RHS_Y: u32 = 156735;
		const A: Vector<u32> = Vector::new(A_X, A_Y);
		const B: Vector<u32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u32> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_05() {
		const A_X: i32 = -99989796;
		const A_Y: i32 = 868462;
		const RHS_X: i32 = 11121314;
		const RHS_Y: i32 = -156735;
		const A: Vector<i32> = Vector::new(A_X, A_Y);
		const B: Vector<i32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i32> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_06() {
		const A_X: u64 = 34385221568451004;
		const A_Y: u64 = 775218960468323;
		const RHS_X: u64 = 10842267456381903;
		const RHS_Y: u64 = 513897651258763;
		const A: Vector<u64> = Vector::new(A_X, A_Y);
		const B: Vector<u64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u64> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_07() {
		const A_X: i64 = -34385221568451004;
		const A_Y: i64 = 775218960468323;
		const RHS_X: i64 = 10842267456381903;
		const RHS_Y: i64 = -513897651258763;
		const A: Vector<i64> = Vector::new(A_X, A_Y);
		const B: Vector<i64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i64> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_08() {
		const A_X: u128 = 753154682723179648101430;
		const A_Y: u128 = 253448910430876192408113;
		const RHS_X: u128 = 594837264587963254125793;
		const RHS_Y: u128 = 123456789012345678901234;
		const A: Vector<u128> = Vector::new(A_X, A_Y);
		const B: Vector<u128> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<u128> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_09() {
		const A_X: i128 = -71808433846287399410816;
		const A_Y: i128 = 2710896536725620914729;
		const RHS_X: i128 = 673520165547328004521462;
		const RHS_Y: i128 = -1234567890123456789012;
		const A: Vector<i128> = Vector::new(A_X, A_Y);
		const B: Vector<i128> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<i128> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_10() {
		const A_X: usize = 910229776674578740;
		const A_Y: usize = 775346328037744557;
		const RHS_X: usize = 890596211341745422;
		const RHS_Y: usize = 356629621899658002;
		const A: Vector<usize> = Vector::new(A_X, A_Y);
		const B: Vector<usize> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<usize> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_11() {
		const A_X: isize = -555144761072283375;
		const A_Y: isize = -618241292701782516;
		const RHS_X: isize = -630822953165765373;
		const RHS_Y: isize = -354852285947913161;
		const A: Vector<isize> = Vector::new(A_X, A_Y);
		const B: Vector<isize> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<isize> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_12() {
		const A_X: f32 = 2.3236801624298095703125;
		const A_Y: f32 = 6.095500469207763671875;
		const RHS_X: f32 = -7.7994251251220703125;
		const RHS_Y: f32 = -9.25922870635986328125;
		const A: Vector<f32> = Vector::new(A_X, A_Y);
		const B: Vector<f32> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<f32> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_sub_13() {
		const A_X: f64 = 9094924168448048317657520807834234640892181891518240986759168.0;
		const A_Y: f64 = 8443599126371414379654864980266240158403912962654371116482560.0;
		const RHS_X: f64 = -8606246334228050844468748003196119889206728317870082462056448.0;
		const RHS_Y: f64 = -6892461893904804034376935372196642757706266769942608267276516.0;
		const A: Vector<f64> = Vector::new(A_X, A_Y);
		const B: Vector<f64> = Vector::new(RHS_X, RHS_Y);
		const EXPECTED: Vector<f64> = Vector::new(A_X - RHS_X, A_Y - RHS_Y);

		assert_eq!(A - B, EXPECTED);
	}

	#[test]
	fn operator_mul_00() {
		const A_X: u8 = 13;
		const A_Y: u8 = 17;
		const A: Vector<u8> = Vector::new(A_X, A_Y);
		const B: u8 = 5;
		const EXPECTED: Vector<u8> = Vector::new(A_X * B, A_Y * B);

		assert_eq!(A * B, EXPECTED);
		// assert_eq!(B * A, EXPECTED);
	}

	#[test]
	fn operator_mul_01() {
		const A_X: i8 = -7;
		const A_Y: i8 = 11;
		const A: Vector<i8> = Vector::new(A_X, A_Y);
		const B: i8 = -2;
		const EXPECTED: Vector<i8> = Vector::new(A_X * B, A_Y * B);

		assert_eq!(A * B, EXPECTED);
		// assert_eq!(B * A, EXPECTED);
	}
}
