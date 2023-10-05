use crate::error::{DecodingError, EncodingError};

pub trait Field: Sized {
	/// Decodes a field from its `str` representation to its concrete type value.
	///
	/// # Parameters
	/// * `field` - The field to decode.
	///
	/// # Returns
	/// - `Ok(Self)` if the field was successfully decoded.
	/// - `Err(DecodingError)` if the field could not be decoded.
	fn decode(field: &str) -> Result<Self, DecodingError>;

	/// Encodes a field from its concrete type value to its `str` representation.
	///
	/// # Parameters
	/// * `self` - The field to encode.
	/// * `target` - The string to append the encoded field to.
	///
	/// # Returns
	/// - `Ok(())` if the field was successfully encoded.
	/// - `Err(EncodingError)` if the field could not be encoded.
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError>;
}

impl Field for String {
	fn decode(field: &str) -> Result<Self, DecodingError> {
		Ok(field.to_string())
	}

	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {
		if self.contains([',', '\n']) {
			Err(EncodingError)
		} else {
			Ok(target.push_str(self))
		}
	}
}

impl<T> Field for Option<T>
where
	T: Field,
{
	fn decode(field: &str) -> Result<Self, DecodingError> {
		if field.is_empty() {
			return Ok(None);
		}
		match T::decode(field) {
			Ok(value) => Ok(Some(value)),
			Err(err) => Err(err),
		}
	}
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {
		match self {
			Some(value) => value.encode(target),
			None => Ok(()),
		}
	}
}

macro_rules! impl_field_for_int {
	$(($type:ty), *) => {
		$(
			impl Field for $type {
				fn decode(field: &str) -> Result<Self, DecodingError> {
					match field.parse() {
						Ok(value) => Ok(value),
						Err(_) => Err(DecodingError),
					}
				}

				fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {
					match write!(target, "{}", self) {
						Ok(()) => Ok(()),
						Err(_) => Err(EncodingError),
					}
				}
			}
		)*
	};
}

impl_field_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
