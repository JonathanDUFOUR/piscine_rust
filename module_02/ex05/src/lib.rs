#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

impl Color {
	/// Calculates the distance with another color.
	///
	/// # Parameters
	/// * `other` - The color to calculate the distance with.
	///
	/// # Return
	/// The distance between the two colors.
	fn distance(self: &Self, other: &Self) -> u32 {
		let diff_red: i32 = self.red as i32 - other.red as i32;
		let diff_green: i32 = self.green as i32 - other.green as i32;
		let diff_blue: i32 = self.blue as i32 - other.blue as i32;

		(diff_red * diff_red + diff_green * diff_green + diff_blue * diff_blue) as u32
	}

	/// Adds a color to the canvas, and returns the resulting color.
	/// The canvas is assumed to be completly opaque.
	///
	/// # Parameters
	/// * `color` - The color to add to the canvas,
	/// represented by a tuple containing the Color instance and its opacity.
	///
	/// # Return
	/// The resulting color.
	fn mix_color_to_canvas(self: &Self, canvas: &Self, opacity: u8) -> Self {
		#[inline(always)]
		fn mix_component(a: u8, b: u8, opacity: u8) -> u8 {
			return ((a as u16 * opacity as u16 + b as u16 * (255 - opacity) as u16)
				/ u8::MAX as u16) as u8;
		}

		Self::new(
			mix_component(self.red, canvas.red, opacity),
			mix_component(self.green, canvas.green, opacity),
			mix_component(self.blue, canvas.blue, opacity),
		)
	}

	/// Finds out the mix of colors that results in the closest color to the calling instance,
	/// using a well defined number of colors to mix.
	///
	/// # Parameters
	/// * `canvas` - The current color of the canvas we are painting on.
	/// * `closest` - The closest resulting mixed color we found so far.
	/// * `palette` - The palette of colors to mix.
	/// * `number_of_colors_to_mix` - The remaining number of colors we must add to the mix.
	///
	/// # Return
	/// The resulting mixed color that is closest to the `self` color.
	fn mix_recursively(
		self: &Self,
		canvas: &Self,
		closest: &mut Self,
		palette: &[(Self, u8)],
		number_of_colors_to_mix: u32,
	) -> Self {
		if number_of_colors_to_mix == 0 {
			return canvas.clone();
		}

		for i in 0..palette.len() {
			let current: Self = self.mix_recursively(
				&palette[i].0.mix_color_to_canvas(canvas, palette[i].1),
				closest,
				palette,
				number_of_colors_to_mix - 1,
			);

			if current.distance(self) < closest.distance(self) {
				if current == *self {
					return current;
				}
				*closest = current;
			}
		}

		closest.clone()
	}

	pub const RED: Self = Self::new(0xff, 0x00, 0x00);
	pub const GREEN: Self = Self::new(0x00, 0xff, 0x00);
	pub const BLUE: Self = Self::new(0x00, 0x00, 0xff);
	pub const WHITE: Self = Self::new(0xff, 0xff, 0xff);

	/// Creates a new Color instance and initializes its attributes.
	///
	/// # Parameters
	/// * `red` - The red component of the color.
	/// * `green` - The green component of the color.
	/// * `blue` - The blue component of the color.
	///
	/// # Return
	/// The newly created and initialized Color instance.
	///
	/// # Examples
	/// ```
	/// use ex05::Color;
	///
	/// let color: Color = Color::new(255, 0, 0);
	///
	/// assert_eq!(color, Color::RED);
	/// ```
	#[inline(always)]
	pub const fn new(red: u8, green: u8, blue: u8) -> Self {
		Self { red, green, blue }
	}

