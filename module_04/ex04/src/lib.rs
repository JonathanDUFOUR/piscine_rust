type Integer = u8;
type BitField = u16;

/// A fixed sized bitset.
struct BitSet {
	inner: [BitField; Self::LEN],
}

// region: impl BitSet
impl BitSet {
	/// The maximum number of usable bits in each BitSet instance.
	const BITS: Integer = 42;

	/// This guard is here to ensure that `Self::BITS` is strictly greater than 0.
	const _BITS_GUARD: Integer = Self::BITS - 1;

	/// The number of BitField required to store `BITS` bits.
	const LEN: usize = {
		const LEN: Integer = BitSet::BITS / BitField::BITS as Integer;

		match LEN * BitField::BITS as Integer {
			BitSet::BITS => LEN as usize,
			____________ => LEN as usize + 1,
		}
	};

	/// Creates a new BitSet instance and initializes it as entierly filled with ones.
	///
	/// ### Return
	/// The newly created BitSet instance.
	#[inline(always)]
	const fn new() -> Self {
		Self { inner: [!0; Self::LEN] }
	}

	/// Get the value of the `n`th bit in the bit set.
	///
	/// ### Parameters
	/// * `n` - The position of the bit.
	///
	/// ### Return
	/// * `true` - The bit is set to 1.
	/// * `false` - The bit is set to 0.
	///
	/// ### Panic
	/// `n` is out of bounds.
	#[inline(always)]
	fn get_bit(self: &Self, n: Integer) -> bool {
		if n >= Self::BITS {
			panic!("n({n}) is out of bounds.");
		}

		let i: usize = (n / BitField::BITS as Integer) as usize;
		let bit_position_in_field: Integer = n % BitField::BITS as Integer;

		self.inner[i] >> bit_position_in_field & 1 == 1
	}

	/// Clears the `n`th bit in the bit set to 0.
	///
	/// ### Parameters
	/// * `n` - The position of the bit.
	///
	/// ### Panic
	/// `n` is out of bounds.
	#[inline(always)]
	fn clear_bit(self: &mut Self, n: Integer) {
		if n >= Self::BITS {
			panic!("n({n}) is out of bounds.");
		}

		let i: usize = (n / BitField::BITS as Integer) as usize;
		let mask: BitField = !(1 << n % BitField::BITS as Integer);

		self.inner[i] &= mask;
	}

	/// Searches for the first bit that is set to 1 in the `n` first bits of the bit set.
	///
	/// ### Parameters
	/// * `n` - The number of bits to check.
	///
	/// ### Return
	/// * `Some(n)` - The position of the first bit that is set to 1.
	/// * `None` - There is no bit that is set to 1.
	///
	/// ### Panic
	/// `n` is greater than the number of bits in `self`.
	fn find_first_set_bit(self: &Self, n: Integer) -> Option<Integer> {
		if n > Self::BITS {
			panic!("n({n}) is greater than the number of bits in the BitSet({}).", Self::BITS);
		}

		let len: usize = (n / BitField::BITS as Integer) as usize;

		for i in 0..len {
			if self.inner[i] != 0 {
				for bit_position_in_field in 0..BitField::BITS {
					if self.inner[i] >> bit_position_in_field & 1 == 1 {
						return Some(
							i as Integer * BitField::BITS as Integer
								+ bit_position_in_field as Integer,
						);
					}
				}
			}
		}

		let remaining_bits_to_check: Integer = n % BitField::BITS as Integer;

		for bit_position_in_field in 0..remaining_bits_to_check {
			if self.inner[len] >> bit_position_in_field & 1 == 1 {
				return Some(
					len as Integer * BitField::BITS as Integer + bit_position_in_field as Integer,
				);
			}
		}

		None
	}
}
// endregion

/// An implementation of the Sieve of Eratosthenes.
/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
/// This implementation uses multiple limited chunks of numbers
/// instead of a single huge chunk, allowing to find prime numbers to whatever limit we want,
/// without having to allocate a huge memory area.
pub struct Sieve {
	/// A vector that contains the prime numbers that have already been found,
	/// sorted in ascending order.
	primes_found_so_far: Vec<Integer>,

