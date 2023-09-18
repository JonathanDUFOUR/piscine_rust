trait Field: Sized {
	fn decode(field: &str) -> Result<Self, DecodingError>;
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError>;
}
