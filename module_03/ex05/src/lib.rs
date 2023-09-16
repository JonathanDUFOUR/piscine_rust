use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
	/// # Returns
	///
	/// The newly created Vector instance.
	///
	/// # Example
	/// ```
	/// use ex05::Vector;
	///
	/// let vector: Vector<u8> = Vector::new(1, 2);
	/// ```
	#[inline(always)]
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl Vector<f32> {
	/// Calculate the length of the vector.
	///
	/// # Returns
	///
	///  The calculated length of the vector.
	///
	/// # Example
	/// ```
	/// use ex05::Vector;
	///
	/// let vector: Vector<f32> = Vector::new(3.0, 4.0);
	/// assert_eq!(vector.length(), 5.0);
	/// ```
	#[inline(always)]
	pub fn length(self: &Self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

impl Vector<f64> {
	/// Calculate the length of the vector.
	///
	/// # Returns
	///
	/// The calculated length of the vector.
	///
	/// # Example
	/// ```
	/// use ex05::Vector;
	///
	/// let vector: Vector<f64> = Vector::new(3.0, 4.0);
	/// assert_eq!(vector.length(), 5.0);
	/// ```
	#[inline(always)]
	pub fn length(self: &Self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}
}

impl<T> Add for Vector<T>
where
	T: Add<Output = T>,
{
	type Output = Self;

	#[inline(always)]
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl<T> Sub for Vector<T>
where
	T: Sub<Output = T>,
{
	type Output = Self;

	#[inline(always)]
	fn sub(self: Self, rhs: Self) -> Self::Output {
		Self::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl<T> Mul<T> for Vector<T>
where
	T: Mul<Output = T> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn mul(self: Self, rhs: T) -> Self::Output {
		Self::new(self.x * rhs, self.y * rhs)
	}
}

impl<T> Div<T> for Vector<T>
where
	T: Div<Output = T> + Copy,
{
	type Output = Self;

	#[inline(always)]
	fn div(self: Self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl<T> AddAssign for Vector<T>
where
	T: AddAssign,
{
	#[inline(always)]
	fn add_assign(self: &mut Self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> SubAssign for Vector<T>
where
	T: SubAssign,
{
	#[inline(always)]
	fn sub_assign(self: &mut Self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> MulAssign<T> for Vector<T>
where
	T: MulAssign + Copy,
{
	#[inline(always)]
	fn mul_assign(self: &mut Self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> DivAssign<T> for Vector<T>
where
	T: DivAssign + Copy,
{
	#[inline(always)]
	fn div_assign(self: &mut Self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	struct A {}

	impl A {
		#[inline(always)]
		const fn new() -> Self {
			Self {}
		}
	}

	impl Add for A {
		type Output = Self;

		#[inline(always)]
		fn add(self: Self, _rhs: Self) -> Self::Output {
			Self {}
		}
	}

	impl Sub for A {
		type Output = Self;

		#[inline(always)]
		fn sub(self: Self, _rhs: Self) -> Self::Output {
			Self {}
		}
	}

	impl Mul for A {
		type Output = Self;

		#[inline(always)]
		fn mul(self: Self, _rhs: Self) -> Self::Output {
			Self {}
		}
	}

	impl Div for A {
		type Output = Self;

		#[inline(always)]
		fn div(self: Self, _rhs: Self) -> Self::Output {
			Self {}
		}
	}

	impl AddAssign for A {
		#[inline(always)]
		fn add_assign(self: &mut Self, _rhs: Self) {}
	}

	impl SubAssign for A {
		#[inline(always)]
		fn sub_assign(self: &mut Self, _rhs: Self) {}
	}

	impl MulAssign for A {
		#[inline(always)]
		fn mul_assign(self: &mut Self, _rhs: Self) {}
	}

	impl DivAssign for A {
		#[inline(always)]
		fn div_assign(self: &mut Self, _rhs: Self) {}
	}

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	struct B {
		n: u8,
	}

	impl B {
		#[inline(always)]
		const fn new(n: u8) -> Self {
			Self { n }
		}
	}

	impl Add for B {
		type Output = Self;

		#[inline(always)]
		fn add(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n + rhs.n }
		}
	}

	impl Sub for B {
		type Output = Self;

		#[inline(always)]
		fn sub(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n - rhs.n }
		}
	}

	impl Mul for B {
		type Output = Self;

		#[inline(always)]
		fn mul(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n * rhs.n }
		}
	}

	impl Div for B {
		type Output = Self;

		#[inline(always)]
		fn div(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n / rhs.n }
		}
	}

	impl AddAssign for B {
		#[inline(always)]
		fn add_assign(self: &mut Self, rhs: Self) {
			self.n += rhs.n;
		}
	}

	impl SubAssign for B {
		#[inline(always)]
		fn sub_assign(self: &mut Self, rhs: Self) {
			self.n -= rhs.n;
		}
	}

	impl MulAssign for B {
		#[inline(always)]
		fn mul_assign(self: &mut Self, rhs: Self) {
			self.n *= rhs.n;
		}
	}

	impl DivAssign for B {
		#[inline(always)]
		fn div_assign(self: &mut Self, rhs: Self) {
			self.n /= rhs.n;
		}
	}

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	struct C {
		n: i8,
	}

	impl C {
		#[inline(always)]
		const fn new(n: i8) -> Self {
			Self { n }
		}
	}

	impl Add for C {
		type Output = Self;

		#[inline(always)]
		fn add(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n + rhs.n }
		}
	}

	impl Sub for C {
		type Output = Self;

		#[inline(always)]
		fn sub(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n - rhs.n }
		}
	}

	impl Mul for C {
		type Output = Self;

		#[inline(always)]
		fn mul(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n * rhs.n }
		}
	}

	impl Div for C {
		type Output = Self;

		#[inline(always)]
		fn div(self: Self, rhs: Self) -> Self::Output {
			Self { n: self.n / rhs.n }
		}
	}

	impl AddAssign for C {
		#[inline(always)]
		fn add_assign(self: &mut Self, rhs: Self) {
			self.n += rhs.n;
		}
	}

	impl SubAssign for C {
		#[inline(always)]
		fn sub_assign(self: &mut Self, rhs: Self) {
			self.n -= rhs.n;
		}
	}

	impl MulAssign for C {
		#[inline(always)]
		fn mul_assign(self: &mut Self, rhs: Self) {
			self.n *= rhs.n;
		}
	}

	impl DivAssign for C {
		#[inline(always)]
		fn div_assign(self: &mut Self, rhs: Self) {
			self.n /= rhs.n;
		}
	}

	#[inline(always)]
	fn test_function_new<T>(x: T, y: T)
	where
		T: Copy + std::fmt::Debug + PartialEq,
	{
		assert_eq!(Vector::new(x, y), Vector { x, y });
	}

	#[inline(always)]
	fn test_operator_equivalent<T>(v0_x: T, v0_y: T, v1_x: T, v1_y: T)
	where
		T: Copy + PartialEq,
	{
		let v0: Vector<T> = Vector::new(v0_x, v0_y);
		let v1: Vector<T> = Vector::new(v1_x, v1_y);
		let expected: bool = v0_x == v1_x && v0_y == v1_y;

		assert_eq!(v0 == v0, true);
		assert_eq!(v0 == v1, expected);
		assert_eq!(v1 == v0, expected);
	}

	#[inline(always)]
	fn test_operator_different<T>(v0_x: T, v0_y: T, v1_x: T, v1_y: T)
	where
		T: Copy + PartialEq,
	{
		let v0: Vector<T> = Vector::new(v0_x, v0_y);
		let v1: Vector<T> = Vector::new(v1_x, v1_y);
		let expected: bool = v0_x != v1_x || v0_y != v1_y;

		assert_eq!(v0 != v0, false);
		assert_eq!(v0 != v1, expected);
		assert_eq!(v1 != v0, expected);
	}

	#[inline(always)]
	fn test_operator_add<T>(v0_x: T, v0_y: T, v1_x: T, v1_y: T)
	where
		T: Add<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let v0: Vector<T> = Vector::new(v0_x, v0_y);
		let v1: Vector<T> = Vector::new(v1_x, v1_y);
		let expected: Vector<T> = Vector::new(v0_x + v1_x, v0_y + v1_y);

		assert_eq!(v0 + v1, expected);
		assert_eq!(v1 + v0, expected);
	}

	#[inline(always)]
	fn test_operator_sub<T>(lhs_x: T, lhs_y: T, rhs_x: T, rhs_y: T)
	where
		T: Sub<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let lhs: Vector<T> = Vector::new(lhs_x, lhs_y);
		let rhs: Vector<T> = Vector::new(rhs_x, rhs_y);
		let expected: Vector<T> = Vector::new(lhs_x - rhs_x, lhs_y - rhs_y);

		assert_eq!(lhs - rhs, expected);
	}

	#[inline(always)]
	fn test_operator_mul<T>(lhs_x: T, lhs_y: T, rhs: T)
	where
		T: Mul<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let lhs: Vector<T> = Vector::new(lhs_x, lhs_y);
		let expected: Vector<T> = Vector::new(lhs_x * rhs, lhs_y * rhs);

		assert_eq!(lhs * rhs, expected);
	}

	#[inline(always)]
	fn test_operator_div<T>(lhs_x: T, lhs_y: T, rhs: T)
	where
		T: Div<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let lhs: Vector<T> = Vector::new(lhs_x, lhs_y);
		let expected: Vector<T> = Vector::new(lhs_x / rhs, lhs_y / rhs);

		assert_eq!(lhs / rhs, expected);
	}

	#[inline(always)]
	fn test_operator_add_assign<T>(v0_x: T, v0_y: T, v1_x: T, v1_y: T)
	where
		T: AddAssign + Add<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let v0: Vector<T> = Vector::new(v0_x, v0_y);
		let v1: Vector<T> = Vector::new(v1_x, v1_y);
		let expected: Vector<T> = Vector::new(v0_x + v1_x, v0_y + v1_y);
		let mut v2: Vector<T>;

		v2 = v0;
		v2 += v1;
		assert_eq!(v2, expected);
		v2 = v1;
		v2 += v0;
		assert_eq!(v2, expected);
	}

	#[inline(always)]
	fn test_operator_sub_assign<T>(lhs_x: T, lhs_y: T, rhs_x: T, rhs_y: T)
	where
		T: SubAssign + Sub<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let rhs: Vector<T> = Vector::new(rhs_x, rhs_y);
		let expected: Vector<T> = Vector::new(lhs_x - rhs_x, lhs_y - rhs_y);
		let mut lhs: Vector<T> = Vector::new(lhs_x, lhs_y);

		lhs -= rhs;
		assert_eq!(lhs, expected);
	}

	#[inline(always)]
	fn test_operator_mul_assign<T>(lhs_x: T, lhs_y: T, rhs: T)
	where
		T: MulAssign + Mul<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let expected: Vector<T> = Vector::new(lhs_x * rhs, lhs_y * rhs);
		let mut lhs: Vector<T> = Vector::new(lhs_x, lhs_y);

		lhs *= rhs;
		assert_eq!(lhs, expected);
	}

	#[inline(always)]
	fn test_operator_div_assign<T>(lhs_x: T, lhs_y: T, rhs: T)
	where
		T: DivAssign + Div<Output = T> + Copy + std::fmt::Debug + PartialEq,
	{
		let expected: Vector<T> = Vector::new(lhs_x / rhs, lhs_y / rhs);
		let mut lhs: Vector<T> = Vector::new(lhs_x, lhs_y);

		lhs /= rhs;
		assert_eq!(lhs, expected);
	}

	#[inline(always)]
	fn test_function_length_f32(x: f32, y: f32) {
		let v: Vector<f32> = Vector::new(x, y);
		let expected: f32 = (x * x + y * y).sqrt();

		if expected.is_nan() {
			assert!(v.length().is_nan());
		} else {
			assert_eq!(v.length(), expected);
		}
	}

	#[inline(always)]
	fn test_function_length_f64(x: f64, y: f64) {
		let v: Vector<f64> = Vector::new(x, y);
		let expected: f64 = (x * x + y * y).sqrt();

		if expected.is_nan() {
			assert!(v.length().is_nan());
		} else {
			assert_eq!(v.length(), expected);
		}
	}

	#[test]
	fn new_00() {
		test_function_new(A::new(), A::new());
	}

	#[test]
	fn new_01() {
		test_function_new(B::new(21), B::new(42));
	}

	#[test]
	fn new_02() {
		test_function_new(C::new(-56), C::new(124));
	}

	#[test]
	fn new_03() {
		test_function_new('a', 'b');
	}

	#[test]
	fn new_04() {
		test_function_new(false, true);
	}

	#[test]
	fn new_05() {
		test_function_new("Hello", "World");
	}

	#[test]
	fn operator_equivalent_00() {
		test_operator_equivalent(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_equivalent_01() {
		test_operator_equivalent(B::new(0x00), B::new(0xfe), B::new(0x00), B::new(0xff));
	}

	#[test]
	fn operator_equivalent_02() {
		test_operator_equivalent(C::new(-42), C::new(125), C::new(-42), C::new(125));
	}

	#[test]
	fn operator_equivalent_03() {
		test_operator_equivalent('1', '1', '1', '0');
	}

	#[test]
	fn operator_equivalent_04() {
		test_operator_equivalent(false, false, false, false);
	}

	#[test]
	fn operator_equivalent_05() {
		test_operator_equivalent("0", "1", "1", "0");
	}

	#[test]
	fn operator_different_00() {
		test_operator_different(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_different_01() {
		test_operator_different(B::new(0x00), B::new(0xfe), B::new(0x00), B::new(0xff));
	}

	#[test]
	fn operator_different_02() {
		test_operator_different(C::new(-42), C::new(125), C::new(-42), C::new(125));
	}

	#[test]
	fn operator_different_03() {
		test_operator_different('1', '1', '1', '0');
	}

	#[test]
	fn operator_different_04() {
		test_operator_different(false, false, false, false);
	}

	#[test]
	fn operator_different_05() {
		test_operator_different("0", "1", "1", "0");
	}

	#[test]
	fn operator_add_00() {
		test_operator_add(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_add_01() {
		test_operator_add(B::new(0x12), B::new(0x34), B::new(0x56), B::new(0x78));
	}

	#[test]
	fn operator_add_02() {
		test_operator_add(C::new(-14), C::new(70), C::new(15), C::new(-52));
	}

	#[test]
	fn operator_sub_00() {
		test_operator_sub(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_sub_01() {
		test_operator_sub(B::new(0x72), B::new(0x81), B::new(0x09), B::new(0x36));
	}

	#[test]
	fn operator_sub_02() {
		test_operator_sub(C::new(10), C::new(-23), C::new(-99), C::new(48));
	}

	#[test]
	fn operator_mul_00() {
		test_operator_mul(A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_mul_01() {
		test_operator_mul(B::new(0x21), B::new(0x1d), B::new(0x03));
	}

	#[test]
	fn operator_mul_02() {
		test_operator_mul(C::new(-9), C::new(32), C::new(-4));
	}

	#[test]
	fn operator_div_00() {
		test_operator_div(A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_div_01() {
		test_operator_div(B::new(0xcb), B::new(0x3f), B::new(0x20));
	}

	#[test]
	fn operator_div_02() {
		test_operator_div(C::new(-111), C::new(-55), C::new(111));
	}

	#[test]
	fn operator_add_assign_00() {
		test_operator_add_assign(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_add_assign_01() {
		test_operator_add_assign(B::new(0x88), B::new(0xc4), B::new(0x0e), B::new(0x1f));
	}

	#[test]
	fn operator_add_assign_02() {
		test_operator_add_assign(C::new(-22), C::new(40), C::new(71), C::new(-86));
	}

	#[test]
	fn operator_sub_assign_00() {
		test_operator_sub_assign(A::new(), A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_sub_assign_01() {
		test_operator_sub_assign(B::new(0xd2), B::new(0x42), B::new(0xa1), B::new(0x35));
	}

	#[test]
	fn operator_sub_assign_02() {
		test_operator_sub_assign(C::new(-1), C::new(13), C::new(-25), C::new(9));
	}

	#[test]
	fn operator_mul_assign_00() {
		test_operator_mul_assign(A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_mul_assign_01() {
		test_operator_mul_assign(B::new(0x08), B::new(0x06), B::new(0x0c));
	}

	#[test]
	fn operator_mul_assign_02() {
		test_operator_mul_assign(C::new(-2), C::new(3), C::new(42));
	}

	#[test]
	fn operator_div_assign_00() {
		test_operator_div_assign(A::new(), A::new(), A::new());
	}

	#[test]
	fn operator_div_assign_01() {
		test_operator_div_assign(B::new(0x92), B::new(0x3e), B::new(0x0a));
	}

	#[test]
	fn operator_div_assign_02() {
		test_operator_div_assign(C::new(-35), C::new(64), C::new(-28));
	}

	#[test]
	fn function_length_00() {
		test_function_length_f32(0.0, 0.0);
	}

	#[test]
	fn function_length_01() {
		test_function_length_f32(-3.0, 4.0);
	}

	#[test]
	fn function_length_02() {
		test_function_length_f32(12.0, -7.0);
	}

	#[test]
	fn function_length_03() {
		test_function_length_f32(f32::INFINITY, f32::NEG_INFINITY);
	}

	#[test]
	fn function_length_04() {
		test_function_length_f32(f32::NAN, f32::NAN);
	}

	#[test]
	fn function_length_05() {
		test_function_length_f64(0.0, 0.0);
	}

	#[test]
	fn function_length_06() {
		test_function_length_f64(-3.0, 4.0);
	}

	#[test]
	fn function_length_07() {
		test_function_length_f64(12.0, -7.0);
	}

	#[test]
	fn function_length_08() {
		test_function_length_f64(f64::INFINITY, f64::NEG_INFINITY);
	}

	#[test]
	fn function_length_09() {
		test_function_length_f64(f64::NAN, f64::NAN);
	}

	#[test]
	fn subject_00() {
		let v = Vector {
			x: String::from("Hello, World!"),
			y: String::from("Hello, Rust!"),
		};
		let w = v.clone();

		assert_eq!(&v, &w);
	}

	#[test]
	fn subject_01() {
		let v = Vector::new("Hello, World!", "Hello, Rust!");
		let a = v;
		let b = v;
		assert_eq!(a, b);
	}
}
