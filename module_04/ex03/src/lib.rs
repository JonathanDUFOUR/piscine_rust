struct Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: PartialOrd,
{
	inner: I,
	last: Option<I::Item>,
}

impl<I> Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: PartialOrd,
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
		Self { inner: collection.into_iter(), last: None }
	}
}

impl<I> Iterator for Increasing<I>
where
	I: Iterator,
	<I as Iterator>::Item: PartialOrd,
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
		match self.last {
			Some(last) => {
				while let Some(next) = self.inner.next() {
					if next > last {
						self.last = Some(next);
						return Some(next);
					}
				}
				None
			}
			None => match self.inner.next() {
				Some(next) => {
					self.last = Some(next);
					Some(next)
				}
				None => None,
			},
		}
	}
}
