/// An iterator through a Fibonacci sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fibs {
	current: Option<u32>,
	next: Option<u32>,
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
		Fibs {
			current: Some(first),
			next: Some(second),
		}
	}

	/// Calculate the sum of all even Fibonacci numbers below 1000.
	///
	/// # Returns
	/// The sum of all even Fibonacci numbers below 1000.
	///
	/// # Example
	/// ```
	/// use ex03::Fibs;
	///
	/// const FIBS: Fibs = Fibs::new(0, 1);
	///
	/// assert_eq!(FIBS.even_fibs_below_1000(), 798);
	/// ```
	pub fn even_fibs_below_1000(self: &Self) -> u32 {
		self.take_while(|&x| x < 1000).filter(|&x| x % 2 == 0).sum()
	}
}

impl Iterator for Fibs {
	type Item = u32;

	fn next(self: &mut Self) -> Option<Self::Item> {
		let current: Option<u32> = self.current;
		let next_next: Option<u32> = match (self.current, self.next) {
			(Some(current), Some(next)) => current.checked_add(next),
			_ => None,
		};

		self.current = self.next;
		self.next = next_next;

		current
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
				current: Some(0),
				next: Some(0)
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
				current: Some(0),
				next: Some(1)
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
				current: Some(0),
				next: Some(u32::MAX)
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
				current: Some(2),
				next: Some(0)
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
				current: Some(3),
				next: Some(4)
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
				current: Some(5),
				next: Some(u32::MAX)
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
				current: Some(u32::MAX),
				next: Some(0)
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
				current: Some(u32::MAX),
				next: Some(6)
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
				current: Some(u32::MAX),
				next: Some(u32::MAX)
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

	// region: next_04
	#[test]
	fn next_04() {
		let mut fib: Fibs = Fibs::new(u32::MAX, 0);

		assert_eq!(fib.next(), Some(u32::MAX));
		assert_eq!(fib.next(), Some(0));
		assert_eq!(fib.next(), Some(u32::MAX));
		assert_eq!(fib.next(), Some(u32::MAX));
		assert_eq!(fib.next(), None);
		assert_eq!(fib.next(), None);
		assert_eq!(fib.next(), None);
		assert_eq!(fib.next(), None);
	}
	// endregion

	// region: even_fibs_below_1000_00
	#[test]
	fn even_fibs_below_1000_00() {
		const FIB: Fibs = Fibs::new(0, 1);

		assert_eq!(FIB.even_fibs_below_1000(), 798);
	}
	// endregion

	// region: even_fibs_below_1000_01
	#[test]
	fn even_fibs_below_1000_01() {
		const FIB: Fibs = Fibs::new(0, 2);

		assert_eq!(FIB.even_fibs_below_1000(), 1972);
	}
	// endregion

	// region: even_fibs_below_1000_02
	#[test]
	fn even_fibs_below_1000_02() {
		const FIB: Fibs = Fibs::new(0, 501);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_03
	#[test]
	fn even_fibs_below_1000_03() {
		const FIB: Fibs = Fibs::new(0, 1000);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_04
	#[test]
	fn even_fibs_below_1000_04() {
		const FIB: Fibs = Fibs::new(0, u32::MAX);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_05
	#[test]
	fn even_fibs_below_1000_05() {
		const FIB: Fibs = Fibs::new(1, 0);

		assert_eq!(FIB.even_fibs_below_1000(), 798);
	}
	// endregion

	// region: even_fibs_below_1000_06
	#[test]
	fn even_fibs_below_1000_06() {
		const FIB: Fibs = Fibs::new(1, 1);

		assert_eq!(FIB.even_fibs_below_1000(), 798);
	}
	// endregion

	// region: even_fibs_below_1000_07
	#[test]
	fn even_fibs_below_1000_07() {
		const FIB: Fibs = Fibs::new(1, 3);

		assert_eq!(FIB.even_fibs_below_1000(), 420);
	}
	// endregion

	// region: even_fibs_below_1000_08
	#[test]
	fn even_fibs_below_1000_08() {
		const FIB: Fibs = Fibs::new(1, 765);

		assert_eq!(FIB.even_fibs_below_1000(), 766);
	}
	// endregion

	// region: even_fibs_below_1000_09
	#[test]
	fn even_fibs_below_1000_09() {
		const FIB: Fibs = Fibs::new(1, 1002);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_10
	#[test]
	fn even_fibs_below_1000_10() {
		const FIB: Fibs = Fibs::new(1, u32::MAX);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_11
	#[test]
	fn even_fibs_below_1000_11() {
		const FIB: Fibs = Fibs::new(42, 0);

		assert_eq!(FIB.even_fibs_below_1000(), 2310);
	}
	// endregion

	// region: even_fibs_below_1000_12
	#[test]
	fn even_fibs_below_1000_12() {
		const FIB: Fibs = Fibs::new(42, 21);

		assert_eq!(FIB.even_fibs_below_1000(), 504);
	}
	// endregion

	// region: even_fibs_below_1000_13
	#[test]
	fn even_fibs_below_1000_13() {
		const FIB: Fibs = Fibs::new(42, 631);

		assert_eq!(FIB.even_fibs_below_1000(), 42);
	}
	// endregion

	// region: even_fibs_below_1000_14
	#[test]
	fn even_fibs_below_1000_14() {
		const FIB: Fibs = Fibs::new(42, 958);

		assert_eq!(FIB.even_fibs_below_1000(), 1000);
	}
	// endregion

	// region: even_fibs_below_1000_15
	#[test]
	fn even_fibs_below_1000_15() {
		const FIB: Fibs = Fibs::new(42, u32::MAX - 1);

		assert_eq!(FIB.even_fibs_below_1000(), 42);
	}
	// endregion

	// region: even_fibs_below_1000_16
	#[test]
	fn even_fibs_below_1000_16() {
		const FIB: Fibs = Fibs::new(1000, 1000);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion

	// region: even_fibs_below_1000_17
	#[test]
	fn even_fibs_below_1000_17() {
		const FIB: Fibs = Fibs::new(u32::MAX, u32::MAX);

		assert_eq!(FIB.even_fibs_below_1000(), 0);
	}
	// endregion
}
