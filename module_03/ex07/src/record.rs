trait Record: Sized {
	fn decode(line: &str) -> Result<Self, DecodingError> {}
	fn encode(&self, target: &mut String) -> Result<(), EncodingError> {}
}