	/// Tries mixing colors as if painted on a white canvas to obtain a result as close as possible
	/// to the calling instance.
	///
	/// # Parameters
	/// * `palette` - The palette of colors to mix.
	/// * `max` - The maximum number of colors to mix.
	///
	/// # Return
	/// The resulting mixed color that is closest to the `self` color.
	///
	/// # Examples
	/// ```
	/// use ex05::Color;
	///
	/// assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
	/// assert_eq!(Color::RED.closest_mix(&[(Color::RED, 255)], 0), Color::WHITE);
	/// assert_eq!(
	/// 	Color::new(254, 23, 102).closest_mix(
	/// 		&[
	/// 			(Color::RED, 100),
	/// 			(Color::GREEN, 100),
	/// 			(Color::BLUE, 100),
	/// 		],
	/// 		5),
	/// 	Color::new(217, 34, 71)
	/// );
	/// ```
	pub fn closest_mix(self: &Self, palette: &[(Self, u8)], max: u32) -> Self {
		if *self == Self::WHITE || palette.len() == 0 || max == 0 {
			return Self::WHITE;
		}

		let mut closest: Self = Self::WHITE;
		for number_of_colors_to_mix in 1..=max {
			let current: Self =
				self.mix_recursively(&Self::WHITE, &mut closest, palette, number_of_colors_to_mix);

			if current == *self {
				return current;
			}
		}

		closest
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ntest::timeout;

	#[test]
	#[timeout(25)]
	fn new_00() {
		assert_eq!(Color::new(0x00, 0x00, 0x00), Color { red: 0x00, green: 0x00, blue: 0x00 });
	}

	#[test]
	#[timeout(25)]
	fn new_01() {
		assert_eq!(Color::new(0xff, 0xff, 0xff), Color { red: 0xff, green: 0xff, blue: 0xff });
	}

	#[test]
	#[timeout(25)]
	fn new_02() {
		assert_eq!(Color::new(0x11, 0x22, 0x33), Color { red: 0x11, green: 0x22, blue: 0x33 });
	}

	#[test]
	#[timeout(25)]
	fn new_03() {
		assert_eq!(Color::new(0x12, 0x34, 0x56), Color { red: 0x12, green: 0x34, blue: 0x56 });
	}

	#[test]
	#[timeout(25)]
	fn new_04() {
		assert_eq!(Color::new(0x78, 0x9a, 0xbc), Color { red: 0x78, green: 0x9a, blue: 0xbc });
	}

	#[test]
	#[timeout(25)]
	fn constant_red() {
		assert_eq!(Color::RED, Color::new(0xff, 0x00, 0x00));
	}

	#[test]
	#[timeout(25)]
	fn constant_green() {
		assert_eq!(Color::GREEN, Color::new(0x00, 0xff, 0x00));
	}

	#[test]
	#[timeout(25)]
	fn constant_blue() {
		assert_eq!(Color::BLUE, Color::new(0x00, 0x00, 0xff));
	}

	#[test]
	#[timeout(25)]
	fn constant_white() {
		assert_eq!(Color::WHITE, Color::new(0xff, 0xff, 0xff));
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_00() {
		assert_eq!(Color::WHITE.closest_mix(&[], 0), Color::WHITE);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_01() {
		assert_eq!(Color::WHITE.closest_mix(&[], u32::MAX), Color::WHITE);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_02() {
		assert_eq!(
			Color::WHITE.closest_mix(
				&[
					(Color::new(0x21, 0x42, 0x84), 0x7b),
					(Color::new(0x99, 0x66, 0x33), 0x33),
				],
				0
			),
			Color::WHITE
		);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_03() {
		assert_eq!(
			Color::WHITE.closest_mix(
				&[
					(Color::new(0x2a, 0xf0, 0x07), 0x76),
					(Color::new(0x8c, 0x39, 0xa2), 0xda),
				],
				u32::MAX
			),
			Color::WHITE
		);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_04() {
		assert_eq!(Color::RED.closest_mix(&[], 0), Color::WHITE);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_05() {
		assert_eq!(Color::RED.closest_mix(&[], u32::MAX), Color::WHITE);
	}

	#[test]
	#[timeout(100)]
	fn closest_mix_06() {
		assert_eq!(
			Color::RED.closest_mix(
				&[
					(Color::new(0x2a, 0x18, 0xf0), 0x0d),
					(Color::new(0x03, 0xfe, 0xd5), 0x26),
				],
				0
			),
			Color::WHITE
		);
	}

	#[test]
	#[timeout(5000)]
	fn closest_mix_07() {
		assert_eq!(
			Color::new(0xfe, 0xfe, 0xfe).closest_mix(
				&[
					(Color::new(0xff, 0xff, 0x00), 0x42),
					(Color::new(0xff, 0x00, 0xff), 0x42),
					(Color::new(0x00, 0xff, 0xff), 0x42),
				],
				7
			),
			Color::WHITE
		)
	}

	#[test]
	#[timeout(500)]
	fn closest_mix_08() {
		assert_eq!(
			Color::RED.closest_mix(
				&[
					(Color::new(0x00, 0x00, 0x00), 0xff),
					(Color::WHITE, 0xff),
					(Color::RED, 0xff),
					(Color::GREEN, 0xff),
					(Color::BLUE, 0xff),
				],
				u32::MAX
			),
			Color::RED
		)
	}

	#[test]
	#[timeout(1000)]
	fn closest_mix_09() {
		assert_eq!(
			Color::new(0x80, 0x80, 0x80).closest_mix(
				&[
					(Color::new(0x00, 0x00, 0x00), 0x80),
					(Color::new(0x11, 0x11, 0x11), 0x80),
					(Color::new(0x22, 0x22, 0x22), 0x80),
					(Color::new(0x33, 0x33, 0x33), 0x80),
					(Color::new(0x44, 0x44, 0x44), 0x80),
					(Color::new(0x55, 0x55, 0x55), 0x80),
					(Color::new(0x66, 0x66, 0x66), 0x80),
					(Color::new(0x77, 0x77, 0x77), 0x80),
					(Color::new(0x88, 0x88, 0x88), 0x80),
					(Color::new(0x99, 0x99, 0x99), 0x80),
					(Color::new(0xaa, 0xaa, 0xaa), 0x80),
					(Color::new(0xbb, 0xbb, 0xbb), 0x80),
					(Color::new(0xcc, 0xcc, 0xcc), 0x80),
					(Color::new(0xdd, 0xdd, 0xdd), 0x80),
					(Color::new(0xee, 0xee, 0xee), 0x80),
					(Color::new(0xff, 0xff, 0xff), 0x80),
				],
				u32::MAX
			),
			Color::new(0x80, 0x80, 0x80)
		);
	}

	#[test]
	#[timeout(250)]
	fn closest_mix_10() {
		assert_eq!(
			Color::new(0x91, 0x1f, 0x3d).closest_mix(
				&[
					(Color::new(0x8d, 0x1b, 0x39), 0xff),
					(Color::new(0x94, 0x22, 0x40), 0xff),
					(Color::new(0x8f, 0x1d, 0x3b), 0xff),
					(Color::new(0x92, 0x20, 0x3e), 0xff),
				],
				1
			),
			Color::new(0x92, 0x20, 0x3e)
		);
	}

	#[test]
	#[timeout(3000)]
	fn closest_mix_11() {
		assert_eq!(
			Color::new(0x33, 0x02, 0xd1).closest_mix(
				&[
					(Color::RED, 0x42),
					(Color::GREEN, 0x42),
					(Color::BLUE, 0x42),
				],
				11
			),
			Color::new(0x33, 0x08, 0xd1)
		);
	}

	#[test]
	#[timeout(1000)]
	fn closest_mix_12() {
		assert_eq!(
			Color::new(0x58, 0xe4, 0x0a).closest_mix(
				&[
					(Color::new(0x00, 0x00, 0x00), 0x21),
					(Color::new(0x1c, 0xdb, 0x81), 0xa2),
					(Color::new(0x8e, 0x49, 0xa3), 0x14),
					(Color::new(0x3f, 0x0e, 0xb6), 0xe4),
					(Color::new(0xd8, 0x44, 0x15), 0x9b),
				],
				5
			),
			Color::new(0x5e, 0xa7, 0x5c)
		);
	}
}
