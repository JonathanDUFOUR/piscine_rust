type Inner = u32;
type BitField = u16;

/// The maximum number of usable bits in each BitSet instance.
const BITSET_BITS: Inner = 42;

const BITSET_LEN: usize = {
	const LEN: Inner = BITSET_BITS / BitField::BITS;

	match LEN * BitField::BITS {
		BITSET_BITS => LEN as usize,
		___________ => LEN as usize + 1,
	}
};

enum BitValue {
	ZERO,
	ONE,
}

impl PartialEq<u8> for BitValue {
	fn eq(self: &Self, rhs: &u8) -> bool {
		match (self, rhs) {
			(&Self::ZERO, 0) | (&Self::ONE, 1) => true,
			__________________________________ => false,
		}
	}
}

/// A fixed sized bitset.
struct BitSet([BitField; BITSET_LEN]);

impl BitSet {
	const BITS: Inner = BITSET_BITS;
	const LEN: usize = BITSET_LEN;

	/// Creates a new BitSet instance and initializes it as entierly filled with zeros.
	///
	/// # Return
	/// The newly created BitSet instance.
	///
	/// # Example
	/// ```
	/// use ex04::BitSet;
	///
	/// let bit_set: BitSet = BitSet::new();
	/// ```
	pub const fn new() -> Self {
		Self([0; Self::LEN])
	}

	/// Checks if the `n` first bits are all set to 0 in the binary representation of `self`.
	///
	/// # Parameteres
	/// * `n` - The number of bits to check.
	///
	/// # Return
	/// * `true` - The `n` first bits are all set to 0.
	/// * `false` - There is at least 1 bit that is set to 1 in the `n` first bits.
	///
	/// # Panic
	/// `n` is greater than the number of bits in `self`.
	fn are_first_bits_zeros(self: &Self, n: Inner) -> bool {
		if n > Self::BITS {
			panic!("n({}) is greater than the number of bits in the BitSet({}).", n, Self::BITS);
		}

		let len: usize = (n / BitField::BITS) as usize;

		for i in 0..len {
			if self.0[i] != 0 {
				return false;
			}
		}

		let checked_bits_so_far: Inner = len as Inner * BitField::BITS as Inner;

		if checked_bits_so_far < n {
			let mask: BitField = !0 >> (BitField::BITS - (n - checked_bits_so_far));

			if self.0[len] & mask != 0 {
				return false;
			}
		}

		true
	}
}

impl std::ops::Index<Inner> for BitSet {
	type Output = BitValue;

	/// # Parameters
	/// * `i` - The index of the wanted bit.
	///
	/// # Return
	/// A reference to the wanted bit in the calling BitSet.
	///
	/// # Panic
	/// The index is out of bound.
	fn index(self: &Self, i: Inner) -> &Self::Output {
		if i >= Self::BITS {
			panic!("index {} is out of bounds.", i);
		}

		let j: usize = (i / BitField::BITS) as usize;
		let mask: BitField = 1 << i % BitField::BITS;

		match self.0[j] & mask {
			0 => &BitValue::ZERO,
			_ => &BitValue::ONE,
		}
	}
}

/// An iterator that generates prime numbers.
pub struct Prime {
	/// The number to find the next prime from.
	n: Inner,

	/// A vector that contains the prime numbers that have already been found,
	/// sorted in ascending order.
	primes: Vec<Inner>,

	/// A bit set where each bit represents a number, starting at `self.offset`.
	/// First, each bit is set to 0, then the bits representing non-prime numbers are set to 1.
	/// These non-prime numbers are found by removing the multiples
	/// of the already known prime numbers, and then removing the multiples
	/// of the remaining numbers in the sieve from the sieve.
	/// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes for more information.
	sieve: BitSet,

	/// The number represented by the first bit of `self.sieve`.
	sieve_first: Inner,

	/// The effective number of bits that `self.sieve` uses.
	sieve_bits: Inner,
}

