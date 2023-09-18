impl Field for String {
	fn decode(field: &str) -> Result<Self, DecodingError> {}
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {}
}