	/// The inner bitset that represents the numbers in the range.
	/// * 0 means that the represented number is not prime.
	/// * 1 means that the represented number is prime.
	inner: BitSet,

	/// The number represented by the first bit of `self.inner`.
	first: Integer,

	/// The number of remaining numbers that have not yet been computed by the sieve.
	remaining_numbers: Integer,

	/// The number of numbers that are considered by the sieve for the current chunk.
	len: Integer,
}

// region: impl Sieve
impl Sieve {
	/// Creates a new Sieve instance and initializes its attributes.
	/// The newly created Sieve instance is used to find all the prime numbers
	/// up to whatever limit we want.
	///
	/// ### Return
	/// The newly created Sieve instance.
	///
	/// ### Example
	/// ```
	/// use ex04::Sieve;
	///
	/// let sieve: Sieve = Sieve::new();
	/// ```
	#[inline(always)]
	pub fn new() -> Self {
		const fn min(a: Integer, b: Integer) -> Integer {
			if a < b {
				a
			} else {
				b
			}
		}

		const INNER: BitSet = BitSet::new();
		const FIRST: Integer = 2;
		const REMAINING_NUMBERS: Integer = Integer::MAX - FIRST + 1;
		const LEN: Integer = min(BitSet::BITS, REMAINING_NUMBERS);

		let mut sieve: Self = Self {
			primes_found_so_far: Vec::new(),
			inner: INNER,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: LEN,
		};

		sieve.remove_non_primes();

		sieve
	}

	/// Update inner attributes to consider the next chunk of numbers.
	/// All the numbers of the next chunk are considered prime by default.
	/// The non-prime numbers will be removed later.
	fn fill_with_next_chunk(self: &mut Self) {
		self.inner = BitSet::new();
		if let Some(sum) = self.first.checked_add(self.len) {
			self.first = sum;
		}
		self.remaining_numbers -= self.len;
		self.len = BitSet::BITS.min(self.remaining_numbers);
	}

	/// Remove the non-prime numbers from the current chunk of numbers.
	/// The non-prime numbers are found by multiplying the prime numbers
	/// that we have found so far, and then multiplying the remaining
	/// numbers in the chunk from the itself (Yes, it sounds like an Inception).
	fn remove_non_primes(self: &mut Self) {
		#[inline(always)]
		fn remove_prime_multiples(
			multiple: Integer,
			first: Integer,
			inner: &mut BitSet,
			prime: Integer,
			len: Integer,
		) {
			let mut bit_position: Integer = multiple - first;

			while bit_position < len {
				inner.clear_bit(bit_position);
				match bit_position.checked_add(prime) {
					Some(sum) => bit_position = sum,
					None => break,
				}
			}
		}

		for prime in &self.primes_found_so_far {
			let multiple: Integer = match self.first.checked_next_multiple_of(*prime) {
				Some(multiple) => multiple,
				None => continue,
			};

			remove_prime_multiples(multiple, self.first, &mut self.inner, *prime, self.len);
		}

		for bit_position in 0..self.len {
			if self.inner.get_bit(bit_position) {
				let prime: Integer = self.first + bit_position;

				let multiple: Integer = match prime.checked_mul(prime) {
					Some(square) => square,
					None => continue,
				};

				remove_prime_multiples(multiple, self.first, &mut self.inner, prime, self.len);
			}
		}
	}

	/// Searches for the first prime number that is greater
	/// than the greatest prime number found so far
	/// and save it as the new greatest prime number found so far.
	///
	/// ### Return
	/// * `Some(prime)` - The new greatest prime number found so far.
	/// * `None` - There is no next prime number.
	fn find_next_prime(self: &mut Self) -> Option<Integer> {
		loop {
			match self.inner.find_first_set_bit(self.len) {
				Some(bit_position) => {
					let prime: Integer = self.first + bit_position;

					self.inner.clear_bit(bit_position);
					self.primes_found_so_far.push(prime);

					return Some(prime);
				}
				None => {
					if self.remaining_numbers == 0 {
						return None;
					}

					self.fill_with_next_chunk();
					self.remove_non_primes();
				}
			}
		}
	}
}
// endregion

