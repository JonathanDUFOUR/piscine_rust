/// Get the name of a color, given its RGB value.
///
/// # Arguments
///
/// * `color` - A slice of RGB values.
///
/// # Examples
///
/// ```
/// use ex02::color_name;
///
/// assert_eq!(color_name(&[255, 0, 0]), "pure red");
/// ```
pub fn color_name(color: &[u8; 3]) -> &str {
	match color {
		[0, 0, 0] => "pure black",
		[255, 255, 255] => "pure white",
		[255, 0, 0] => "pure red",
		[0, 255, 0] => "pure green",
		[0, 0, 255] => "pure blue",
		[128, 128, 128] => "pure grey",
		[0..=31, 0..=31, 0..=31] => "almost black",
		[129..=255, 0..=127, 0..=127] => "redish",
		[0..=127, 129..=255, 0..=127] => "greenish",
		[0..=127, 0..=127, 129..=255] => "blueish",
		_ => "unknown",
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(test)]
	#[test]
	fn test_lifetimes() {
		let name_of_the_best_color;

		{
			let the_best_color = [42, 42, 42];
			name_of_the_best_color = color_name(&the_best_color);
		}

		assert_eq!(name_of_the_best_color, "unknown");
	}
}
