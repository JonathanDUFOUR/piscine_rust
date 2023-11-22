///
pub struct Groups<'a, F> {
	s: &'a str,
	f: F,
}

impl<'a, F> Groups<'a, F> {
	/// Creates a new Groups instance and initializes its attributes.
	///
	/// ### Parameters
	/// * `s` - The string to iterate over, and apply the
	///
	/// ### Return
	/// The newly created Groups instance.
	///
	/// ### Examples
	/// ```
	/// use ex05::Groups;
	///
	/// type F = fn(char) -> bool;
	///
	/// let groups: Groups<'_, F> = Groups::new("Hello Rust!", |c| c.is_alphabetic());
	/// ```
	pub fn new(s: &'a str, f: F) -> Self
	where
		F: FnMut(char) -> bool,
	{
		Groups { s, f }
	}
}

impl<'a, F> Iterator for Groups<'a, F>
where
	F: FnMut(char) -> bool,
{
	type Item = &'a str;

	/// Searches for the next group of characters that satisfy the predicate.
	///
	/// ### Return
	/// * `Some(group)` - The next group of characters that satisfy the predicate.
	/// * `None` - There are no more groups of characters that satisfy the predicate.
	///
	/// ### Examples
	/// ```
	/// use ex05::Groups;
	///
	/// type F = fn(char) -> bool;
	///
	/// let mut groups: Groups<'_, F> = Groups::new("Hello Rust!", |c| c.is_alphabetic());
	///
	/// assert_eq!(groups.next(), Some("Hello"));
	/// assert_eq!(groups.next(), Some("Rust"));
	/// assert_eq!(groups.next(), None);
	/// ```
	fn next(&mut self) -> Option<Self::Item> {
		for (i, c) in self.s.char_indices() {
			if (self.f)(c) {
				self.s = &self.s[i..];
				break;
			}
		}
		for (i, c) in self.s.char_indices() {
			if !(self.f)(c) {
				let group: &str = &self.s[..i];

				self.s = &self.s[i..];

				return Some(group);
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	type F = fn(char) -> bool;

	const CHARS: [char; 11] = ['a', '#', '2', ' ', 'ç', '\t', '¥', 'D', '\n', '¬', '5'];

	fn f(c: char) -> bool {
		c.is_whitespace()
	}

	// region: groups_new_00
	#[test]
	fn groups_new_00() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), f(c));
		}
	}
	// endregion

	// region: groups_new_01
	#[test]
	fn groups_new_01() {
		const S: &str = "a";

		let groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), f(c));
		}
	}
	// endregion

	// region: groups_new_02
	#[test]
	fn groups_new_02() {
		const S: &str = "42";

		let groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.s, S);
	}
	// endregion

	// region: groups_new_03
	#[test]
	fn groups_new_03() {
		const S: &str = "Hello Rust!";

		let groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.s, S);
	}
	// endregion

	// region: groups_next_00
	#[test]
	fn groups_next_00() {
		const S: &str = "";

		let mut groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_01
	#[test]
	fn groups_next_01() {
		const S: &str = "Q";

		let mut groups: Groups<'_, F> = Groups::new(S, f);

		assert_eq!(groups.next(), None);
	}
	// endregion
}
