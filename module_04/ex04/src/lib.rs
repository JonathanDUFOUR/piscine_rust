type Integer = u8;
type BitField = u16;

/// A fixed sized bitset.
struct BitSet {
	inner: [BitField; Self::LEN],
}

impl BitSet {
	/// The maximum number of usable bits in each BitSet instance.
	const BITS: Integer = 42;

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

		None
	}
}

/// An implementation of the Sieve of Eratosthenes.
/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
/// This implementation uses multiple limited chunks of numbers
/// instead of a single huge chunk of numbers from 2 to N,
/// allowing to find prime numbers to whatever limit we want, without having to allocate
/// a huge chunk of memory.
pub struct Sieve {
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

	/// A vector that contains the prime numbers that have already been found,
	/// sorted in ascending order.
	primes_found_so_far: Vec<Integer>,
}

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
			inner: INNER,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: LEN,
			// REMIND: `primes_found_so_far` must NOT be empty.
			primes_found_so_far: vec![2],
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
			mut multiple: Integer,
			first: Integer,
			inner: &mut BitSet,
			prime: Integer,
			len: Integer,
		) {
			let mut bit_position: Integer = multiple - first;

			while bit_position < len {
				inner.clear_bit(bit_position);
				multiple += prime;
				bit_position += prime;
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
			if !self.inner.get_bit(bit_position) {
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

#[cfg(test)]
mod tests {
	// use super::*;
}
