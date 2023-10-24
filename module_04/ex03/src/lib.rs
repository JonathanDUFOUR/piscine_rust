#[derive(Debug)]
pub struct Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: Clone + PartialOrd,
{
	inner: I,
	previous: Option<I::Item>,
}

impl<I> Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: Clone + PartialOrd,
{
	/// Creates a new Increasing iterator instance and initializes its attributes.
	/// The newly created Increasing iterator instance will iterate over a given collection
	/// skipping every element that is not strictly greater than
	/// the element that this same iterator was previously on.
	///
	/// # Type parameters
	/// - `C`: the type of the collection to iterate over.
	///
	/// # Parameters
	/// - `collection`: the collection to iterate over.
	///
	/// # Return
	/// The newly created Increasing iterator instance.
	///
	/// # Example
	/// ```
	/// use ex03::Increasing;
	///
	/// let mut it = Increasing::new([1, 2, 2, 3, 3, 3]);
	/// ```
	pub fn new<C>(collection: C) -> Self
	where
		C: IntoIterator<IntoIter = I>,
	{
		Self { inner: collection.into_iter(), previous: None }
	}
}

impl<I> Iterator for Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: Clone + PartialOrd,
{
	type Item = I::Item;

	/// Advances the iterator to the next element that is strictly greater than
	/// the element that this same iterator was previously on.
	///
	/// # Return
	/// * `Some(<I::Item>)` - The next element that fits the mentioned constraint.
	/// * `None` - There are no more elements that fits the mentioned constraint.
	///
	/// # Example
	/// ```
	/// use ex03::Increasing;
	///
	/// let mut it = Increasing::new([1, 2, 2, 3, 3, 3]);
	///
	/// assert_eq!(it.next(), Some(1));
	/// assert_eq!(it.next(), Some(2));
	/// assert_eq!(it.next(), Some(3));
	/// assert_eq!(it.next(), None);
	/// ```
	fn next(self: &mut Self) -> Option<Self::Item> {
		match self.previous.take() {
			Some(previous) => {
				while let Some(next) = self.inner.next() {
					if next > previous {
						self.previous = Some(next);
						return self.previous.clone();
					}
				}
				self.previous = None;

				None
			}
			None => {
				self.previous = self.inner.next();

				self.previous.clone()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	impl<I> PartialEq for Increasing<I>
	where
		I: Clone + Iterator,
		<I as Iterator>::Item: Clone + PartialOrd,
	{
		fn eq(self: &Self, rhs: &Self) -> bool {
			self.inner.clone().eq(rhs.inner.clone()) && self.previous.eq(&rhs.previous)
		}
	}

	// region: new_00
	#[test]
	fn new_00() {
		let a: [u8; 0] = [];
		let it: Increasing<std::array::IntoIter<u8, 0>> = Increasing::new(a);

		assert_eq!(it, Increasing { inner: a.into_iter(), previous: None });
	}
	// endregion

	// region: new_01
	#[test]
	fn new_01() {
		let a: [u16; 1] = [567];
		let it: Increasing<std::array::IntoIter<u16, 1>> = Increasing::new(a);

		assert_eq!(it, Increasing { inner: a.into_iter(), previous: None });
	}
	// endregion

	// region: new_02
	#[test]
	fn new_02() {
		let v: Vec<u32> = vec![0, 1, 2, 3, 4, 3, 2, 1, 0];
		let it: Increasing<std::vec::IntoIter<u32>> = Increasing::new(v.clone());

		assert_eq!(it, Increasing { inner: v.into_iter(), previous: None });
	}
	// endregion

	// region: new_03
	#[test]
	fn new_03() {
		use std::collections::LinkedList;

		let l: LinkedList<u64> = LinkedList::from_iter(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
		let it: Increasing<std::collections::linked_list::IntoIter<u64>> =
			Increasing::new(l.clone());

		assert_eq!(it, Increasing { inner: l.into_iter(), previous: None });
	}
	// endregion

	// region: next_00
	#[test]
	fn next_00() {
		let a: [u8; 0] = [];
		let mut it: Increasing<std::array::IntoIter<u8, 0>> = Increasing::new(a);

		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_01
	#[test]
	fn next_01() {
		let a: [u16; 1] = [0];
		let mut it: Increasing<std::array::IntoIter<u16, 1>> = Increasing::new(a);

		assert_eq!(it.next(), Some(0));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_02
	#[test]
	fn next_02() {
		let v: Vec<u32> = vec![0, 1, 2, 3, 4, 3, 2, 1, 0];
		let mut it: Increasing<std::vec::IntoIter<u32>> = Increasing::new(v);

		assert_eq!(it.next(), Some(0));
		assert_eq!(it.next(), Some(1));
		assert_eq!(it.next(), Some(2));
		assert_eq!(it.next(), Some(3));
		assert_eq!(it.next(), Some(4));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_03
	#[test]
	fn next_03() {
		use std::collections::LinkedList;

		let ll: LinkedList<u64> = LinkedList::from_iter(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
		let mut it: Increasing<std::collections::linked_list::IntoIter<u64>> = Increasing::new(ll);

		assert_eq!(it.next(), Some(9));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_04
	#[test]
	fn next_04() {
		use std::collections::VecDeque;

		let vd: VecDeque<u64> = VecDeque::from_iter(vec![0, 1, 0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3]);
		let mut it: Increasing<std::collections::vec_deque::IntoIter<u64>> = Increasing::new(vd);

		assert_eq!(it.next(), Some(0));
		assert_eq!(it.next(), Some(1));
		assert_eq!(it.next(), Some(2));
		assert_eq!(it.next(), Some(3));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_05
	#[test]
	fn next_05() {
		let a: [u64; 5] = [
			// region: a
			u64::MAX - 4,
			u64::MAX - 3,
			u64::MAX - 2,
			u64::MAX - 1,
			u64::MAX - 0,
			// endregion
		];
		let mut it: Increasing<std::array::IntoIter<u64, 5>> = Increasing::new(a);

		assert_eq!(it.next(), Some(u64::MAX - 4));
		assert_eq!(it.next(), Some(u64::MAX - 3));
		assert_eq!(it.next(), Some(u64::MAX - 2));
		assert_eq!(it.next(), Some(u64::MAX - 1));
		assert_eq!(it.next(), Some(u64::MAX - 0));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_06
	#[test]
	fn next_06() {
		let v: Vec<f32> = vec![f32::NEG_INFINITY, -3.14, 3.14, 0.0, f32::NAN, f32::INFINITY];
		let mut it: Increasing<std::vec::IntoIter<f32>> = Increasing::new(v);

		assert_eq!(it.next(), Some(f32::NEG_INFINITY));
		assert_eq!(it.next(), Some(-3.14));
		assert_eq!(it.next(), Some(3.14));
		assert_eq!(it.next(), Some(f32::INFINITY));
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion

	// region: next_07
	#[test]
	fn next_07() {
		let v: Vec<f64> = vec![
			// region: v
			f64::NAN,
			f64::NEG_INFINITY,
			-56789.01234,
			-42.42,
			0.0,
			21.21,
			183729.349167,
			f64::INFINITY,
			f64::NAN,
			// endregion
		];
		let mut it: Increasing<std::vec::IntoIter<f64>> = Increasing::new(v);

		assert!(it.next().unwrap().is_nan());
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
		assert_eq!(it.next(), None);
	}
	// endregion
}
