pub type Integer = u16;

const STARTING_PRIMES: [Integer; 0] = [];
// const STARTING_PRIMES: [Integer; 1] = [2];
// const STARTING_PRIMES: [Integer; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[inline(always)]
const fn min(a: Integer, b: Integer) -> Integer {
	if a < b {
		a
	} else {
		b
	}
}

type BitField = usize;

/// An implementation of the Sieve of Eratosthenes.
/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
/// This implementation uses multiple limited ranges of numbers instead of a single huge range,
/// allowing to find prime numbers to whatever limit we want
/// without having to allocate a huge memory area.
pub struct Sieve {
	/// A vector that contains the prime numbers that have already been found,
	/// sorted in ascending order.
	primes_found_so_far: Vec<Integer>,

	/// A bit field that represents the numbers in the current range.<br>
	/// For each bit:
	/// * 0 means that the represented number is not prime.
	/// * 1 means that the represented number is prime.
	range: BitField,

	/// The number represented by the first bit of `self.range`.
	first: Integer,

	/// The number of remaining numbers that have not yet been computed by the sieve.
	remaining_numbers: Integer,

	/// The number of numbers that are considered by the sieve for the current range.
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
		const FIRST: Integer = match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => *last + 1,
			None => 2,
			_ => 0,
		};
		const REMAINING_NUMBERS: Integer = match FIRST {
			0 => 0,
			_ => Integer::MAX - FIRST + 1,
		};
		const LEN: Integer = min(BitField::BITS as Integer, REMAINING_NUMBERS);

		let mut sieve: Self = Self {
			primes_found_so_far: STARTING_PRIMES.to_vec(),
			range: !0,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: LEN,
		};

		sieve.remove_non_primes();

		sieve
	}

	/// Update inner attributes to consider the next range of numbers.
	/// All the numbers of the next range are considered prime by default.
	/// The non-prime numbers will be removed later.
	fn fill_with_next_range(self: &mut Self) {
		self.range = !0;
		if let Some(sum) = self.first.checked_add(self.len) {
			self.first = sum;
		}
		self.remaining_numbers -= self.len;
		self.len = self.remaining_numbers.min(BitField::BITS as Integer);
	}

	/// Remove the non-prime numbers from the current range of numbers.
	/// The non-prime numbers are found by multiplying the prime numbers
	/// that we have found so far, and then multiplying the remaining
	/// numbers in the range from the itself (Yes, it sounds like an Inception).
	fn remove_non_primes(self: &mut Self) {
		#[inline(always)]
		fn remove_prime_multiples(
			multiple: Integer,
			first: Integer,
			range: &mut BitField,
			prime: Integer,
			len: Integer,
		) {
			let mut bit_position: Integer = multiple - first;

			while bit_position < len {
				*range &= !(1 << bit_position);
				match bit_position.checked_add(prime) {
					Some(sum) => bit_position = sum,
					None => break,
				}
			}
		}

		if self.len == 0 {
			return;
		}

		// TODO: Replace `sqrt()` by `isqrt()` when it will be stable.
		let sqrt: Integer = ((self.first + (self.len - 1)) as f32).sqrt() as Integer;

		for prime in &self.primes_found_so_far {
			if *prime > sqrt {
				break;
			}

			let multiple: Integer = match self.first.checked_next_multiple_of(*prime) {
				Some(multiple) => multiple,
				None => continue,
			};

			remove_prime_multiples(multiple, self.first, &mut self.range, *prime, self.len);
		}

		for bit_position in 0..self.len {
			if self.range >> bit_position & 1 == 1 {
				let prime: Integer = self.first + bit_position;

				if prime > sqrt {
					break;
				}

				let multiple: Integer = prime * prime;

				remove_prime_multiples(multiple, self.first, &mut self.range, prime, self.len);
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
			let n: Integer = self.range.trailing_zeros() as Integer;

			if n < self.len {
				let prime: Integer = self.first + n as Integer;

				self.range &= !(1 << n);
				self.primes_found_so_far.push(prime);

				return Some(prime);
			} else if self.remaining_numbers != 0 {
				self.fill_with_next_range();
				self.remove_non_primes();
			} else {
				return None;
			}
		}
	}
}
// endregion

