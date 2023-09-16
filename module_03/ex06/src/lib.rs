#[derive(Eq, Debug, PartialEq)]
struct Node<T> {
	value: T,
	next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	/// Create a new Node instance and initialize its attributes.
	///
	/// # Arguments
	/// * `value` - The value to be stored in the newly created Node instance.
	/// * `next` - The eventual Node instance that follows the newly created Node instance.
	///
	/// # Returns
	/// The newly created Node instance.
	///
	/// # Examples
	/// ```
	/// use ex06::Node;
	///
	/// let node: Node<u8> = Node::new(0x00, None);
	/// ```
	#[inline(always)]
	pub const fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
		Node { value, next }
	}
}

#[derive(Eq, Debug, PartialEq)]
pub struct List<T> {
	head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
	/// Create a new List instance and initialize its attributes.
	/// The newly created List instance is empty.
	///
	/// # Returns
	/// The newly created List instance.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let list: List<u8> = List::new();
	/// ```
	#[inline(always)]
	pub const fn new() -> Self {
		List { head: None }
	}

	/// Create a new Node instance, initialize its attributes,
	/// and insert it at the beginning of the calling List instance.
	///
	/// # Arguments
	/// * `value` - The value to be stored in the newly created Node instance.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_front(0x01);
	/// list.push_front(0x02);
	/// list.push_front(0x03);
	/// ```
	pub fn push_front(self: &mut Self, value: T) {
		let node: Box<Node<T>> = if let Some(head) = self.head.take() {
			Box::new(Node::new(value, Some(head)))
		} else {
			Box::new(Node::new(value, None))
		};

		self.head = Some(node);
	}

	/// Create a new Node instance, initialize its attributes,
	/// and insert it at the end of the calling List instance.
	///
	/// # Argumets
	/// * `value` - The value to be stored in the newly created Node instance.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x04);
	/// list.push_back(0x05);
	/// list.push_back(0x06);
	/// ```
	pub fn push_back(&mut self, value: T) {
		// TODO: Implemet this function.
	}

	/// # Returns
	/// The number of elements present in the calling List instance.
	///
	/// # Example
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// assert_eq!(list.count(), 0);
	///
	/// list.push_back(0x07);
	/// list.push_back(0x08);
	/// list.push_back(0x09);
	///
	/// assert_eq!(list.count(), 3);
	/// ```
	pub fn count(&self) -> usize {
		// TODO: Implemet this function.
	}

	/// Get a reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Arguments
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// A reference to the wanted element in the calling List instance.
	///
	/// # Panic
	/// The index is out of bounds.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x07);
	/// list.push_back(0x08);
	/// list.push_back(0x09);
	///
	/// assert_eq!(list.get(0), Some(&0x07));
	/// assert_eq!(list.get(1), Some(&0x08));
	/// assert_eq!(list.get(2), Some(&0x09));
	/// assert_eq!(list.get(3), None);
	/// ```
	pub fn get(self: &Self, mut i: usize) -> &T {
		// TODO: Implemet this function.
	}

	/// Get a mutable reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Arguments
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// A mutable reference to the wanted element in the calling List instance.
	///
	/// # Panic
	/// The index is out of bounds.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x0a);
	/// list.push_back(0x0b);
	/// list.push_back(0x0c);
	///
	/// assert_eq!(list.get_mut(0), Some(&mut 0x0a));
	/// assert_eq!(list.get_mut(1), Some(&mut 0x0b));
	/// assert_eq!(list.get_mut(2), Some(&mut 0x0c));
	/// assert_eq!(list.get_mut(3), None);
	/// ```
	pub fn get_mut(&mut self, mut i: usize) -> &mut T {
		// TODO: Implemet this function.
	}

	/// Remove the first element of the calling List instance.
	///
	/// # Returns
	/// The removed element.
	///
	/// # Panic
	/// The calling List instance is empty.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x0d);
	/// list.push_back(0x0e);
	/// list.push_back(0x0f);
	///
	/// assert_eq!(list.remove_front(), Some(0x0d));
	/// assert_eq!(list.remove_front(), Some(0x0e));
	/// assert_eq!(list.remove_front(), Some(0x0f));
	/// assert_eq!(list.remove_front(), None);
	/// ```
	pub fn remove_front(&mut self) -> T {
		// TODO: Implemet this function.
	}

	/// Remove the last element of the calling List instance.
	///
	/// # Returns
	/// The removed element.
	///
	/// # Panic
	/// * the calling List instance is empty.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x10);
	/// list.push_back(0x11);
	/// list.push_back(0x12);
	///
	/// assert_eq!(list.remove_back(), Some(0x12));
	/// assert_eq!(list.remove_back(), Some(0x11));
	/// assert_eq!(list.remove_back(), Some(0x10));
	/// assert_eq!(list.remove_back(), None);
	/// ```
	pub fn remove_back(&mut self) -> T {
		// TODO: Implemet this function.
	}

	/// Remove all the elements of the calling List instance.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x13);
	/// list.push_back(0x14);
	/// list.push_back(0x15);
	/// list.clear();
	/// ```
	pub fn clear(&mut self) {
		// TODO: Implemet this function.
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn node_new_00() {
		assert_eq!(
			Node::new(0xaa, None),
			Node {
				value: 0xaa,
				next: None
			}
		);
	}
}
