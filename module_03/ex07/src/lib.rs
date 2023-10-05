mod error;
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
mod tests {}
