struct Fibs {
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
	/// let fibs = Fibs::new(0, 1);
	/// ```
	pub const fn new(first: u32, second: u32) -> Fibs {
		Fibs { first, second }
	}
}

// TODO: Implement the Iterator trait for Fibs.

#[cfg(test)]
mod tests {
	use super::*;
}