/// Searches in `v` for the first element that is __greater or equal__ to `n`.
/// It is assumed that `v` is sorted in ascending order
/// and that `n` is lower or equal to the last element of `v`.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The number to search the lower bound for.
///
/// # Panic
/// `n` is strictly greater than the last elements of `v`.
////
/// # Return
/// The value of the first element in `v` that is greater or equal to `n`.
fn lower_bound(v: &Vec<Integer>, n: Integer) -> Option<Integer> {
	let mut left: usize = 0;
	let mut right: usize = v.len();

	while left < right {
		let mid: usize = left + (right - left) / 2;

		if v[mid] < n {
			left = mid + 1;
		} else {
			right = mid;
		}
	}

	if left < v.len() {
		Some(v[left])
	} else {
		None
	}
}

/// An iterator that generates prime numbers.
pub struct Prime {
	/// The number to find the next prime from.
	n: Integer,

	/// The sieve of Eratosthenes that is used to find the next prime number.
	sieve: Sieve,

	/// A boolean that indicates if the end of the iterator has been reached.
	is_end_reached: bool,
}

// region: impl Prime
impl Prime {
	/// Creates a new Prime iterator instance and initializes its attributes.
	/// The newly created Prime iterator instance is used to get the prime numbers
	/// starting at `n`, generating the next one at each iteration.
	///
	/// ### Paramters
	/// * `n` - The starting number.
	///
	/// ### Return
	/// The newly created Prime iterator instance.
	///
	/// ### Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new(0);
	/// ```
	pub fn new(n: Integer) -> Self {
		Self { n, sieve: Sieve::new(), is_end_reached: false }
	}
}
// endregion

// region: impl Iterator for Prime
impl Iterator for Prime {
	type Item = Integer;

	/// Generates the next prime number.
	///
	/// ### Return
	/// * `Some(n)` - The next prime number.
	/// * `None` - There is no next prime number.
	///
	/// ### Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new(0);
	///
	/// assert_eq!(prime.next(), Some(2));
	/// assert_eq!(prime.next(), Some(3));
	/// assert_eq!(prime.next(), Some(5));
	/// assert_eq!(prime.next(), Some(7));
	/// assert_eq!(prime.next(), Some(11));
	/// ```
	fn next(self: &mut Self) -> Option<Self::Item> {
		if self.is_end_reached {
			return None;
		}

		let next_prime: Integer;

		if let Some(lb) = lower_bound(&self.sieve.primes_found_so_far, self.n) {
			next_prime = lb;
			if let Some(sum) = lb.checked_add(1) {
				if let Some(lb) = lower_bound(&self.sieve.primes_found_so_far, sum) {
					self.n = lb;
				} else if let Some(prime) = self.sieve.find_next_prime() {
					self.n = prime;
				} else {
					self.is_end_reached = true;
				}
			} else {
				self.is_end_reached = true;
			}
		} else {
			loop {
				if let Some(prime) = self.sieve.find_next_prime() {
					if prime >= self.n {
						next_prime = prime;
						break;
					}
				} else {
					self.is_end_reached = true;
					return None;
				}
			}

			if let Some(prime) = self.sieve.find_next_prime() {
				self.n = prime;
			} else {
				self.is_end_reached = true;
			}
		}

		Some(next_prime)
	}
}
// endregion

#[cfg(test)]
mod tests {
	use super::*;

	const PRIMES: [Integer; 54] = [
		// region: PRIMES
		2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
		97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
		191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
		// endregion
	];

