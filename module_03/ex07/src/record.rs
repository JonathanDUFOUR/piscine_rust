use crate::error::{DecodingError, EncodingError};

pub trait Record: Sized {
	/// Decodes a record from its `str` representation to its concrete collection value.
	///
	/// # Arguments
	/// * `line` - The line to decode.
	///
	/// # Returns
	/// - `Ok(Self)` if the line was successfully decoded.
	/// - `Err(DecodingError)` if the line could not be decoded.
	fn decode(line: &str) -> Result<Self, DecodingError>;

	/// Encodes a record from its concrete collection value to its `str` representation.
	///
	/// # Arguments
	/// * `self` - The record to encode.
	/// * `target` - The string to append the encoded record to.
	///
	/// # Returns
	/// - `Ok(())` if the record was successfully encoded.
	/// - `Err(EncodingError)` if the record could not be encoded.
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError>;
}
