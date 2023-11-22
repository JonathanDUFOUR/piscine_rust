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
		match self.s.char_indices().find(|(_, c)| (self.f)(*c)) {
			Some((i0, _)) => {
				self.s = &self.s[i0..];
				match self.s.char_indices().find(|(_, c)| !(self.f)(*c)) {
					Some((i1, _)) => {
						let (group, rest) = self.s.split_at(i1);
						self.s = rest;
						Some(group)
					}
					None => {
						let group = self.s;
						self.s = "";
						Some(group)
					}
				}
			}
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	type F = fn(char) -> bool;

	const CHARS: [char; 13] = [
		// region: CHARS
		'a', '#', '2', ' ', 'ç', '\t', '¥', 'D', '\n', '¬', '5', 'Û', 'V',
		// endregion
	];

	fn is_alphabetic(c: char) -> bool {
		c.is_alphabetic()
	}

	const fn is_ascii(c: char) -> bool {
		c.is_ascii()
	}

	const fn is_ascii_digit(c: char) -> bool {
		c.is_ascii_digit()
	}

	fn is_lowercase(c: char) -> bool {
		c.is_lowercase()
	}

	fn is_uppercase(c: char) -> bool {
		c.is_uppercase()
	}

	fn is_whitespace(c: char) -> bool {
		c.is_whitespace()
	}

	// region: groups_new_00
	#[test]
	fn groups_new_00() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_alphabetic);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_alphabetic(c));
		}
	}
	// endregion

	// region: groups_new_01
	#[test]
	fn groups_new_01() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii(c));
		}
	}
	// endregion

	// region: groups_new_02
	#[test]
	fn groups_new_02() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii_digit);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii_digit(c));
		}
	}
	// endregion

	// region: groups_new_03
	#[test]
	fn groups_new_03() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_lowercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_lowercase(c));
		}
	}
	// endregion

	// region: groups_new_04
	#[test]
	fn groups_new_04() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_uppercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_uppercase(c));
		}
	}
	// endregion

	// region: groups_new_05
	#[test]
	fn groups_new_05() {
		const S: &str = "";

		let groups: Groups<'_, F> = Groups::new(S, is_whitespace);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_whitespace(c));
		}
	}
	// endregion

	// region: groups_new_06
	#[test]
	fn groups_new_06() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_alphabetic);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_alphabetic(c));
		}
	}
	// endregion

	// region: groups_new_07
	#[test]
	fn groups_new_07() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii(c));
		}
	}
	// endregion

	// region: groups_new_08
	#[test]
	fn groups_new_08() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii_digit);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii_digit(c));
		}
	}
	// endregion

	// region: groups_new_09
	#[test]
	fn groups_new_09() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_lowercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_lowercase(c));
		}
	}
	// endregion

	// region: groups_new_10
	#[test]
	fn groups_new_10() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_uppercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_uppercase(c));
		}
	}
	// endregion

	// region: groups_new_11
	#[test]
	fn groups_new_11() {
		const S: &str = "z";

		let groups: Groups<'_, F> = Groups::new(S, is_whitespace);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_whitespace(c));
		}
	}
	// endregion

	// region: groups_new_12
	#[test]
	fn groups_new_12() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_alphabetic);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_alphabetic(c));
		}
	}
	// endregion

	// region: groups_new_13
	#[test]
	fn groups_new_13() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii(c));
		}
	}
	// endregion

	// region: groups_new_14
	#[test]
	fn groups_new_14() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_ascii_digit);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_ascii_digit(c));
		}
	}
	// endregion

	// region: groups_new_15
	#[test]
	fn groups_new_15() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_lowercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_lowercase(c));
		}
	}
	// endregion

	// region: groups_new_16
	#[test]
	fn groups_new_16() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_uppercase);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_uppercase(c));
		}
	}
	// endregion

	// region: groups_new_17
	#[test]
	fn groups_new_17() {
		const S: &str = "👾_H31l0 Rust! Nic3  2 m33t U._👾";

		let groups: Groups<'_, F> = Groups::new(S, is_whitespace);

		assert_eq!(groups.s, S);
		for c in CHARS {
			assert_eq!((groups.f)(c), is_whitespace(c));
		}
	}
	// endregion

	// region: groups_next_00
	#[test]
	fn groups_next_00() {
		let mut groups: Groups<'_, F> = Groups::new("", is_alphabetic);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_01
	#[test]
	fn groups_next_01() {
		let mut groups: Groups<'_, F> = Groups::new("", is_ascii);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_02
	#[test]
	fn groups_next_02() {
		let mut groups: Groups<'_, F> = Groups::new("", is_ascii_digit);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_03
	#[test]
	fn groups_next_03() {
		let mut groups: Groups<'_, F> = Groups::new("", is_lowercase);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_04
	#[test]
	fn groups_next_04() {
		let mut groups: Groups<'_, F> = Groups::new("", is_uppercase);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_05
	#[test]
	fn groups_next_05() {
		let mut groups: Groups<'_, F> = Groups::new("", is_whitespace);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_06
	#[test]
	fn groups_next_06() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_alphabetic);

		assert_eq!(groups.next(), Some("foo"));
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_07
	#[test]
	fn groups_next_07() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_ascii);

		assert_eq!(groups.next(), Some("foo"));
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_08
	#[test]
	fn groups_next_08() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_ascii_digit);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_09
	#[test]
	fn groups_next_09() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_lowercase);

		assert_eq!(groups.next(), Some("foo"));
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_10
	#[test]
	fn groups_next_10() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_uppercase);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_11
	#[test]
	fn groups_next_11() {
		let mut groups: Groups<'_, F> = Groups::new("foo", is_whitespace);

		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_12
	#[test]
	fn groups_next_12() {
		const EXPECTED: [&str; 96] = [
			// region: EXPECTED
			"nce",
			"upon",
			"a",
			"time",
			"there",
			"existed",
			"a",
			"giant",
			"tree",
			"that",
			"was",
			"the",
			"source",
			"of",
			"mana",
			"A",
			"war",
			"however",
			"caused",
			"this",
			"tree",
			"to",
			"wither",
			"away",
			"and",
			"a",
			"hero",
			"s",
			"ife",
			"was",
			"sacrificed",
			"in",
			"order",
			"to",
			"take",
			"its",
			"place",
			"Grieving",
			"over",
			"the",
			"loss",
			"the",
			"goddess",
			"disappeared",
			"un",
			"the",
			"heavens",
			"The",
			"goddess",
			"left",
			"the",
			"angels",
			"with",
			"this",
			"edict",
			"You",
			"must",
			"wake",
			"me",
			"if",
			"I",
			"should",
			"sleep",
			"the",
			"world",
			"shall",
			"be",
			"destroyed",
			"The",
			"angels",
			"bore",
			"the",
			"Chosen",
			"ne",
			"who",
			"headed",
			"towards",
			"the",
			"tower",
			"that",
			"reached",
			"up",
			"unto",
			"the",
			"heavens",
			"And",
			"that",
			"marked",
			"the",
			"beginning",
			"of",
			"the",
			"regeneration",
			"of",
			"the",
			"world",
			// endregion
		];

		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_alphabetic,
		);

		for expected in EXPECTED {
			assert_eq!(groups.next(), Some(expected));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_13
	#[test]
	fn groups_next_13() {
		const EXPECTED: [&str; 12] = [
			// region: EXPECTED
			"0nce upon a time, there existed a giant tree ",
			" that was the source of mana ",
			".
			A war, however, caused this tree ",
			" to wither away, and a hero",
			"s life was sacrificed ",
			"
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.
			The goddess left the 3 angels ",
			" with this edict:
			",
			"You must wake me, 4 if I should sleep ",
			", the world shall be destroyed ",
			".",
			"
			The angels ",
			" bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.
			And that marked the beginning of the regeneration of the world.",
			// endregion
		];

		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_ascii,
		);

		for expected in EXPECTED {
			assert_eq!(groups.next(), Some(expected));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_14
	#[test]
	fn groups_next_14() {
		const EXPECTED: [&str; 6] = ["0", "1", "2", "3", "4", "0"];

		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_ascii_digit,
		);

		for expected in EXPECTED {
			assert_eq!(groups.next(), Some(expected));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_15
	#[test]
	fn groups_next_15() {
		const EXPECTED: [&str; 93] = [
			// region: EXPECTED
			"nce",
			"upon",
			"a",
			"time",
			"there",
			"existed",
			"a",
			"giant",
			"tree",
			"that",
			"was",
			"the",
			"source",
			"of",
			"mana",
			"war",
			"however",
			"caused",
			"this",
			"tree",
			"to",
			"wither",
			"away",
			"and",
			"a",
			"hero",
			"ife",
			"was",
			"sacrificed",
			"in",
			"order",
			"to",
			"take",
			"its",
			"place",
			"rieving",
			"over",
			"the",
			"loss",
			"the",
			"goddess",
			"disappeared",
			"un",
			"the",
			"heavens",
			"he",
			"goddess",
			"left",
			"the",
			"angels",
			"with",
			"this",
			"edict",
			"ou",
			"must",
			"wake",
			"me",
			"if",
			"should",
			"sleep",
			"the",
			"world",
			"shall",
			"be",
			"destroyed",
			"he",
			"angels",
			"bore",
			"the",
			"hosen",
			"ne",
			"who",
			"headed",
			"towards",
			"the",
			"tower",
			"that",
			"reached",
			"up",
			"unto",
			"the",
			"heavens",
			"nd",
			"that",
			"marked",
			"the",
			"beginning",
			"of",
			"the",
			"regeneration",
			"of",
			"the",
			"world",
			// endregion
		];
		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_lowercase,
		);

		for expexted in EXPECTED {
			assert_eq!(groups.next(), Some(expexted));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_16
	#[test]
	fn groups_next_16() {
		const EXPECTED: [&str; 8] = ["A", "G", "T", "Y", "I", "T", "C", "A"];

		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_uppercase,
		);

		for expected in EXPECTED {
			assert_eq!(groups.next(), Some(expected));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion

	// region: groups_next_17
	#[test]
	fn groups_next_17() {
		const EXPECTED: [&str; 90] = [
			// region: EXPECTED
			" ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "\n",
			" ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
			" ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "\r\n\t", " ", " ",
			" ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
			" ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "\n", " ", " ", " ", " ",
			" ", " ", " ", " ", " ", " ",
			// endregion
		];
		let mut groups: Groups<'_, F> = Groups::new(
			// region: attribute `s`
			"0nce upon a time, there existed a giant tree 🌳 that was the source of mana ✨.\n
			A war, however, caused this tree 🌳 to wither away, and a hero’s 1ife was sacrificed 💔
			in order to take its place.
			Grieving over the loss, the goddess disappeared un2 the heavens.\r\n\t
			The goddess left the 3 angels 👼👼🏿👼🏽 with this edict: \n
			“You must wake me, 4 if I should sleep 😴, the world shall be destroyed 💥.”
			The angels 👼🏽👼👼🏿 bore the Chosen 0ne,
			who headed towards the tower that reached up unto the heavens.\n
			And that marked the beginning of the regeneration of the world.",
			// endregion
			is_whitespace,
		);

		for expected in EXPECTED {
			assert_eq!(groups.next(), Some(expected));
		}
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
		assert_eq!(groups.next(), None);
	}
	// endregion
}