	#[inline(always)]
	fn test_bit_set_clear_bit(bs: &mut BitSet) {
		for bit_position in 0..BitSet::BITS {
			let i: usize = (bit_position / BitField::BITS as Integer) as usize;
			let bit_position_in_field: Integer = bit_position % BitField::BITS as Integer;

			bs.clear_bit(bit_position);
			assert_eq!(bs.inner[i] >> bit_position_in_field & 1, 0);
		}
	}

	fn check_sieve_inner_bit_set(bs: &BitSet, len: Integer, first: Integer) {
		const PRIMES_LAST: Integer = PRIMES[PRIMES.len() - 1];

		for bit_position in 0..len.min(PRIMES_LAST + 1) {
			let i: usize = (bit_position / BitField::BITS as Integer) as usize;
			let bit_position_in_field: Integer = bit_position % BitField::BITS as Integer;

			match PRIMES.binary_search(&(first + bit_position)) {
				Ok(__) => assert_eq!(bs.inner[i] >> bit_position_in_field & 1, 1),
				Err(_) => assert_eq!(bs.inner[i] >> bit_position_in_field & 1, 0),
			}
		}
	}

	// region: bit_set_new_00
	#[test]
	fn bit_set_new_00() {
		const BS: BitSet = BitSet::new();

		assert_eq!(BS.inner, [!0; BitSet::LEN]);
	}
	// endregion

