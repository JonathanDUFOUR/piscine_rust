struct Node<T> {
	value: T,
	next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	/// Create a new Node instance and initialize its attributes.
	/// The newly created Node instance is isolated.
	///
	/// # Arguments
	///
	/// * `value` - The value to be stored in the newly created Node instance.
	///
	/// # Returns
	///
	/// The newly created Node instance.
	///
	/// # Examples
	/// ```
	/// use ex06::Node;
	///
	/// let node: Node<u8> = Node::new(0x00);
	/// ```
	#[inline(always)]
	pub const fn new(value: T) -> Self {
		Node { value, next: None }
	}
}

pub struct List<T> {
	head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
	/// Create a new List instance and initialize its attributes.
	/// The newly created List instance is empty.
	///
	/// # Returns
	///
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
	///
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
		let new_node: Box<Node<T>> = Box::new(Node::new(value));

		match self.head {
			Some(node) => new_node.next = Some(node),
			None => (),
		}
		self.head = Some(new_node);
	}

	/// Create a new Node instance, initialize its attributes,
	/// and insert it at the end of the calling List instance.
	///
	/// # Argumets
	///
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
	fn push_back(&mut self, value: T) {
		let new_node: Box<Node<T>> = Box::new(Node::new(value));

		match self.head {
			Some(node) => {
				let mut curr: Box<Node<T>> = node;

				loop {
					match curr.next {
						Some(next) => curr = next,
						None => break,
					}
				}

				curr.next = Some(new_node);
			}
			None => self.head = Some(new_node),
		}
	}

	/// # Returns
	///
	/// The number of elements present in the calling List instance.
	///
	/// # Example
	/// ```
	/// use ex06::List;
	///
	/// let list: List<u8> = List::new();
	///
	/// list.push_back(0x07);
	/// list.push_back(0x08);
	/// list.push_back(0x09);
	///
	/// assert_eq!(list.count(), 3);
	/// ```
	fn count(&self) -> usize {
		let mut count: usize = 0;
		let mut curr: Option<Box<Node<T>>> = self.head;

		loop {
			match curr {
				Some(node) => {
					count += 1;
					curr = node.next
				}
				None => break,
			}
		}

		count
	}

	/// Get a reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Arguments
	///
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	///
	/// A reference to the wanted element in the calling List instance.
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
	/// ```
	fn get(&self, i: usize) -> Option<&T> {
		// TODO: Implement this function.
		None
	}

	/// Get a mutable reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Arguments
	///
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	///
	/// A mutable reference to the wanted element in the calling List instance.
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
	/// ```
	fn get_mut(&mut self, i: usize) -> Option<&mut T> {
		// TODO: Implement this function.
		None
	}

	/// Remove the first element of the calling List instance.
	///
	/// # Returns
	///
	/// The removed element.
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
	/// ```
	fn remove_front(&mut self) -> Option<T> {
		// TODO: Implement this function.
		None
	}

	/// Remove the last element of the calling List instance.
	///
	/// # Returns
	///
	/// The removed element.
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
	/// ```
	fn remove_back(&mut self) -> Option<T> {
		// TODO: Implement this function.
		None
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
	fn clear(&mut self) {
		// TODO: Implemet this function.
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
