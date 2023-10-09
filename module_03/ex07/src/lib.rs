mod error;
mod field;
mod record;

use crate::error::{DecodingError, EncodingError};
use crate::record::Record;

/// Decodes a CSV content from its `str` representation to a collection of records.
///
/// # Type parameters
/// * `R` - The record type to decode.
///
/// # Parameters
/// * `content` - The CSV content to decode.
///
/// # Returns
/// - `Ok(Vec<R>)` if the CSV content was successfully decoded.
/// - `Err(DecodingError)` if the CSV content could not be decoded.
pub fn decode_csv<R: Record>(content: &str) -> Result<Vec<R>, DecodingError> {
	let mut records: Vec<R> = Vec::new();

	for line in content.lines() {
		match R::decode(line) {
			Ok(record) => records.push(record),
			Err(err) => return Err(err),
		}
	}

	Ok(records)
}

/// Encodes a CSV content from a collection of records to its `str` representation.
///
/// # Type parameters
/// * `R` - The record type to encode.
///
/// # Parameters
/// * `records` - The records to encode.
///
/// # Returns
/// - `Ok(String)` if the CSV content was successfully encoded.
/// - `Err(EncodingError)` if the CSV content could not be encoded.
pub fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError> {
	let mut content: String = String::new();

	for record in records {
		match record.encode(&mut content) {
			Ok(_) => content.push('\n'),
			Err(err) => return Err(err),
		}
	}

	Ok(content)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::field::Field;

	#[derive(Debug, Eq, PartialEq)]
	struct A {}

	impl_record_for_struct!(A {});

	#[derive(Debug, Eq, PartialEq)]
	struct B {
		a: String,
		b: Option<u8>,
	}

	impl_record_for_struct!(B {
		a: String,
		b: Option<u8>,
	});

	#[derive(Debug, Eq, PartialEq)]
	struct C {
		a: u8,
		b: u16,
		c: u32,
		d: u64,
		e: u128,
		f: usize,
		g: i8,
		h: i16,
		i: i32,
		j: i64,
		k: i128,
		l: isize,
	}

	impl_record_for_struct!(C {
		a: u8,
		b: u16,
		c: u32,
		d: u64,
		e: u128,
		f: usize,
		g: i8,
		h: i16,
		i: i32,
		j: i64,
		k: i128,
		l: isize,
	});

	// region: decode_csv_00
	#[test]
	fn test_decode_csv_00() {
		let content: &str = "";
		let records: Vec<A> = match decode_csv(content) {
			Ok(value) => value,
			Err(DecodingError) => panic!("could not decode CSV"),
		};

		assert_eq!(records, vec![]);
	}
	// endregion

	// region: decode_csv_01
	#[test]
	fn test_decode_csv_01() {
		let content: &str = "\
			Hello,\n\
			,0\n\
			World!,42\n\
		";
		let records: Vec<B> = match decode_csv(content) {
			Ok(value) => value,
			Err(DecodingError) => panic!("could not decode CSV"),
		};

		assert_eq!(
			records,
			vec![
				B {
					a: "Hello".to_string(),
					b: None,
				},
				B {
					a: "".to_string(),
					b: Some(0),
				},
				B {
					a: "World!".to_string(),
					b: Some(42),
				},
			]
		);
	}
	// endregion

	// region: decode_csv_02
	#[test]
	fn test_decode_csv_02() {
		let content: &str = "0,1,2,3,4,5,6,7,8,9,10,11\n";
		let records: Vec<C> = match decode_csv(content) {
			Ok(value) => value,
			Err(DecodingError) => panic!("could not decode CSV"),
		};

		assert_eq!(
			records,
			vec![C {
				a: 0,
				b: 1,
				c: 2,
				d: 3,
				e: 4,
				f: 5,
				g: 6,
				h: 7,
				i: 8,
				j: 9,
				k: 10,
				l: 11,
			},]
		);
	}
	// endregion

	// region: encode_csv_00
	#[test]
	fn test_encode_csv_00() {
		let records: Vec<A> = vec![];
		let content: String = match encode_csv(&records) {
			Ok(value) => value,
			Err(EncodingError) => panic!("could not encode CSV"),
		};

		assert_eq!(content, "");
	}
	// endregion

	// region: encode_csv_01
	#[test]
	fn test_encode_csv_01() {
		let records: Vec<B> = vec![B {
			a: "Never gonna give you up".to_string(),
			b: Some(98),
		}];
		let content: String = match encode_csv(&records) {
			Ok(value) => value,
			Err(EncodingError) => panic!("could not encode CSV"),
		};

		assert_eq!(content, "Never gonna give you up,98\n");
	}
	// endregion

	// region: encode_csv_02
	#[test]
	fn test_encode_csv_02() {
		let records: Vec<C> = vec![C {
			a: 66,
			b: 65,
			c: 63,
			d: 60,
			e: 56,
			f: 51,
			g: 45,
			h: 38,
			i: 30,
			j: 21,
			k: 11,
			l: 0,
		}];
		let content: String = match encode_csv(&records) {
			Ok(value) => value,
			Err(EncodingError) => panic!("could not encode CSV"),
		};

		assert_eq!(content, "66,65,63,60,56,51,45,38,30,21,11,0\n");
	}
	// endregion
}
