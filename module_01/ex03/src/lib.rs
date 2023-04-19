/// Search for the largest subslice of a slice that contains all and only specified values.
///
/// # Arguments
///
/// * `haystack` - The slice to search in.
/// * `needle` - The slice of values to search for.
///
/// # Returns
///
/// The largest subslice of `haystack` that contains all and only values in `needles`.
///
/// # Examples
///
/// ```
/// use ex03::largest_group;
///
/// assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
/// ```
pub fn largest_group<'lifetime>(haystack: &'lifetime [u32], needle: &[u32]) -> &'lifetime [u32] {
	let mut subslice: &[u32] = &[];

	return subslice;
}