impl Prime {
	/// Creates a new Prime iterator instance and initializes its attributes.
	/// The newly created Prime iterator instance is used to get the prime numbers
	/// starting at `n`, generating the next one at each iteration.
	///
	/// # Paramters
	/// * `n` - The starting number.
	///
	/// # Return
	/// The newly created Prime iterator instance.
	///
	/// # Example
	/// ```
	/// use ex04::Prime;
	///
	/// let mut prime: Prime = Prime::new(0);
	/// ```
	pub fn new(n: Inner) -> Self {
		let primes: Vec<Inner> = Vec::new();
		let sieve: BitSet = BitSet::new();
		let sieve_first: Inner = 2;
		let remaining_numbers: Inner = Inner::MAX - sieve_first + 1;
		let sieve_bits: Inner = Inner::min(BitSet::BITS, remaining_numbers);

		Self { n, primes, sieve, sieve_first, sieve_bits }
	}

	/// Sets the bits of `self.sieve` to 0,
	/// and update `self.sieve_first` to represent the first number of the next chunk of numbers.
	fn fill_sieve_with_next_chunk(self: &mut Self) {
		// TODO
	}

	/// Sets the bits of the non-prime numbers in the sieve to 1.
	/// The non-prime numbers are found by the multiplying the prime numbers
	/// that are already in `self.primes`, and then multiplying the remaining
	/// numbers in the sieve from the sieve. (Yes, it sounds like an Inception)
	fn remove_non_prime_from_sieve(self: &mut Self) {
		#[inline(always)]
		fn set_bit_to_one(multiple: Inner, sieve: &mut [BitField; SIEVE_LEN], sieve_first: Inner) {
			let absolute_bit_index: usize = (multiple - sieve_first) as usize;
			let relative_bit_index: usize = absolute_bit_index % BitField::BITS as usize;
			let sieve_index: usize = absolute_bit_index / BitField::BITS as usize;

			sieve[sieve_index] |= 1 << relative_bit_index;
		}

		for prime in &self.primes {
			let mut multiple: Inner = match self.sieve_first.checked_next_multiple_of(*prime) {
				Some(multiple) => multiple,
				None => continue,
			};

			while multiple - self.sieve_first < self.sieve_bits {
				set_bit_to_one(multiple, &mut self.sieve, self.sieve_first);
				multiple += *prime;
			}
		}

		for i in 0..self.sieve.len() {
			for bit in 0..BitField::BITS {
				if self.sieve[i] & (1 << bit) == 0 {
					let prime: Inner =
						self.sieve_first + i as Inner * BitField::BITS as Inner + bit as Inner;

					let mut multiple: Inner = match prime.checked_mul(prime) {
						Some(square) => square,
						None => continue,
					};

					while multiple - self.sieve_first < self.sieve_bits {
						set_bit_to_one(multiple, &mut self.sieve, self.sieve_first);
						multiple += prime;
					}
				}
			}
		}
	}

	/// Transfers the first prime number from `self.sieve` to `self.primes`.
	/// The prime number is found by searching for the first bit that is set to 0
	/// in the binary representation of `self.sieve`.
	/// It is assumed that `self.sieve` contains at least 1 of it.
	/// Once the prime number has been found, it is removed from `self.sieve`,
	/// and appended to `self.primes`.
	fn transfer_first_from_sieve_to_primes(self: &mut Self) {
		for i in 0..self.sieve.len() {
			for bit in 0..BitField::BITS {
				let mask: BitField = 1 << bit;

				if self.sieve[i] & mask == 0 {
					let prime: Inner =
						self.sieve_first + i as Inner * BitField::BITS as Inner + bit as Inner;
					self.sieve[i] |= mask;
					self.primes.push(prime);
					return;
				}
			}
		}
	}
}

impl Iterator for Prime {
	type Item = u32;

	/// Generates the next prime number.
	///
	/// # Return
	/// * `Some(n)` - The next prime number.
	/// * `None` - There is no next prime number.
	///
	/// # Example
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
		let next_prime: Inner;

		while self.primes.is_empty() || self.n > *self.primes.last().unwrap() {
			while self.sieve.is_full_of_ones() {
				self.fill_sieve_with_next_chunk()?;
				self.remove_non_prime_from_sieve();
			}
			self.transfer_first_from_sieve_to_primes();
		}
		next_prime = *self.primes.last().unwrap();

		if self.sieve.is_full_of_ones() {
			self.fill_sieve_with_next_chunk();
			self.remove_non_prime_from_sieve();
		}
		if !self.sieve.is_full_of_ones() {
			self.transfer_first_from_sieve_to_primes();
			self.n = *self.primes.last().unwrap();
		}

		Some(next_prime)
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
}
