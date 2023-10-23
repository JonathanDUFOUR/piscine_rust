pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	/// Creates a new Point instance with given coordinates.
	///
	/// # Parameters
	/// * `x` - The x coordinate of the point to create.
	/// * `y` - The y coordinate of the point to create.
	///
	/// # Return
	/// The newly created Point instance.
	///
	/// # Examples
	/// ```
	/// use ex01::Point;
	///
	/// let p: Point = Point::new(1.0, 2.0);
	///
	/// assert_eq!(p.x, 1.0);
	/// assert_eq!(p.y, 2.0);
	/// ```
	pub fn new(x: f32, y: f32) -> Self {
		return Self { x, y };
	}

	/// Creates a new Point instance with coordinates (0, 0).
	///
	/// # Return
	/// The newly created Point instance.
	///
	/// # Examples
	/// ```
	/// use ex01::Point;
	///
	/// let p: Point = Point::zero();
	///
	/// assert_eq!(p.x, 0.0);
	/// assert_eq!(p.y, 0.0);
	/// ```
	pub fn zero() -> Self {
		return Self { x: 0.0, y: 0.0 };
	}

	/// Calculates the distance with another given point.
	///
	/// # Parameters
	/// * `other` - The other point to calculate the distance with.
	///
	/// # Return
	/// The distance between the two points.
	///
	/// # Examples
	/// ```
	/// use ex01::Point;
	///
	/// let p0: Point = Point::new(1.0, 2.0);
	/// let p1: Point = Point::new(2.0, 1.0);
	///
	/// assert_eq!(p0.distance(&p1), 2.0_f32.sqrt());
	/// ```
	pub fn distance(self: &Self, other: &Self) -> f32 {
		if self.x.is_nan() || self.y.is_nan() || other.x.is_nan() || other.y.is_nan() {
			return f32::NAN;
		}
		if self.x == other.x {
			if self.y == other.y {
				return 0.0;
			}
			if self.y > other.y {
				return self.y - other.y;
			}
			return other.y - self.y;
		}
		if self.y == other.y {
			if self.x > other.x {
				return self.x - other.x;
			}
			return other.x - self.x;
		}

		let dx: f64 = (self.x - other.x) as f64;
		let dy: f64 = (self.y - other.y) as f64;

		return (dx.powi(2) + dy.powi(2)).sqrt() as f32;
	}

	/// Translates the point by given coordinates.
	///
	/// # Parameters
	/// * `dx` - The x coordinate to translate the point by.
	/// * `dy` - The y coordinate to translate the point by.
	///
	/// # Examples
	/// ```
	/// use ex01::Point;
	///
	/// let mut p: Point = Point::new(1.0, 2.0);
	///
	/// p.translate(-2.0, 1.0);
	/// assert_eq!(p.x, -1.0);
	/// assert_eq!(p.y, 3.0);
	/// ```
	pub fn translate(self: &mut Self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn point_new_00() {
		let p: Point = Point::new(0.0, 0.0);

		assert_eq!(p.x, 0.0);
		assert_eq!(p.y, 0.0);
	}

	#[test]
	fn point_new_01() {
		let p: Point = Point::new(0.0, 1.0);

		assert_eq!(p.x, 0.0);
		assert_eq!(p.y, 1.0);
	}

	#[test]
	fn point_new_02() {
		let p: Point = Point::new(1.0, 0.0);

		assert_eq!(p.x, 1.0);
		assert_eq!(p.y, 0.0);
	}

	#[test]
	fn point_new_03() {
		let p: Point = Point::new(-1.2, -2.3);

		assert_eq!(p.x, -1.2);
		assert_eq!(p.y, -2.3);
	}

	#[test]
	fn point_new_04() {
		let p: Point = Point::new(-2147483648.0, 2147483647.0);

		assert_eq!(p.x, -2147483648.0);
		assert_eq!(p.y, 2147483647.0);
	}

	#[test]
	fn point_new_05() {
		let p: Point = Point::new(4294967295.0, -4294967296.0);

		assert_eq!(p.x, 4294967295.0);
		assert_eq!(p.y, -4294967296.0);
	}

	#[test]
	fn point_new_06() {
		let p: Point = Point::new(f32::MIN, f32::MAX);

		assert_eq!(p.x, f32::MIN);
		assert_eq!(p.y, f32::MAX);
	}

	#[test]
	fn point_new_07() {
		let p: Point = Point::new(f32::NEG_INFINITY, f32::INFINITY);

		assert_eq!(p.x, f32::NEG_INFINITY);
		assert_eq!(p.y, f32::INFINITY);
	}

	#[test]
	fn point_new_08() {
		let p: Point = Point::new(f32::NAN, f32::NAN);

		assert_eq!(f32::is_nan(p.x), true);
		assert_eq!(f32::is_nan(p.y), true);
	}

	#[test]
	fn point_zero_00() {
		let p: Point = Point::zero();

		assert_eq!(p.x, 0.0);
		assert_eq!(p.y, 0.0);
	}

	#[test]
	fn point_distance_00() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, 0.0);

		assert_eq!(p0.distance(&p1), 0.0);
	}

	#[test]
	fn point_distance_01() {
		let p0: Point = Point::new(42.0, 21.0);
		let p1: Point = Point::new(42.0, -21.0);

		assert_eq!(p0.distance(&p1), 42.0);
	}

	#[test]
	fn point_distance_02() {
		let p0: Point = Point::new(42.0, -21.0);
		let p1: Point = Point::new(42.0, 21.0);

		assert_eq!(p0.distance(&p1), 42.0);
	}

	#[test]
	fn point_distance_03() {
		let p0: Point = Point::new(13.0, -5.0);
		let p1: Point = Point::new(-1.0, -5.0);

		assert_eq!(p0.distance(&p1), 14.0);
	}

	#[test]
	fn point_distance_04() {
		let p0: Point = Point::new(-1.0, -5.0);
		let p1: Point = Point::new(13.0, -5.0);

		assert_eq!(p0.distance(&p1), 14.0);
	}

	#[test]
	fn point_distance_05() {
		let p0: Point = Point::new(1.0, 3.0);
		let p1: Point = Point::new(3.0, 2.0);

		assert_eq!(p0.distance(&p1), 2.236068);
	}

	#[test]
	fn point_distance_06() {
		let p0: Point = Point::new(-1.0, -3.0);
		let p1: Point = Point::new(3.0, 2.0);

		assert_eq!(p0.distance(&p1), 6.4031243);
	}

	#[test]
	fn point_distance_07() {
		let p0: Point = Point::new(1.0, 3.0);
		let p1: Point = Point::new(-3.0, -2.0);

		assert_eq!(p0.distance(&p1), 6.4031243);
	}

	#[test]
	fn point_distance_08() {
		let p0: Point = Point::new(-1.0, 3.0);
		let p1: Point = Point::new(3.0, -2.0);

		assert_eq!(p0.distance(&p1), 6.4031243);
	}

	#[test]
	fn point_distance_09() {
		let p0: Point = Point::new(1.0, -3.0);
		let p1: Point = Point::new(-3.0, 2.0);

		assert_eq!(p0.distance(&p1), 6.4031243);
	}

	#[test]
	fn point_distance_10() {
		let p0: Point = Point::new(-1.0, -3.0);
		let p1: Point = Point::new(-3.0, -2.0);

		assert_eq!(p0.distance(&p1), 2.236068);
	}

	#[test]
	fn point_distance_11() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, f32::MAX);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_12() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, f32::MIN);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_13() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MAX, 0.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_14() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MIN, 0.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_15() {
		let p0: Point = Point::new(0.0, f32::MAX / 2.0);
		let p1: Point = Point::new(0.0, f32::MIN / 2.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_16() {
		let p0: Point = Point::new(0.0, f32::MIN / 2.0);
		let p1: Point = Point::new(0.0, f32::MAX / 2.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_17() {
		let p0: Point = Point::new(f32::MAX / 2.0, 0.0);
		let p1: Point = Point::new(f32::MIN / 2.0, 0.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_18() {
		let p0: Point = Point::new(f32::MIN / 2.0, 0.0);
		let p1: Point = Point::new(f32::MAX / 2.0, 0.0);

		assert_eq!(p0.distance(&p1), f32::MAX);
	}

	#[test]
	fn point_distance_19() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MAX / 2.0, f32::MAX / 2.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_20() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MAX / 2.0, f32::MIN / 2.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_21() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MIN / 2.0, f32::MAX / 2.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_22() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::MIN / 2.0, f32::MIN / 2.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_23() {
		let p0: Point = Point::new(f32::MAX / 2.0, f32::MAX / 2.0);
		let p1: Point = Point::new(0.0, 0.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_24() {
		let p0: Point = Point::new(f32::MAX / 2.0, f32::MIN / 2.0);
		let p1: Point = Point::new(0.0, 0.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_25() {
		let p0: Point = Point::new(f32::MIN / 2.0, f32::MAX / 2.0);
		let p1: Point = Point::new(0.0, 0.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_26() {
		let p0: Point = Point::new(f32::MIN / 2.0, f32::MIN / 2.0);
		let p1: Point = Point::new(0.0, 0.0);

		assert_eq!(p0.distance(&p1), 240615954826175140000000000000000000000.0);
	}

	#[test]
	fn point_distance_27() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, f32::INFINITY);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_28() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, f32::NEG_INFINITY);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_29() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::INFINITY, 0.0);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_30() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::NEG_INFINITY, 0.0);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_31() {
		let p0: Point = Point::new(0.0, f32::INFINITY);
		let p1: Point = Point::new(0.0, f32::INFINITY);

		assert_eq!(p0.distance(&p1), 0.0);
	}

	#[test]
	fn point_distance_32() {
		let p0: Point = Point::new(0.0, f32::INFINITY);
		let p1: Point = Point::new(0.0, f32::NEG_INFINITY);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_33() {
		let p0: Point = Point::new(0.0, f32::INFINITY);
		let p1: Point = Point::new(f32::INFINITY, 0.0);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_34() {
		let p0: Point = Point::new(0.0, f32::INFINITY);
		let p1: Point = Point::new(f32::NEG_INFINITY, 0.0);

		assert_eq!(p0.distance(&p1), f32::INFINITY);
	}

	#[test]
	fn point_distance_35() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(0.0, f32::NAN);

		assert_eq!(f32::is_nan(p0.distance(&p1)), true);
	}

	#[test]
	fn point_distance_36() {
		let p0: Point = Point::new(0.0, 0.0);
		let p1: Point = Point::new(f32::NAN, 0.0);

		assert_eq!(f32::is_nan(p0.distance(&p1)), true);
	}

	#[test]
	fn point_distance_37() {
		let p0: Point = Point::new(0.0, f32::NAN);
		let p1: Point = Point::new(0.0, f32::NAN);

		assert_eq!(f32::is_nan(p0.distance(&p1)), true);
	}

	#[test]
	fn point_distance_38() {
		let p0: Point = Point::new(0.0, f32::NAN);
		let p1: Point = Point::new(f32::NAN, 0.0);

		assert_eq!(f32::is_nan(p0.distance(&p1)), true);
	}
}
