impl<T> Field for Option<T>
where
	T: Field,
{
	fn decode(field: &str) -> Result<Self, DecodingError> {}
	fn encode(self: &Self, target: &mut String) -> Result<(), EncodingError> {}
}