/// Searches in `v` for the first element that is __greater or equal__ to `n`.<br>
/// It is assumed that `v` is sorted in ascending order.
///
/// # Parameters
/// * `v` - The vector to search in.
/// * `n` - The number to search the lower bound for.
///
/// # Return
/// * `Some(lb)` - The first element that is __greater or equal__ to `n` in `v`.
/// * `None` - There is no element that is __greater or equal__ to `n` in `v`.
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
	/// * `Some(prime)` - The next prime number.
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

type Exponent = u8;
type PrimeFactor = (Integer, Exponent);

/// Decompose `n` into its prime factors, with for each, its exponent.
/// The prime factors are sorted in ascending order.
///
/// ### Parameters
/// * `n` - The number to decompose.
///
/// ### Return
/// A vector that contains the prime factors of `n`, with for each, its exponent.
///
/// ### Example
/// ```
/// use ex04::prime_decomposition;
///
/// assert_eq!(prime_decomposition(0), vec![]);
/// assert_eq!(prime_decomposition(2), vec![(2, 1)]);
/// assert_eq!(prime_decomposition(5), vec![(5, 1)]);
/// assert_eq!(prime_decomposition(42), vec![(2, 1), (3, 1), (7, 1)]);
/// assert_eq!(prime_decomposition(72), vec![(2, 3), (3, 2)]);
/// ```
pub fn prime_decomposition(mut n: Integer) -> Vec<PrimeFactor> {
	let mut prime_factors: Vec<PrimeFactor> = Vec::new();

	for prime in Prime::new(2) {
		if prime > n {
			break;
		}

		let mut exponent: Exponent = 0;

		while n % prime == 0 {
			n /= prime;
			exponent += 1;
		}

		if exponent > 0 {
			prime_factors.push((prime, exponent));
		}
	}

	prime_factors
}

#[cfg(test)]
mod tests {
	use primes::PrimeSet;

	use super::*;

	const PRIMES: [Integer; 54] = [
		// region: PRIMES
		2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
		97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
		191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
		// endregion
	];

	#[inline(always)]
	fn check_sieve_range(range: &BitField, len: Integer, first: Integer) {
		const PRIMES_LAST: Integer = PRIMES[PRIMES.len() - 1];

		for bit_position in 0..min(len, PRIMES_LAST - first + 1) {
			match PRIMES.binary_search(&(first + bit_position)) {
				Ok(__) => assert_eq!(range >> bit_position & 1, 1),
				Err(_) => assert_eq!(range >> bit_position & 1, 0),
			}
		}
	}

