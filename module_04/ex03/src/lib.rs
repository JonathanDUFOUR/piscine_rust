#[derive(Debug, Eq, PartialEq)]
pub struct Fibs {
	first: u32,
	second: u32,
}

impl Fibs {
	/// Create a new [Fibs] instance and initialize its attributes.
	///
	/// # Parameters
	/// * `first` - The first number of the sequence.
	/// * `second` - The second number of the sequence.
	///
	/// # Returns
	/// The newly created [Fibs] instance.
	///
	/// # Examples
	/// ```
	/// use ex03::Fibs;
	///
	/// const FIB: Fibs = Fibs::new(0, 1);
	/// ```
	pub const fn new(first: u32, second: u32) -> Fibs {
		Fibs { first, second }
	}
}

impl Iterator for Fibs {
	type Item = u32;

	fn next(self: &mut Self) -> Option<Self::Item> {
		match self.first.checked_add(self.second) {
			Some(third) => {
				let first: u32 = self.first;

				self.first = self.second;
				self.second = third;

				Some(first)
			}
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// region: new_00
	#[test]
	fn new_00() {
		const FIB: Fibs = Fibs::new(0, 0);

		assert_eq!(
			FIB,
			Fibs {
				first: 0,
				second: 0
			}
		);
	}
	// endregion

	// region: new_01
	#[test]
	fn new_01() {
		const FIB: Fibs = Fibs::new(0, 1);

		assert_eq!(
			FIB,
			Fibs {
				first: 0,
				second: 1
			}
		);
	}
	// endregion

	// region: new_02
	#[test]
	fn new_02() {
		const FIB: Fibs = Fibs::new(0, u32::MAX);

		assert_eq!(
			FIB,
			Fibs {
				first: 0,
				second: u32::MAX
			}
		);
	}
	// endregion

	// region: new_03
	#[test]
	fn new_03() {
		const FIB: Fibs = Fibs::new(2, 0);

		assert_eq!(
			FIB,
			Fibs {
				first: 2,
				second: 0
			}
		);
	}
	// endregion

	// region: new_04
	#[test]
	fn new_04() {
		const FIB: Fibs = Fibs::new(3, 4);

		assert_eq!(
			FIB,
			Fibs {
				first: 3,
				second: 4
			}
		);
	}
	// endregion

	// region: new_05
	#[test]
	fn new_05() {
		const FIB: Fibs = Fibs::new(5, u32::MAX);

		assert_eq!(
			FIB,
			Fibs {
				first: 5,
				second: u32::MAX
			}
		);
	}
	// endregion

	// region: new_06
	#[test]
	fn new_06() {
		const FIB: Fibs = Fibs::new(u32::MAX, 0);

		assert_eq!(
			FIB,
			Fibs {
				first: u32::MAX,
				second: 0
			}
		);
	}
	// endregion

	// region: new_07
	#[test]
	fn new_07() {
		const FIB: Fibs = Fibs::new(u32::MAX, 6);

		assert_eq!(
			FIB,
			Fibs {
				first: u32::MAX,
				second: 6
			}
		);
	}
	// endregion

	// region: new_08
	#[test]
	fn new_08() {
		const FIB: Fibs = Fibs::new(u32::MAX, u32::MAX);

		assert_eq!(
			FIB,
			Fibs {
				first: u32::MAX,
				second: u32::MAX
			}
		);
	}
	// endregion

	// region: next_00
	#[test]
	fn next_00() {
		let mut fib: Fibs = Fibs::new(0, 0);

		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(0));
	}
	// endregion

	// region: next_01
	#[test]
	fn next_01() {
		let mut fib: Fibs = Fibs::new(0, 1);

		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(1));
		assert_eq!(fib.next(), Some(1));
		assert_eq!(fib.next(), Some(2));
		assert_eq!(fib.next(), Some(3));
		assert_eq!(fib.next(), Some(5));
		assert_eq!(fib.next(), Some(8));
		assert_eq!(fib.next(), Some(13));
	}
	// endregion

	// region: next_02
	#[test]
	fn next_02() {
		let mut fib: Fibs = Fibs::new(123, 456);

		assert_eq!(fib.next(), Some(123));
		assert_eq!(fib.next(), Some(456));
		assert_eq!(fib.next(), Some(579));
		assert_eq!(fib.next(), Some(1035));
		assert_eq!(fib.next(), Some(1614));
		assert_eq!(fib.next(), Some(2649));
		assert_eq!(fib.next(), Some(4263));
		assert_eq!(fib.next(), Some(6912));
	}
	// endregion

	// region: next_03
	#[test]
	fn next_03() {
		let mut fib: Fibs = Fibs::new(999988877, 666655544);

		assert_eq!(fib.next(), Some(999988877));
		assert_eq!(fib.next(), Some(666655544));
		assert_eq!(fib.next(), Some(1666644421));
		assert_eq!(fib.next(), Some(2333299965));
		assert_eq!(fib.next(), Some(3999944386));
		assert_eq!(fib.next(), None);
		assert_eq!(fib.next(), None);
		assert_eq!(fib.next(), None);
	}
	// endregion
}