	// region: bit_set_get_bit_00
	#[test]
	fn bit_set_get_bit_00() {
		const BS: BitSet = BitSet { inner: [0; BitSet::LEN] };
		const LEN: Integer = match BitSet::BITS % BitField::BITS as Integer {
			0 => BitSet::LEN as Integer,
			_ => BitSet::LEN as Integer - 1,
		};
		const LAST_BIT_FIELD_BITS: Integer = BitSet::BITS as Integer - LEN;

		for i in 0..LEN {
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 7), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 6), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 5), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 4), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 3), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 2), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 1), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 0), false);
		}
		for bit_position_in_field in 0..LAST_BIT_FIELD_BITS {
			assert_eq!(BS.get_bit(bit_position_in_field), false);
		}
	}
	// endregion

	// region: bit_set_get_bit_01
	#[test]
	fn bit_set_get_bit_01() {
		const BS: BitSet = BitSet { inner: [!0; BitSet::LEN] };
		const LEN: Integer = match BitSet::BITS % BitField::BITS as Integer {
			0 => BitSet::LEN as Integer,
			_ => BitSet::LEN as Integer - 1,
		};
		const LAST_BIT_FIELD_BITS: Integer = BitSet::BITS as Integer - LEN;

		for i in 0..LEN {
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 7), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 6), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 5), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 4), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 3), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 2), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 1), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 0), true);
		}
		for bit_position_in_field in 0..LAST_BIT_FIELD_BITS {
			assert_eq!(BS.get_bit(bit_position_in_field), true);
		}
	}
	// endregion

	// region: bit_set_get_bit_02
	#[test]
	fn bit_set_get_bit_02() {
		const BS: BitSet = BitSet { inner: [0b_10101100; BitSet::LEN] };
		const LEN: Integer = match BitSet::BITS % BitField::BITS as Integer {
			0 => BitSet::LEN as Integer,
			_ => BitSet::LEN as Integer - 1,
		};
		const LAST_BIT_FIELD_BITS: Integer = BitSet::BITS as Integer - LEN;

		for i in 0..LEN {
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 7), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 6), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 5), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 4), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 3), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 2), true);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 1), false);
			assert_eq!(BS.get_bit(i * BitField::BITS as Integer + 0), false);
		}
		if LAST_BIT_FIELD_BITS > 7 {
			assert_eq!(BS.get_bit(7), true);
		}
		if LAST_BIT_FIELD_BITS > 6 {
			assert_eq!(BS.get_bit(6), false);
		}
		if LAST_BIT_FIELD_BITS > 5 {
			assert_eq!(BS.get_bit(5), true);
		}
		if LAST_BIT_FIELD_BITS > 4 {
			assert_eq!(BS.get_bit(4), false);
		}
		if LAST_BIT_FIELD_BITS > 3 {
			assert_eq!(BS.get_bit(3), true);
		}
		if LAST_BIT_FIELD_BITS > 2 {
			assert_eq!(BS.get_bit(2), true);
		}
		if LAST_BIT_FIELD_BITS > 1 {
			assert_eq!(BS.get_bit(1), false);
		}
		if LAST_BIT_FIELD_BITS > 0 {
			assert_eq!(BS.get_bit(0), false);
		}
	}
	// endregion

	// region: bit_set_clear_bit_00
	#[test]
	fn bit_set_clear_bit_00() {
		test_bit_set_clear_bit(&mut BitSet { inner: [0; BitSet::LEN] });
	}
	// endregion

	// region: bit_set_clear_bit_01
	#[test]
	fn bit_set_clear_bit_01() {
		test_bit_set_clear_bit(&mut BitSet { inner: [!0; BitSet::LEN] });
	}
	// endregion

	// region: bit_set_clear_bit_02
	#[test]
	fn bit_set_clear_bit_02() {
		test_bit_set_clear_bit(&mut BitSet { inner: [0b_11100101; BitSet::LEN] });
	}
	// endregion

	// region: bit_set_find_first_set_bit_00
	#[test]
	fn bit_set_find_first_set_bit_00() {
		const BS: BitSet = BitSet { inner: [0; BitSet::LEN] };

		for n in 0..=BitSet::BITS {
			assert_eq!(BS.find_first_set_bit(n), None);
		}
	}
	// endregion

	// region: bit_set_find_first_set_bit_01
	#[test]
	fn bit_set_find_first_set_bit_01() {
		const BS: BitSet = BitSet { inner: [!0; BitSet::LEN] };

		assert_eq!(BS.find_first_set_bit(0), None);
		for n in 1..=BitSet::BITS {
			assert_eq!(BS.find_first_set_bit(n), Some(0));
		}
	}
	// endregion

	// region: bit_set_find_first_set_bit_02
	#[test]
	fn bit_set_find_first_set_bit_02() {
		const BS: BitSet = BitSet { inner: [0b_00001000; BitSet::LEN] };

		for n in 0..4.min(BitSet::BITS) {
			assert_eq!(BS.find_first_set_bit(n), None);
		}
		for n in 4..=BitSet::BITS {
			assert_eq!(BS.find_first_set_bit(n), Some(3));
		}
	}
	// endregion

	// region: sieve_new_00
	#[test]
	fn sieve_new_00() {
		const REMAINING_NUMBERS: Integer = Integer::MAX - 2 + 1;
		let sieve: Sieve = Sieve::new();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		check_sieve_inner_bit_set(&sieve.inner, sieve.len, sieve.first);
		assert_eq!(sieve.first, 2);
		assert_eq!(sieve.remaining_numbers, REMAINING_NUMBERS);
		assert_eq!(sieve.len, REMAINING_NUMBERS.min(BitSet::BITS));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_00
	#[test]
	fn sieve_fill_with_next_chunk_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_01
	#[test]
	fn sieve_fill_with_next_chunk_01() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_02
	#[test]
	fn sieve_fill_with_next_chunk_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, BitSet::BITS);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_03
	#[test]
	fn sieve_fill_with_next_chunk_03() {
		const FIRST: Integer = 42;
		const REMAINING_NUMBERS: Integer = Integer::MAX - FIRST + 1;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: REMAINING_NUMBERS.min(BitSet::BITS),
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		match FIRST.checked_add(BitSet::BITS) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitSet::BITS) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, sieve.remaining_numbers.min(BitSet::BITS));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_04
	#[test]
	fn sieve_fill_with_next_chunk_04() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_05
	#[test]
	fn sieve_fill_with_next_chunk_05() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_06
	#[test]
	fn sieve_fill_with_next_chunk_06() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, BitSet::BITS);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_07
	#[test]
	fn sieve_fill_with_next_chunk_07() {
		const FIRST: Integer = 42;
		const REMAINING_NUMBERS: Integer = Integer::MAX - FIRST + 1;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: REMAINING_NUMBERS.min(BitSet::BITS),
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		match FIRST.checked_add(BitSet::BITS) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitSet::BITS) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, sieve.remaining_numbers.min(BitSet::BITS));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_08
	#[test]
	fn sieve_fill_with_next_chunk_08() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0b_00100111; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_09
	#[test]
	fn sieve_fill_with_next_chunk_09() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0b_00100111; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_10
	#[test]
	fn sieve_fill_with_next_chunk_10() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0b_00100111; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, BitSet::BITS);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_11
	#[test]
	fn sieve_fill_with_next_chunk_11() {
		const FIRST: Integer = 42;
		const REMAINING_NUMBERS: Integer = Integer::MAX - FIRST + 1;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0b_00100111; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: REMAINING_NUMBERS.min(BitSet::BITS),
		};

		sieve.fill_with_next_chunk();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		match FIRST.checked_add(BitSet::BITS) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitSet::BITS) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, sieve.remaining_numbers.min(BitSet::BITS));
	}
	// endregion

	// region: sieve_remove_non_primes_00
	#[test]
	fn sieve_remove_non_primes_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_01
	#[test]
	fn sieve_remove_non_primes_01() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_remove_non_primes_02
	#[test]
	fn sieve_remove_non_primes_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_03
	#[test]
	fn sieve_remove_non_primes_03() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_remove_non_primes_04
	#[test]
	fn sieve_remove_non_primes_04() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_05
	#[test]
	fn sieve_remove_non_primes_05() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_remove_non_primes_06
	#[test]
	fn sieve_remove_non_primes_06() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_07
	#[test]
	fn sieve_remove_non_primes_07() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_remove_non_primes_08
	#[test]
	fn sieve_remove_non_primes_08() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_09
	#[test]
	fn sieve_remove_non_primes_09() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_10
	#[test]
	fn sieve_remove_non_primes_10() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_11
	#[test]
	fn sieve_remove_non_primes_11() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = PRIMES.into_iter().filter(|prime| *prime < FIRST).collect();
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, primes);
		check_sieve_inner_bit_set(&sieve.inner, sieve.len, sieve.first);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_remove_non_primes_12
	#[test]
	fn sieve_remove_non_primes_12() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.inner.inner, [!0; BitSet::LEN]);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_13
	#[test]
	fn sieve_remove_non_primes_13() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = PRIMES.into_iter().filter(|prime| *prime < FIRST).collect();
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			inner: BitSet { inner: [!0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, primes);
		check_sieve_inner_bit_set(&sieve.inner, sieve.len, sieve.first);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitSet::BITS);
		assert_eq!(sieve.len, BitSet::BITS);
	}
	// endregion

	// region: sieve_find_next_prime_00
	#[test]
	fn sieve_find_next_prime_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_01
	#[test]
	fn sieve_find_next_prime_01() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: 0,
			len: BitSet::BITS,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_02
	#[test]
	fn sieve_find_next_prime_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: 0,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_03
	#[test]
	fn sieve_find_next_prime_03() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_04
	#[test]
	fn sieve_find_next_prime_04() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = PRIMES.into_iter().filter(|prime| *prime < FIRST).collect();
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: 0,
			len: BitSet::BITS,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_05
	#[test]
	fn sieve_find_next_prime_05() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = PRIMES.into_iter().filter(|prime| *prime < FIRST).collect();
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), Some(43));
	}
	// endregion

	// region: sieve_find_next_prime_06
	#[test]
	fn sieve_find_next_prime_06() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = PRIMES.into_iter().filter(|prime| *prime < FIRST).collect();
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			inner: BitSet { inner: [0; BitSet::LEN] },
			first: FIRST,
			remaining_numbers: BitSet::BITS,
			len: BitSet::BITS,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion
}
