use crate::error::{DecodingError, EncodingError};

pub trait Record: Sized {
	fn decode(line: &str) -> Result<Self, DecodingError>;
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError>;
}
