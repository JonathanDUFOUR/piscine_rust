type Integer = u32;
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
		const LEN: Integer = BitSet::BITS / BitField::BITS;

		match LEN * BitField::BITS {
			BitSet::BITS => LEN as usize,
			____________ => LEN as usize + 1,
		}
	};

	/// Creates a new BitSet instance and initializes it as entierly filled with ones.
	///
	/// ### Return
	/// The newly created BitSet instance.
	///
	/// ### Example
	/// ```
	/// use ex04::BitSet;
	///
	/// let bit_set: BitSet = BitSet::new();
	/// ```
	#[inline(always)]
	const fn new() -> Self {
		Self { inner: [!0; Self::LEN] }
	}

	/// ### Return
	/// The number of bit fields in the BitSet.
	#[inline(always)]
	const fn len(self: &Self) -> usize {
		self.inner.len()
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
		if n < 0 || n >= Self::BITS {
			panic!("n({n}) is out of bounds.");
		}

		let j: usize = (n / BitField::BITS) as usize;
		let mask: BitField = 1 << n % BitField::BITS;

		self.inner[j] & mask != 0
	}

	/// Sets the `n`th bit in the bit set to 1.
	///
	/// ### Parameters
	/// * `n` - The position of the bit.
	///
	/// ### Panic
	/// `n` is out of bounds.
	#[inline(always)]
	fn set_bit(self: &mut Self, n: Integer) {
		if n >= Self::BITS {
			panic!("n({n}) is out of bounds.");
		}

		let i: usize = (n / BitField::BITS) as usize;
		let mask: BitField = 1 << n % BitField::BITS;

		self.inner[i] |= mask;
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

		let i: usize = (n / BitField::BITS) as usize;
		let mask: BitField = !(1 << n % BitField::BITS);

		self.inner[i] &= mask;
	}

	/// Checks if the `n` first bits are all set to 0 in the bit set.
	///
	/// ### Parameteres
	/// * `n` - The number of bits to check.
	///
	/// ### Return
	/// * `true` - The `n` first bits are all set to 0.
	/// * `false` - There is at least 1 bit that is set to 1 in the `n` first bits.
	///
	/// ### Panic
	/// `n` is greater than the number of bits in `self`.
	fn are_first_bits_zeros(self: &Self, n: Integer) -> bool {
		if n > Self::BITS {
			panic!("n({n}) is greater than the number of bits in the BitSet({}).", Self::BITS);
		}

		let len: usize = (n / BitField::BITS) as usize;

		for i in 0..len {
			if self.inner[i] != 0 {
				return false;
			}
		}

		let checked_bits_so_far: Integer = len as Integer * BitField::BITS as Integer;

		if checked_bits_so_far < n {
			let mask: BitField = !0 >> (BitField::BITS - (n - checked_bits_so_far));

			if self.inner[len] & mask != 0 {
				return false;
			}
		}

		true
	}
}

/// A data structure to represent a range of numbers, and find the prime numbers in it.
/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
struct Sieve {
	/// The inner bitset that represents the numbers in the range.
	/// 0 means that the represented number is not prime.
	/// 1 means that the represented number is prime.
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
	/// The newly created Sieve instance is used to find the prime numbers
	/// in the range `[first, first + len[`.
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
	const fn new() -> Self {
		const INNER: BitSet = BitSet::new();
		const FIRST: Integer = 2;
		const REMAINING_NUMBERS: Integer = Integer::MAX - FIRST + 1;
		const LEN: Integer = BitSet::BITS.min(REMAINING_NUMBERS);

		Self {
			inner: INNER,
			first: FIRST,
			remaining_numbers: REMAINING_NUMBERS,
			len: LEN,
			primes_found_so_far: Vec::new(),
		}
	}

	/// ### Return
	/// The number of numbers that are considered by the sieve for the current chunk.
	#[inline(always)]
	const fn len(self: &Self) -> Integer {
		self.len
	}

	/// ### Return
	/// * `Some(n)` - The last element of `self.primes_found_so_far`,
	/// corresponding to the greatest prime number found so far.
	/// * `None` - There is no prime number in `self.primes_found_so_far`.
	const fn greatest_prime_found_so_far(self: &Self) -> Option<Integer> {
		self.primes_found_so_far.last()
	}

	/// Sets all the bits of `self.sieve` to 1,
	/// and update `self.sieve_first` to represent the first number of the next chunk of numbers.
	fn fill_with_next_chunk(self: &mut Self) {
		self.inner = BitSet::new();
		if let Some(sum) = self.first.checked_add(self.len) {
			self.first = sum;
		}
		self.remaining_numbers -= self.len;
		self.len = BitSet::BITS.min(self.remaining_numbers);
	}

	/// Sets the bits of the non-prime numbers in the sieve to 0.
	/// The non-prime numbers are found by multiplying the prime numbers
	/// that are already in `primes`, and then multiplying the remaining
	/// numbers in the sieve from the sieve. (Yes, it sounds like an Inception)
	///
	/// ### Parameters
	/// * `primes` - The prime numbers that have already been found.
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
}

/// An iterator that generates prime numbers.
pub struct Prime {
	/// The number to find the next prime from.
	n: Integer,

	/// The sieve of Eratosthenes that is used to find the next prime number.
	sieve: Sieve,
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
		Self { n, sieve: Sieve::new() }
	}
}

impl Iterator for Prime {
	type Item = u32;

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
		// TODO
		while self.n > self.sieve.greatest_prime_found_so_far()
		None
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
