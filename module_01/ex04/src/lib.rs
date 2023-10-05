fn __partition(boxes: &mut [[u32; 2]], mut low: usize, mut high: usize) -> usize {
	let pivot: u32 = boxes[(high - low) / 2 + low][0];

	loop {
		while boxes[low][0] > pivot {
			low = low + 1;
		}
		while boxes[high][0] < pivot {
			high = high - 1;
		}
		if low >= high {
			return high;
		}

		boxes.swap(low, high);

		low = low + 1;
		high = high - 1;
	}
}

fn __quicksort(boxes: &mut [[u32; 2]], low: usize, high: usize) {
	if low < high {
		let pivot: usize = __partition(boxes, low, high);
		__quicksort(boxes, low, pivot);
		__quicksort(boxes, pivot + 1, high);
	}
}

/// Sort boxes in a way for every box to be contained in the previous one.
///
/// # Parameters
/// * `boxes` - A slice containing the boxes to sort.
///
/// # Panics
/// No such sorting is possible.
///
/// # Examples
/// ```
/// use ex04::sort_boxes;
///
/// let mut boxes: [[u32; 2]; 4] = [[1, 2], [2, 3], [3, 4], [4, 5]];
///
/// sort_boxes(&mut boxes);
///
/// assert_eq!(boxes, [[4, 5], [3, 4], [2, 3], [1, 2]]);
/// ```
pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
	if boxes.len() < 2 {
		return;
	}

	__quicksort(boxes, 0, boxes.len() - 1);

	for i in 0..boxes.len() - 1 {
		if boxes[i][1] < boxes[i + 1][1] {
			panic!("Sorting is not possible");
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn sort_boxes_00() {
		let mut boxes: [[u32; 2]; 0] = [];

		sort_boxes(&mut boxes);
		assert_eq!(boxes, [] as [[u32; 2]; 0]);
	}

	#[test]
	fn sort_boxes_01() {
		let mut boxes: [[u32; 2]; 1] = [[1, 1]];

		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[1, 1]]);
	}

	#[test]
	fn sort_boxes_02() {
		let mut boxes: [[u32; 2]; 2] = [[1, 1], [2, 2]];

		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[2, 2], [1, 1]]);
	}

	#[test]
	fn sort_boxes_03() {
		let mut boxes: [[u32; 2]; 3] = [[1, 2], [2, 3], [3, 4]];

		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[3, 4], [2, 3], [1, 2]]);
	}

	#[test]
	fn sort_boxes_04() {
		let mut boxes: [[u32; 2]; 20] = [
			[180, 90],
			[180, 90],
			[140, 70],
			[140, 70],
			[100, 50],
			[100, 50],
			[60, 30],
			[60, 30],
			[20, 10],
			[20, 10],
			[40, 20],
			[40, 20],
			[80, 40],
			[80, 40],
			[120, 60],
			[120, 60],
			[160, 80],
			[160, 80],
			[200, 100],
			[200, 100],
		];

		sort_boxes(&mut boxes);
		assert_eq!(
			boxes,
			[
				[200, 100],
				[200, 100],
				[180, 90],
				[180, 90],
				[160, 80],
				[160, 80],
				[140, 70],
				[140, 70],
				[120, 60],
				[120, 60],
				[100, 50],
				[100, 50],
				[80, 40],
				[80, 40],
				[60, 30],
				[60, 30],
				[40, 20],
				[40, 20],
				[20, 10],
				[20, 10],
			]
		);
	}

	#[test]
	#[should_panic(expected = "Sorting is not possible")]
	fn sort_boxes_05() {
		let mut boxes: [[u32; 2]; 2] = [[1, 3], [2, 2]];

		sort_boxes(&mut boxes);
	}

	#[test]
	#[should_panic(expected = "Sorting is not possible")]
	fn sort_boxes_06() {
		let mut boxes: [[u32; 2]; 2] = [[3, 1], [2, 2]];

		sort_boxes(&mut boxes);
	}
}
