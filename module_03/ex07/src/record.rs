use crate::error::{DecodingError, EncodingError};

pub trait Record: Sized {
	/// Decodes a record from its `str` representation to its concrete type value.
	///
	/// # Parameters
	/// * `line` - The line to decode.
	///
	/// # Return
	/// * `Ok(Self)` - The decoded record.
	/// * `Err(DecodingError)` - The line could not be decoded.
	fn decode(line: &str) -> Result<Self, DecodingError>;

	/// Encodes a record from its concrete type value to its `str` representation,
	/// and appends it to a target string.
	/// Note that the resulting `str` representation is not terminated by a newline.
	///
	/// # Parameters
	/// * `self` - The record to encode.
	/// * `target` - The string to append the encoded record to.
	///
	/// # Return
	/// * `Ok(())` - The record was successfully encoded.
	/// * `Err(EncodingError)` - The record could not be encoded.
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError>;
}

#[macro_export]
macro_rules! impl_record_for_struct {
	($struct_identifier:ident {}) => {
		impl Record for $struct_identifier {
			fn decode(line: &str) -> Result<Self, DecodingError> {
				let mut fields: std::str::Split<'_, char> = line.split(',');

				match fields.next() {
					Some(field) if field.is_empty() => (),
					_ => return Err(DecodingError),
				};
				if fields.next().is_some() {
					return Err(DecodingError);
				}

				Ok($struct_identifier {})
			}

			fn encode(&self, _target: &mut String) -> Result<(), EncodingError> {
				Ok(())
			}
		}
	};
	($struct_identifier:ident {
			$first_field_identifier:ident: $first_field_type:ty,
			$($next_field_identifier:ident: $next_field_type:ty),*
			$(,)?
		}
	) => {
		impl Record for $struct_identifier {
			fn decode(line: &str) -> Result<Self, DecodingError> {
				let mut fields: std::str::Split<'_, char> = line.split(',');

				let $first_field_identifier: $first_field_type = match fields.next() {
					Some(field) => Field::decode(field)?,
					None => return Err(DecodingError),
				};
				$(
					let $next_field_identifier: $next_field_type = match fields.next() {
						Some(field) => Field::decode(field)?,
						None => return Err(DecodingError),
					};
				)*
				if fields.next().is_some() {
					return Err(DecodingError);
				}

				Ok($struct_identifier {
					$first_field_identifier,
					$($next_field_identifier),*
				})
			}

			fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {
				Field::encode(&self.$first_field_identifier, target)?;
				$(
					target.push(',');
					Field::encode(&self.$next_field_identifier, target)?;
				)*

				Ok(())
			}
		}
	};
}
