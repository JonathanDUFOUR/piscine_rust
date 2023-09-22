mod error;
mod record;

use crate::error::{DecodingError, EncodingError};
use crate::record::Record;

pub fn decode_csv<R: Record>(contents: &str) -> Result<Vec<R>, DecodingError> {
	// TODO: implement this function.
	Err(DecodingError {})
}
pub fn encode_csv<R: Record>(records: &[R]) -> Result<String, EncodingError> {
	// TODO: implement this function.
	Err(EncodingError {})
}

#[cfg(test)]
mod tests {}