	// region: sieve_new_00
	#[test]
	fn sieve_new_00() {
		let sieve: Sieve = Sieve::new();

		assert_eq!(sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&sieve.range, sieve.len, sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(sieve.first, *last + 1);
				assert_eq!(sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(sieve.first, 2);
				assert_eq!(sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(sieve.first, 0);
				assert_eq!(sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(sieve.len, min(sieve.remaining_numbers, BitField::BITS as Integer));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_00
	#[test]
	fn sieve_fill_with_next_chunk_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
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
			range: 0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_02
	#[test]
	fn sieve_fill_with_next_chunk_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, BitField::BITS as Integer);
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
			range: 0,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: min(REMAINING_NUMBERS, BitField::BITS as Integer),
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		match FIRST.checked_add(BitField::BITS as Integer) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitField::BITS as Integer) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, min(sieve.remaining_numbers, BitField::BITS as Integer));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_04
	#[test]
	fn sieve_fill_with_next_chunk_04() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_05
	#[test]
	fn sieve_fill_with_next_chunk_05() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_06
	#[test]
	fn sieve_fill_with_next_chunk_06() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, BitField::BITS as Integer);
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
			range: !0,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: min(REMAINING_NUMBERS, BitField::BITS as Integer),
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		match FIRST.checked_add(BitField::BITS as Integer) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitField::BITS as Integer) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, min(sieve.remaining_numbers, BitField::BITS as Integer));
	}
	// endregion

	// region: sieve_fill_with_next_chunk_08
	#[test]
	fn sieve_fill_with_next_chunk_08() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00100111,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_09
	#[test]
	fn sieve_fill_with_next_chunk_09() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00100111,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_fill_with_next_chunk_10
	#[test]
	fn sieve_fill_with_next_chunk_10() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00100111,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, BitField::BITS as Integer);
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
			range: 0b_00100111,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: min(REMAINING_NUMBERS, BitField::BITS as Integer),
		};

		sieve.fill_with_next_range();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		match FIRST.checked_add(BitField::BITS as Integer) {
			Some(sum) => assert_eq!(sieve.first, sum),
			None => assert_eq!(sieve.first, FIRST),
		}
		match REMAINING_NUMBERS.checked_sub(BitField::BITS as Integer) {
			Some(diff) => assert_eq!(sieve.remaining_numbers, diff),
			None => assert_eq!(sieve.remaining_numbers, 0),
		}
		assert_eq!(sieve.len, min(sieve.remaining_numbers, BitField::BITS as Integer));
	}
	// endregion

	// region: sieve_remove_non_primes_00
	#[test]
	fn sieve_remove_non_primes_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
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
			range: 0,
			first: 0,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_remove_non_primes_02
	#[test]
	fn sieve_remove_non_primes_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_03
	#[test]
	fn sieve_remove_non_primes_03() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_remove_non_primes_04
	#[test]
	fn sieve_remove_non_primes_04() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
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
			range: 0,
			first: FIRST,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_remove_non_primes_06
	#[test]
	fn sieve_remove_non_primes_06() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_07
	#[test]
	fn sieve_remove_non_primes_07() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, 0);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, BitField::BITS as Integer);
	}
	// endregion

	// region: sieve_remove_non_primes_08
	#[test]
	fn sieve_remove_non_primes_08() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
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
			range: !0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, 0);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_10
	#[test]
	fn sieve_remove_non_primes_10() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_11
	#[test]
	fn sieve_remove_non_primes_11() {
		const FIRST: Integer = 42;
		const LEN: Integer = min(Integer::MAX - FIRST + 1, BitField::BITS as Integer);
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: !0,
			first: FIRST,
			remaining_numbers: 0,
			len: LEN,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, primes);
		check_sieve_range(&sieve.range, sieve.len, sieve.first);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, 0);
		assert_eq!(sieve.len, LEN);
	}
	// endregion

	// region: sieve_remove_non_primes_12
	#[test]
	fn sieve_remove_non_primes_12() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: 0,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, Vec::new());
		assert_eq!(sieve.range, !0);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, 0);
	}
	// endregion

	// region: sieve_remove_non_primes_13
	#[test]
	fn sieve_remove_non_primes_13() {
		const FIRST: Integer = 42;
		const LEN: Integer = min(Integer::MAX - FIRST + 1, BitField::BITS as Integer);
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: !0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: LEN,
		};

		sieve.remove_non_primes();

		assert_eq!(sieve.primes_found_so_far, primes);
		check_sieve_range(&sieve.range, sieve.len, sieve.first);
		assert_eq!(sieve.first, FIRST);
		assert_eq!(sieve.remaining_numbers, BitField::BITS as Integer);
		assert_eq!(sieve.len, LEN);
	}
	// endregion

	// region: sieve_find_next_prime_00
	#[test]
	fn sieve_find_next_prime_00() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
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
			range: 0,
			first: 0,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_02
	#[test]
	fn sieve_find_next_prime_02() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
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
			range: 0,
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
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: 0,
			first: FIRST,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_05
	#[test]
	fn sieve_find_next_prime_05() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: 0,
			first: FIRST,
			remaining_numbers: min(Integer::MAX - FIRST + 1, BitField::BITS as Integer),
			len: 0,
		};

		match lower_bound(&PRIMES.to_vec(), FIRST) {
			Some(lb) if lb - FIRST < sieve.remaining_numbers => {
				assert_eq!(sieve.find_next_prime(), Some(lb));
			}
			________________________________________________ => {
				assert_eq!(sieve.find_next_prime(), None);
			}
		}
	}
	// endregion

	// region: sieve_find_next_prime_06
	#[test]
	fn sieve_find_next_prime_06() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: 0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_07
	#[test]
	fn sieve_find_next_prime_07() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_08
	#[test]
	fn sieve_find_next_prime_08() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), Some(0));
	}
	// endregion

	// region: sieve_find_next_prime_09
	#[test]
	fn sieve_find_next_prime_09() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), Some(0));
	}
	// endregion

	// region: sieve_find_next_prime_10
	#[test]
	fn sieve_find_next_prime_10() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_11
	#[test]
	fn sieve_find_next_prime_11() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), Some(FIRST));
	}
	// endregion

	// region: sieve_find_next_prime_12
	#[test]
	fn sieve_find_next_prime_12() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: min(Integer::MAX - FIRST + 1, BitField::BITS as Integer),
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), Some(FIRST));
	}
	// endregion

	// region: sieve_find_next_prime_13
	#[test]
	fn sieve_find_next_prime_13() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: !0,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		assert_eq!(sieve.find_next_prime(), Some(FIRST));
	}
	// endregion

	// region: sieve_find_next_prime_14
	#[test]
	fn sieve_find_next_prime_14() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00101000,
			first: 0,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_15
	#[test]
	fn sieve_find_next_prime_15() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00101000,
			first: 0,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		if BitField::BITS < 4 {
			assert_eq!(sieve.find_next_prime(), None);
		} else {
			assert_eq!(sieve.find_next_prime(), Some(3));
		}
	}
	// endregion

	// region: sieve_find_next_prime_16
	#[test]
	fn sieve_find_next_prime_16() {
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00101000,
			first: 0,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		if BitField::BITS < 4 {
			assert_eq!(sieve.find_next_prime(), None);
		} else {
			assert_eq!(sieve.find_next_prime(), Some(3));
		}
	}
	// endregion

	// region: sieve_find_next_prime_17
	#[test]
	fn sieve_find_next_prime_17() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00101000,
			first: FIRST,
			remaining_numbers: 0,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: sieve_find_next_prime_18
	#[test]
	fn sieve_find_next_prime_18() {
		const FIRST: Integer = 42;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: Vec::new(),
			range: 0b_00101000,
			first: FIRST,
			remaining_numbers: 0,
			len: BitField::BITS as Integer,
		};

		if BitField::BITS < 4 {
			assert_eq!(sieve.find_next_prime(), None);
		} else {
			assert_eq!(sieve.find_next_prime(), Some(FIRST + 3));
		}
	}
	// endregion

	// region: sieve_find_next_prime_19
	#[test]
	fn sieve_find_next_prime_19() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: 0b_00101000,
			first: FIRST,
			remaining_numbers: min(Integer::MAX - FIRST + 1, BitField::BITS as Integer),
			len: 0,
		};

		match lower_bound(&PRIMES.to_vec(), FIRST) {
			Some(lb) if lb - FIRST < sieve.remaining_numbers => {
				assert_eq!(sieve.find_next_prime(), Some(lb))
			}
			________________________________________________ => {
				assert_eq!(sieve.find_next_prime(), None)
			}
		}
	}
	// endregion

	// region: sieve_find_next_prime_20
	#[test]
	fn sieve_find_next_prime_20() {
		const FIRST: Integer = 42;
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in PRIMES {
				if prime >= FIRST {
					break;
				}
				v.push(prime);
			}

			v
			// endregion
		};
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: primes.clone(),
			range: 0b_00101000,
			first: FIRST,
			remaining_numbers: BitField::BITS as Integer,
			len: BitField::BITS as Integer,
		};

		if BitField::BITS < 4 {
			assert_eq!(sieve.find_next_prime(), None);
		} else {
			assert_eq!(sieve.find_next_prime(), Some(FIRST + 3));
		}
	}
	// endregion

	// region: sieve_find_next_prime_21
	#[test]
	fn sieve_find_next_prime_21() {
		let primes: Vec<Integer> = {
			// region: primes
			let mut v: Vec<Integer> = Vec::new();

			for prime in primes::Sieve::new().iter() {
				if prime > Integer::MAX as u64 {
					break;
				}
				v.push(prime as Integer);
			}

			v
			// endregion
		};
		let split: (&[Integer], &[Integer]) = primes.split_at(primes.len() - 3);
		let first: Integer = split.1[0];
		let remaining_numbers: Integer = Integer::MAX - first + 1;
		let mut sieve: Sieve = Sieve {
			primes_found_so_far: split.0.to_vec(),
			range: 0,
			first,
			remaining_numbers,
			len: 0,
		};

		assert_eq!(sieve.find_next_prime(), Some(split.1[0]));
		assert_eq!(sieve.find_next_prime(), Some(split.1[1]));
		assert_eq!(sieve.find_next_prime(), Some(split.1[2]));
		assert_eq!(sieve.find_next_prime(), None);
		assert_eq!(sieve.find_next_prime(), None);
	}
	// endregion

	// region: lower_bound_00
	#[test]
	fn lower_bound_00() {
		assert_eq!(lower_bound(&vec![], 0), None);
	}
	// endregion

	// region: lower_bound_01
	#[test]
	fn lower_bound_01() {
		assert_eq!(lower_bound(&vec![0], 0), Some(0));
	}
	// endregion

	// region: lower_bound_02
	#[test]
	fn lower_bound_02() {
		assert_eq!(lower_bound(&vec![0], 1), None);
	}
	// endregion

	// region: lower_bound_03
	#[test]
	fn lower_bound_03() {
		assert_eq!(lower_bound(&vec![1], 0), Some(1));
	}
	// endregion

	// region: lower_bound_04
	#[test]
	fn lower_bound_04() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 0), Some(1));
	}
	// endregion

	// region: lower_bound_05
	#[test]
	fn lower_bound_05() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 1), Some(1));
	}
	// endregion

	// region: lower_bound_06
	#[test]
	fn lower_bound_06() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 3), Some(4));
	}
	// endregion

	// region: lower_bound_07
	#[test]
	fn lower_bound_07() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 5), Some(8));
	}
	// endregion

	// region: lower_bound_08
	#[test]
	fn lower_bound_08() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 8), Some(8));
	}
	// endregion

	// region: lower_bound_09
	#[test]
	fn lower_bound_09() {
		assert_eq!(lower_bound(&vec![1, 2, 4, 8], 9), None);
	}
	// endregion

	// region: lower_bound_10
	#[test]
	fn lower_bound_10() {
		assert_eq!(
			lower_bound(&vec![Integer::MAX - 42, Integer::MAX], Integer::MAX - 21),
			Some(Integer::MAX)
		);
	}
	// endregion

	// region: lower_bound_11
	#[test]
	fn lower_bound_11() {
		assert_eq!(lower_bound(&vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5], 2), Some(2));
	}
	// endregion

	// region: lower_bound_12
	#[test]
	fn lower_bound_12() {
		assert_eq!(lower_bound(&vec![1, 3, 3, 3, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7], 2), Some(3));
	}
	// endregion

	// region: prime_new_00
	#[test]
	fn prime_new_00() {
		let prime: Prime = Prime::new(0);

		assert_eq!(prime.n, 0);
		assert_eq!(prime.sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&prime.sieve.range, prime.sieve.len, prime.sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(prime.sieve.first, *last + 1);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(prime.sieve.first, 2);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(prime.sieve.first, 0);
				assert_eq!(prime.sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(prime.sieve.len, min(prime.sieve.remaining_numbers, BitField::BITS as Integer));
		assert_eq!(prime.is_end_reached, false);
	}
	// endregion

	// region: prime_new_01
	#[test]
	fn prime_new_01() {
		let prime: Prime = Prime::new(1);

		assert_eq!(prime.n, 1);
		assert_eq!(prime.sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&prime.sieve.range, prime.sieve.len, prime.sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(prime.sieve.first, *last + 1);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(prime.sieve.first, 2);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(prime.sieve.first, 0);
				assert_eq!(prime.sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(prime.sieve.len, min(prime.sieve.remaining_numbers, BitField::BITS as Integer));
		assert_eq!(prime.is_end_reached, false);
	}
	// endregion

	// region: prime_new_02
	#[test]
	fn prime_new_02() {
		let prime: Prime = Prime::new(2);

		assert_eq!(prime.n, 2);
		assert_eq!(prime.sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&prime.sieve.range, prime.sieve.len, prime.sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(prime.sieve.first, *last + 1);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(prime.sieve.first, 2);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(prime.sieve.first, 0);
				assert_eq!(prime.sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(prime.sieve.len, min(prime.sieve.remaining_numbers, BitField::BITS as Integer));
		assert_eq!(prime.is_end_reached, false);
	}
	// endregion

	// region: prime_new_03
	#[test]
	fn prime_new_03() {
		let prime: Prime = Prime::new(42);

		assert_eq!(prime.n, 42);
		assert_eq!(prime.sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&prime.sieve.range, prime.sieve.len, prime.sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(prime.sieve.first, *last + 1);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(prime.sieve.first, 2);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(prime.sieve.first, 0);
				assert_eq!(prime.sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(prime.sieve.len, min(prime.sieve.remaining_numbers, BitField::BITS as Integer));
		assert_eq!(prime.is_end_reached, false);
	}
	// endregion

	// region: prime_new_04
	#[test]
	fn prime_new_04() {
		let prime: Prime = Prime::new(Integer::MAX);

		assert_eq!(prime.n, Integer::MAX);
		assert_eq!(prime.sieve.primes_found_so_far, STARTING_PRIMES.to_vec());
		check_sieve_range(&prime.sieve.range, prime.sieve.len, prime.sieve.first);
		match STARTING_PRIMES.last() {
			Some(last) if *last < Integer::MAX => {
				assert_eq!(prime.sieve.first, *last + 1);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - *last);
			}
			None => {
				assert_eq!(prime.sieve.first, 2);
				assert_eq!(prime.sieve.remaining_numbers, Integer::MAX - 1);
			}
			_ => {
				assert_eq!(prime.sieve.first, 0);
				assert_eq!(prime.sieve.remaining_numbers, 0);
			}
		}
		assert_eq!(prime.sieve.len, min(prime.sieve.remaining_numbers, BitField::BITS as Integer));
		assert_eq!(prime.is_end_reached, false);
	}
	// endregion

	// region: prime_next_00
	#[test]
	fn prime_next_00() {
		let mut prime: Prime = Prime::new(0);

		assert_eq!(prime.next(), Some(2));
		assert_eq!(prime.next(), Some(3));
		assert_eq!(prime.next(), Some(5));
		assert_eq!(prime.next(), Some(7));
		assert_eq!(prime.next(), Some(11));
	}
	// endregion

	// region: prime_next_01
	#[test]
	fn prime_next_01() {
		let mut prime: Prime = Prime::new(1);

		assert_eq!(prime.next(), Some(2));
		assert_eq!(prime.next(), Some(3));
		assert_eq!(prime.next(), Some(5));
		assert_eq!(prime.next(), Some(7));
		assert_eq!(prime.next(), Some(11));
	}
	// endregion

	// region: prime_next_02
	#[test]
	fn prime_next_02() {
		let mut prime: Prime = Prime::new(2);

		assert_eq!(prime.next(), Some(2));
		assert_eq!(prime.next(), Some(3));
		assert_eq!(prime.next(), Some(5));
		assert_eq!(prime.next(), Some(7));
		assert_eq!(prime.next(), Some(11));
	}
	// endregion

	// region: prime_next_03
	#[test]
	fn prime_next_03() {
		let mut prime: Prime = Prime::new(8);

		assert_eq!(prime.next(), Some(11));
		assert_eq!(prime.next(), Some(13));
		assert_eq!(prime.next(), Some(17));
		assert_eq!(prime.next(), Some(19));
		assert_eq!(prime.next(), Some(23));
	}
	// endregion

	// region: prime_next_04
	#[test]
	fn prime_next_04() {
		let mut prime: Prime = Prime::new(42);

		assert_eq!(prime.next(), Some(43));
		assert_eq!(prime.next(), Some(47));
		assert_eq!(prime.next(), Some(53));
		assert_eq!(prime.next(), Some(59));
		assert_eq!(prime.next(), Some(61));
	}
	// endregion

	// region: prime_next_05
	#[test]
	fn prime_next_05() {
		const FIRST: Integer = Integer::MAX - 10;
		let mut prime: Prime = Prime::new(FIRST);

		for n in FIRST..=Integer::MAX {
			if primes::is_prime(n as u64) {
				assert_eq!(prime.next(), Some(n));
			}
		}
		for _ in 0..3 {
			assert_eq!(prime.next(), None);
		}
	}
	// endregion

	// region: prime_decomposition_00
	#[test]
	fn prime_decomposition_00() {
		assert_eq!(prime_decomposition(0), vec![]);
	}
	// endregion

	// region: prime_decomposition_01
	#[test]
	fn prime_decomposition_01() {
		assert_eq!(prime_decomposition(1), vec![]);
	}
	// endregion

	// region: prime_decomposition_02
	#[test]
	fn prime_decomposition_02() {
		assert_eq!(prime_decomposition(2), vec![(2, 1)]);
	}
	// endregion

	// region: prime_decomposition_03
	#[test]
	fn prime_decomposition_03() {
		assert_eq!(prime_decomposition(3), vec![(3, 1)]);
	}
	// endregion

	// region: prime_decomposition_04
	#[test]
	fn prime_decomposition_04() {
		assert_eq!(prime_decomposition(4), vec![(2, 2)]);
	}
	// endregion

	// region: prime_decomposition_05
	#[test]
	fn prime_decomposition_05() {
		assert_eq!(prime_decomposition(250), vec![(2, 1), (5, 3)]);
	}
	// endregion

	// region: prime_decomposition_06
	#[test]
	fn prime_decomposition_06() {
		assert_eq!(prime_decomposition(251), vec![(251, 1)]);
	}
	// endregion

	// region: prime_decomposition_07
	#[test]
	fn prime_decomposition_07() {
		assert_eq!(prime_decomposition(252), vec![(2, 2), (3, 2), (7, 1)]);
	}
	// endregion

	// region: prime_decomposition_08
	#[test]
	fn prime_decomposition_08() {
		assert_eq!(prime_decomposition(253), vec![(11, 1), (23, 1)]);
	}
	// endregion

	// region: prime_decomposition_09
	#[test]
	fn prime_decomposition_09() {
		assert_eq!(prime_decomposition(254), vec![(2, 1), (127, 1)]);
	}
	// endregion

	// region: prime_decomposition_10
	#[test]
	fn prime_decomposition_10() {
		assert_eq!(prime_decomposition(255), vec![(3, 1), (5, 1), (17, 1)]);
	}
	// endregion

	// region: prime_decomposition_11
	#[test]
	fn prime_decomposition_11() {
		assert_eq!(prime_decomposition(128), vec![(2, 7)]);
	}
	// endregion
}
