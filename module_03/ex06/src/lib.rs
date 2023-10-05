#[derive(Clone, Eq, Debug, PartialEq)]
struct Node<T> {
	value: T,
	next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	/// Create a new Node instance and initialize its attributes.
	///
	/// # Parameters
	/// * `value` - The value to be stored in the newly created Node instance.
	/// * `next` - The eventual Node instance that follows the newly created Node instance.
	///
	/// # Returns
	/// The newly created Node instance.
	#[inline(always)]
	const fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
		Node { value, next }
	}
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
	/// # Parameters
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
	pub fn push_back(self: &mut Self, value: T) {
		let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

		while let Some(node) = current {
			current = &mut node.next;
		}

		*current = Some(Box::new(Node::new(value, None)));
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
	pub fn count(self: &Self) -> usize {
		let mut count: usize = 0;
		let mut current: &Option<Box<Node<T>>> = &self.head;

		while let Some(node) = current {
			count += 1;
			current = &node.next;
		}

		count
	}

	/// Get a reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Parameters
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// * `Some(&T)` - A reference to the wanted element in the calling List instance.
	/// * `None` - The index is out of bounds.
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
	pub fn get(self: &Self, mut i: usize) -> Option<&T> {
		let mut current: &Option<Box<Node<T>>> = &self.head;

		while let Some(node) = current {
			if i == 0 {
				return Some(&node.value);
			}

			i -= 1;
			current = &node.next;
		}

		None
	}

	/// Get a mutable reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Parameters
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// * `Some(&mut T)` - A mutable reference to the wanted element in the calling List instance.
	/// * `None` - The index is out of bounds.
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
	pub fn get_mut(self: &mut Self, mut i: usize) -> Option<&mut T> {
		let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

		while let Some(ref mut node) = current {
			if i == 0 {
				return Some(&mut node.value);
			}

			i -= 1;
			current = &mut node.next;
		}

		None
	}

	/// Remove the first element of the calling List instance.
	///
	/// # Returns
	/// * `Some(T)` - The removed element.
	/// * `None` - The calling List instance is empty.
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
	pub fn remove_front(self: &mut Self) -> Option<T> {
		if let Some(mut head) = self.head.take() {
			self.head = head.next.take();
			Some(head.value)
		} else {
			None
		}
	}

	/// Remove the last element of the calling List instance.
	///
	/// # Returns
	/// * `Some(T)` - The removed element.
	/// * `None` - The calling List instance is empty.
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
	pub fn remove_back(self: &mut Self) -> Option<T> {
		if self.head.is_some() {
			let mut current: &mut Option<Box<Node<T>>> = &mut self.head;

			while current.is_some() {
				if current.as_ref().unwrap().next.is_none() {
					break;
				}
				current = &mut current.as_mut().unwrap().next;
			}

			Some(current.take().unwrap().value)
		} else {
			None
		}
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
	pub fn clear(self: &mut Self) {
		self.head = None;
	}
}

impl<T> std::ops::Index<usize> for List<T> {
	type Output = T;

	/// Get a reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Parameters
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// A reference to the wanted element in the calling List instance.
	///
	/// # Panics
	/// The index is out of bounds.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x16);
	/// list.push_back(0x17);
	/// list.push_back(0x18);
	///
	/// assert_eq!(list[0], 0x16);
	/// assert_eq!(list[1], 0x17);
	/// assert_eq!(list[2], 0x18);
	/// ```
	fn index(self: &Self, i: usize) -> &Self::Output {
		match self.get(i) {
			Some(value) => value,
			None => panic!("tried to access out of bound index {i}"),
		}
	}
}

impl<T> std::ops::IndexMut<usize> for List<T> {
	/// Get a mutable reference
	/// to the element located at a specific index in the calling List instance.
	///
	/// # Parameters
	/// * `i` - The index of the wanted element.
	///
	/// # Returns
	/// A mutable reference to the wanted element in the calling List instance.
	///
	/// # Panics
	/// The index is out of bounds.
	///
	/// # Examples
	/// ```
	/// use ex06::List;
	///
	/// let mut list: List<u8> = List::new();
	///
	/// list.push_back(0x19);
	/// list.push_back(0x1a);
	/// list.push_back(0x1b);
	///
	/// assert_eq!(list[0], 0x19);
	/// assert_eq!(list[1], 0x1a);
	/// assert_eq!(list[2], 0x1b);
	/// ```
	fn index_mut(self: &mut Self, i: usize) -> &mut Self::Output {
		match self.get_mut(i) {
			Some(value) => value,
			None => panic!("tried to access out of bound index {i}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	struct A {}

	impl A {
		#[inline(always)]
		const fn new() -> Self {
			Self {}
		}
	}

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	struct B {
		n: u8,
	}

	impl B {
		#[inline(always)]
		const fn new(n: u8) -> Self {
			Self { n }
		}
	}

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	struct C {
		n: i8,
	}

	impl C {
		#[inline(always)]
		const fn new(n: i8) -> Self {
			Self { n }
		}
	}

	// region: node_new_00
	#[test]
	fn node_new_00() {
		let node: Node<A> = Node::new(A::new(), None);

		assert_eq!(
			node,
			Node {
				value: A::new(),
				next: None
			}
		);
	}
	// endregion

	// region: node_new_01
	#[test]
	fn node_new_01() {
		let node0: Node<B> = Node::new(B::new(0x12), None);
		let node1: Node<B> = Node::new(B::new(0x23), Some(Box::new(node0)));

		assert_eq!(
			node1,
			Node {
				value: B::new(0x23),
				next: Some(Box::new(Node {
					value: B::new(0x12),
					next: None
				}))
			}
		);
	}
	// endregion

	// region: node_new_02
	#[test]
	fn node_new_02() {
		let node0: Node<C> = Node::new(C::new(-17), None);
		let node1: Node<C> = Node::new(C::new(-51), Some(Box::new(node0)));
		let node2: Node<C> = Node::new(C::new(101), Some(Box::new(node1)));

		assert_eq!(
			node2,
			Node {
				value: C::new(101),
				next: Some(Box::new(Node {
					value: C::new(-51),
					next: Some(Box::new(Node {
						value: C::new(-17),
						next: None
					}))
				}))
			}
		);
	}
	// endregion

	// region: list_new_00
	#[test]
	fn list_new_00() {
		let list: List<A> = List::new();

		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_new_01
	#[test]
	fn list_new_01() {
		let list: List<B> = List::new();

		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_new_02
	#[test]
	fn list_new_02() {
		let list: List<C> = List::new();

		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_push_front_00
	#[test]
	fn list_push_front_00() {
		let mut list: List<A> = List { head: None };

		list.push_front(A::new());

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: A::new(),
					next: None
				}))
			}
		);
	}
	// endregion

	// region: list_push_front_01
	#[test]
	fn list_push_front_01() {
		let mut list: List<B> = List { head: None };

		list.push_front(B::new(0x42));
		list.push_front(B::new(0x24));

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x24),
					next: Some(Box::new(Node {
						value: B::new(0x42),
						next: None,
					}))
				}))
			}
		);
	}
	// endregion

	// region: list_push_front_02
	#[test]
	fn list_push_front_02() {
		let mut list: List<C> = List { head: None };

		list.push_front(C::new(-3));
		list.push_front(C::new(77));
		list.push_front(C::new(-19));

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(77),
						next: Some(Box::new(Node {
							value: C::new(-3),
							next: None,
						}))
					}))
				}))
			}
		);
	}
	// endregion

	// region: list_push_back_00
	#[test]
	fn list_push_back_00() {
		let mut list: List<A> = List { head: None };

		list.push_back(A::new());

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: A::new(),
					next: None
				}))
			}
		);
	}
	// endregion

	// region: list_push_back_01
	#[test]
	fn list_push_back_01() {
		let mut list: List<B> = List { head: None };

		list.push_back(B::new(0xbe));
		list.push_back(B::new(0xaf));

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0xbe),
					next: Some(Box::new(Node {
						value: B::new(0xaf),
						next: None,
					}))
				}))
			}
		);
	}
	// endregion

	// region: list_push_back_02
	#[test]
	fn list_push_back_02() {
		let mut list: List<C> = List { head: None };

		list.push_back(C::new(-5));
		list.push_back(C::new(54));
		list.push_back(C::new(26));

		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-5),
					next: Some(Box::new(Node {
						value: C::new(54),
						next: Some(Box::new(Node {
							value: C::new(26),
							next: None,
						}))
					}))
				}))
			}
		);
	}
	// endregion

	// region: list_count_00
	#[test]
	fn list_count_00() {
		let list: List<A> = List { head: None };

		assert_eq!(list.count(), 0);
	}
	// endregion

	// region: list_count_01
	#[test]
	fn list_count_01() {
		let list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x72),
				next: Some(Box::new(Node {
					value: B::new(0x27),
					next: None,
				})),
			})),
		};

		assert_eq!(list.count(), 2);
	}
	// endregion

	// region: list_count_02
	#[test]
	fn list_count_02() {
		let list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-128),
				next: Some(Box::new(Node {
					value: C::new(127),
					next: Some(Box::new(Node {
						value: C::new(-127),
						next: Some(Box::new(Node {
							value: C::new(126),
							next: Some(Box::new(Node {
								value: C::new(-126),
								next: Some(Box::new(Node {
									value: C::new(125),
									next: Some(Box::new(Node {
										value: C::new(-125),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.count(), 7);
	}
	// endregion

	// region: list_get_00
	#[test]
	fn list_get_00() {
		let list: List<A> = List { head: None };

		assert_eq!(list.get(0), None);
	}
	// endregion

	// region: list_get_01
	#[test]
	fn list_get_01() {
		let list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x0c),
				next: Some(Box::new(Node {
					value: B::new(0x13),
					next: Some(Box::new(Node {
						value: B::new(0x1d),
						next: Some(Box::new(Node {
							value: B::new(0x27),
							next: None,
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.get(0), Some(&B::new(0x0c)));
		assert_eq!(list.get(1), Some(&B::new(0x13)));
		assert_eq!(list.get(2), Some(&B::new(0x1d)));
		assert_eq!(list.get(3), Some(&B::new(0x27)));
		assert_eq!(list.get(4), None);
	}
	// endregion

	// region: list_get_02
	#[test]
	fn list_get_02() {
		let list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-99),
				next: Some(Box::new(Node {
					value: C::new(88),
					next: Some(Box::new(Node {
						value: C::new(-77),
						next: Some(Box::new(Node {
							value: C::new(66),
							next: Some(Box::new(Node {
								value: C::new(-55),
								next: Some(Box::new(Node {
									value: C::new(44),
									next: Some(Box::new(Node {
										value: C::new(-33),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.get(0), Some(&C::new(-99)));
		assert_eq!(list.get(1), Some(&C::new(88)));
		assert_eq!(list.get(2), Some(&C::new(-77)));
		assert_eq!(list.get(3), Some(&C::new(66)));
		assert_eq!(list.get(4), Some(&C::new(-55)));
		assert_eq!(list.get(5), Some(&C::new(44)));
		assert_eq!(list.get(6), Some(&C::new(-33)));
		assert_eq!(list.get(usize::MAX), None);
	}
	// endregion

	// region: list_get_mut_00
	#[test]
	fn list_get_mut_00() {
		let mut list: List<A> = List { head: None };

		assert_eq!(list.get_mut(0), None);
	}
	// endregion

	// region: list_get_mut_01
	#[test]
	fn list_get_mut_01() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x90),
				next: Some(Box::new(Node {
					value: B::new(0x51),
					next: Some(Box::new(Node {
						value: B::new(0xc4),
						next: Some(Box::new(Node {
							value: B::new(0x23),
							next: None,
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.get_mut(3), Some(&mut B::new(0x23)));
		assert_eq!(list.get_mut(2), Some(&mut B::new(0xc4)));
		assert_eq!(list.get_mut(1), Some(&mut B::new(0x51)));
		assert_eq!(list.get_mut(0), Some(&mut B::new(0x90)));
	}
	// endregion

	// region: list_get_mut_02
	#[test]
	fn list_get_mut_02() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-1),
				next: Some(Box::new(Node {
					value: C::new(12),
					next: Some(Box::new(Node {
						value: C::new(-23),
						next: Some(Box::new(Node {
							value: C::new(34),
							next: Some(Box::new(Node {
								value: C::new(-45),
								next: Some(Box::new(Node {
									value: C::new(56),
									next: Some(Box::new(Node {
										value: C::new(-67),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.get_mut(0), Some(&mut C::new(-1)));
		assert_eq!(list.get_mut(1), Some(&mut C::new(12)));
		assert_eq!(list.get_mut(2), Some(&mut C::new(-23)));
		assert_eq!(list.get_mut(3), Some(&mut C::new(34)));
		assert_eq!(list.get_mut(4), Some(&mut C::new(-45)));
		assert_eq!(list.get_mut(5), Some(&mut C::new(56)));
		assert_eq!(list.get_mut(6), Some(&mut C::new(-67)));
	}
	// endregion

	// region: list_remove_front_00
	#[test]
	fn list_remove_front_00() {
		let mut list: List<A> = List { head: None };

		assert_eq!(list.remove_front(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_remove_front_01
	#[test]
	fn list_remove_front_01() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0xd7),
				next: Some(Box::new(Node {
					value: B::new(0x66),
					next: None,
				})),
			})),
		};

		assert_eq!(list.remove_front(), Some(B::new(0xd7)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x66),
					next: None,
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(B::new(0x66)));
		assert_eq!(list, List { head: None });
		assert_eq!(list.remove_front(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_remove_front_02
	#[test]
	fn list_remove_front_02() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-128),
				next: Some(Box::new(Node {
					value: C::new(-64),
					next: Some(Box::new(Node {
						value: C::new(32),
						next: Some(Box::new(Node {
							value: C::new(16),
							next: Some(Box::new(Node {
								value: C::new(-8),
								next: Some(Box::new(Node {
									value: C::new(-4),
									next: Some(Box::new(Node {
										value: C::new(2),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.remove_front(), Some(C::new(-128)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-64),
					next: Some(Box::new(Node {
						value: C::new(32),
						next: Some(Box::new(Node {
							value: C::new(16),
							next: Some(Box::new(Node {
								value: C::new(-8),
								next: Some(Box::new(Node {
									value: C::new(-4),
									next: Some(Box::new(Node {
										value: C::new(2),
										next: None
									})),
								})),
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(-64)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(32),
					next: Some(Box::new(Node {
						value: C::new(16),
						next: Some(Box::new(Node {
							value: C::new(-8),
							next: Some(Box::new(Node {
								value: C::new(-4),
								next: Some(Box::new(Node {
									value: C::new(2),
									next: None
								})),
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(32)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(16),
					next: Some(Box::new(Node {
						value: C::new(-8),
						next: Some(Box::new(Node {
							value: C::new(-4),
							next: Some(Box::new(Node {
								value: C::new(2),
								next: None
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(16)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-8),
					next: Some(Box::new(Node {
						value: C::new(-4),
						next: Some(Box::new(Node {
							value: C::new(2),
							next: None
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(-8)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-4),
					next: Some(Box::new(Node {
						value: C::new(2),
						next: None
					})),
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(-4)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(2),
					next: None
				})),
			}
		);
		assert_eq!(list.remove_front(), Some(C::new(2)));
		assert_eq!(list, List { head: None });
		assert_eq!(list.remove_front(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_remove_back_00
	#[test]
	fn list_remove_back_00() {
		let mut list: List<A> = List { head: None };

		assert_eq!(list.remove_back(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_remove_back_01
	#[test]
	fn list_remove_back_01() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x1a),
				next: Some(Box::new(Node {
					value: B::new(0x20),
					next: None,
				})),
			})),
		};

		assert_eq!(list.remove_back(), Some(B::new(0x20)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x1a),
					next: None,
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(B::new(0x1a)));
		assert_eq!(list, List { head: None });
		assert_eq!(list.remove_back(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_remove_back_02
	#[test]
	fn list_remove_back_02() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-91),
				next: Some(Box::new(Node {
					value: C::new(-12),
					next: Some(Box::new(Node {
						value: C::new(127),
						next: Some(Box::new(Node {
							value: C::new(-63),
							next: Some(Box::new(Node {
								value: C::new(89),
								next: Some(Box::new(Node {
									value: C::new(15),
									next: Some(Box::new(Node {
										value: C::new(-31),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list.remove_back(), Some(C::new(-31)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: Some(Box::new(Node {
						value: C::new(-12),
						next: Some(Box::new(Node {
							value: C::new(127),
							next: Some(Box::new(Node {
								value: C::new(-63),
								next: Some(Box::new(Node {
									value: C::new(89),
									next: Some(Box::new(Node {
										value: C::new(15),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(15)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: Some(Box::new(Node {
						value: C::new(-12),
						next: Some(Box::new(Node {
							value: C::new(127),
							next: Some(Box::new(Node {
								value: C::new(-63),
								next: Some(Box::new(Node {
									value: C::new(89),
									next: None,
								})),
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(89)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: Some(Box::new(Node {
						value: C::new(-12),
						next: Some(Box::new(Node {
							value: C::new(127),
							next: Some(Box::new(Node {
								value: C::new(-63),
								next: None,
							})),
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(-63)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: Some(Box::new(Node {
						value: C::new(-12),
						next: Some(Box::new(Node {
							value: C::new(127),
							next: None,
						})),
					})),
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(127)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: Some(Box::new(Node {
						value: C::new(-12),
						next: None,
					})),
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(-12)));
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-91),
					next: None,
				})),
			}
		);
		assert_eq!(list.remove_back(), Some(C::new(-91)));
		assert_eq!(list, List { head: None });
		assert_eq!(list.remove_back(), None);
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_clear_00
	#[test]
	fn list_clear_00() {
		let mut list: List<A> = List { head: None };

		list.clear();
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_clear_01
	#[test]
	fn list_clear_01() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x1a),
				next: None,
			})),
		};

		list.clear();
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_clear_02
	#[test]
	fn list_clear_02() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-7),
				next: Some(Box::new(Node {
					value: C::new(29),
					next: Some(Box::new(Node {
						value: C::new(88),
						next: Some(Box::new(Node {
							value: C::new(-14),
							next: Some(Box::new(Node {
								value: C::new(112),
								next: Some(Box::new(Node {
									value: C::new(-53),
									next: Some(Box::new(Node {
										value: C::new(-95),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		list.clear();
		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_operator_index_00
	#[test]
	fn list_operator_index_00() {
		let list: List<A> = List {
			head: Some(Box::new(Node {
				value: A::new(),
				next: None,
			})),
		};

		assert_eq!(list[0], A::new());
	}
	// endregion

	// region: list_operator_index_01
	#[test]
	fn list_operator_index_01() {
		let list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x45),
				next: Some(Box::new(Node {
					value: B::new(0xd2),
					next: None,
				})),
			})),
		};

		assert_eq!(list[0], B::new(0x45));
		assert_eq!(list[1], B::new(0xd2));
	}
	// endregion

	// region: list_operator_index_02
	#[test]
	fn list_operator_index_02() {
		let list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-100),
				next: Some(Box::new(Node {
					value: C::new(-50),
					next: Some(Box::new(Node {
						value: C::new(-25),
						next: Some(Box::new(Node {
							value: C::new(-12),
							next: Some(Box::new(Node {
								value: C::new(-6),
								next: Some(Box::new(Node {
									value: C::new(-3),
									next: Some(Box::new(Node {
										value: C::new(-1),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list[0], C::new(-100));
		assert_eq!(list[1], C::new(-50));
		assert_eq!(list[2], C::new(-25));
		assert_eq!(list[3], C::new(-12));
		assert_eq!(list[4], C::new(-6));
		assert_eq!(list[5], C::new(-3));
		assert_eq!(list[6], C::new(-1));
	}
	// endregion

	// region: list_operator_index_03
	#[test]
	#[should_panic(expected = "tried to access out of bound index 0")]
	fn list_operator_index_03() {
		let list: List<A> = List { head: None };

		assert_eq!(list[0], A::new());
	}
	// endregion

	// region: list_operator_index_04
	#[test]
	#[should_panic(expected = "tried to access out of bound index 2")]
	fn list_operator_index_04() {
		let list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x18),
				next: Some(Box::new(Node {
					value: B::new(0x7a),
					next: None,
				})),
			})),
		};

		assert_eq!(list[2], B::new(0x99));
	}
	// endregion

	// region: list_operator_index_05
	#[test]
	#[should_panic(expected = "tried to access out of bound index 18446744073709551615")]
	fn list_operator_index_05() {
		let list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-8),
				next: Some(Box::new(Node {
					value: C::new(-7),
					next: Some(Box::new(Node {
						value: C::new(-6),
						next: Some(Box::new(Node {
							value: C::new(-5),
							next: Some(Box::new(Node {
								value: C::new(-4),
								next: Some(Box::new(Node {
									value: C::new(-3),
									next: Some(Box::new(Node {
										value: C::new(-2),
										next: Some(Box::new(Node {
											value: C::new(-1),
											next: None,
										})),
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};

		assert_eq!(list[usize::MAX], C::new(0));
	}
	// endregion

	// region: list_operator_index_mut_00
	#[test]
	fn list_operator_index_mut_00() {
		let mut list: List<A> = List {
			head: Some(Box::new(Node {
				value: A::new(),
				next: None,
			})),
		};

		list[0] = A::new();
		assert_eq!(list[0], A::new());
	}
	// endregion

	// region: list_operator_index_mut_01
	#[test]
	fn list_operator_index_mut_01() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x18),
				next: Some(Box::new(Node {
					value: B::new(0x27),
					next: Some(Box::new(Node {
						value: B::new(0x9a),
						next: Some(Box::new(Node {
							value: B::new(0x3c),
							next: None,
						})),
					})),
				})),
			})),
		};

		list[0] = B::new(0x3c);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x3c),
					next: Some(Box::new(Node {
						value: B::new(0x27),
						next: Some(Box::new(Node {
							value: B::new(0x9a),
							next: Some(Box::new(Node {
								value: B::new(0x3c),
								next: None,
							})),
						})),
					})),
				}))
			}
		);
		list[1] = B::new(0x9a);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x3c),
					next: Some(Box::new(Node {
						value: B::new(0x9a),
						next: Some(Box::new(Node {
							value: B::new(0x9a),
							next: Some(Box::new(Node {
								value: B::new(0x3c),
								next: None,
							})),
						})),
					})),
				}))
			}
		);
		list[2] = B::new(0x27);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x3c),
					next: Some(Box::new(Node {
						value: B::new(0x9a),
						next: Some(Box::new(Node {
							value: B::new(0x27),
							next: Some(Box::new(Node {
								value: B::new(0x3c),
								next: None,
							})),
						})),
					})),
				}))
			}
		);
		list[3] = B::new(0x18);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: B::new(0x3c),
					next: Some(Box::new(Node {
						value: B::new(0x9a),
						next: Some(Box::new(Node {
							value: B::new(0x27),
							next: Some(Box::new(Node {
								value: B::new(0x18),
								next: None,
							})),
						})),
					})),
				}))
			}
		);
	}
	// endregion

	// region: list_operator_index_mut_02
	#[test]
	fn list_operator_index_mut_02() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-55),
				next: Some(Box::new(Node {
					value: C::new(-46),
					next: Some(Box::new(Node {
						value: C::new(-37),
						next: Some(Box::new(Node {
							value: C::new(-28),
							next: Some(Box::new(Node {
								value: C::new(-19),
								next: None,
							})),
						})),
					})),
				})),
			})),
		};

		list[0] = C::new(-19);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(-46),
						next: Some(Box::new(Node {
							value: C::new(-37),
							next: Some(Box::new(Node {
								value: C::new(-28),
								next: Some(Box::new(Node {
									value: C::new(-19),
									next: None
								})),
							})),
						})),
					})),
				}))
			}
		);
		list[1] = C::new(-28);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(-28),
						next: Some(Box::new(Node {
							value: C::new(-37),
							next: Some(Box::new(Node {
								value: C::new(-28),
								next: Some(Box::new(Node {
									value: C::new(-19),
									next: None
								})),
							})),
						})),
					})),
				}))
			}
		);
		list[2] = C::new(-37);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(-28),
						next: Some(Box::new(Node {
							value: C::new(-37),
							next: Some(Box::new(Node {
								value: C::new(-28),
								next: Some(Box::new(Node {
									value: C::new(-19),
									next: None
								})),
							})),
						})),
					})),
				}))
			}
		);
		list[3] = C::new(-46);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(-28),
						next: Some(Box::new(Node {
							value: C::new(-37),
							next: Some(Box::new(Node {
								value: C::new(-46),
								next: Some(Box::new(Node {
									value: C::new(-19),
									next: None
								})),
							})),
						})),
					})),
				}))
			}
		);
		list[4] = C::new(-55);
		assert_eq!(
			list,
			List {
				head: Some(Box::new(Node {
					value: C::new(-19),
					next: Some(Box::new(Node {
						value: C::new(-28),
						next: Some(Box::new(Node {
							value: C::new(-37),
							next: Some(Box::new(Node {
								value: C::new(-46),
								next: Some(Box::new(Node {
									value: C::new(-55),
									next: None
								})),
							})),
						})),
					})),
				}))
			}
		);
	}
	// endregion

	// region: list_operator_index_mut_03
	#[test]
	#[should_panic(expected = "tried to access out of bound index 0")]
	fn list_operator_index_mut_03() {
		let mut list: List<A> = List { head: None };

		list[0] = A::new();
	}
	// endregion

	// region: list_operator_index_mut_04
	#[test]
	#[should_panic(expected = "tried to access out of bound index 4")]
	fn list_operator_index_mut_04() {
		let mut list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x00),
				next: Some(Box::new(Node {
					value: B::new(0x3d),
					next: Some(Box::new(Node {
						value: B::new(0x21),
						next: Some(Box::new(Node {
							value: B::new(0xa7),
							next: None,
						})),
					})),
				})),
			})),
		};

		list[4] = B::new(0x42);
	}
	// endregion

	// region: list_operator_index_mut_05
	#[test]
	#[should_panic(expected = "tried to access out of bound index 18446744073709551615")]
	fn list_operator_index_mut_05() {
		let mut list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-2),
				next: Some(Box::new(Node {
					value: C::new(49),
					next: Some(Box::new(Node {
						value: C::new(28),
						next: None,
					})),
				})),
			})),
		};

		list[usize::MAX] = C::new(-42);
	}
	// endregion

	// region: list_clone_00
	#[test]
	fn list_clone_00() {
		let list: List<A> = List {
			head: Some(Box::new(Node {
				value: A::new(),
				next: None,
			})),
		};
		let cloned: List<A> = list.clone();

		assert_eq!(list, cloned);
	}
	// endregion

	// region: list_clone_01
	#[test]
	fn list_clone_01() {
		let list: List<B> = List {
			head: Some(Box::new(Node {
				value: B::new(0x7d),
				next: Some(Box::new(Node {
					value: B::new(0x11),
					next: Some(Box::new(Node {
						value: B::new(0x3a),
						next: None,
					})),
				})),
			})),
		};
		let cloned: List<B> = list.clone();

		assert_eq!(list, cloned);
	}
	// endregion

	// region: list_clone_02
	#[test]
	fn list_clone_02() {
		let list: List<C> = List {
			head: Some(Box::new(Node {
				value: C::new(-128),
				next: Some(Box::new(Node {
					value: C::new(64),
					next: Some(Box::new(Node {
						value: C::new(32),
						next: Some(Box::new(Node {
							value: C::new(-16),
							next: Some(Box::new(Node {
								value: C::new(-8),
								next: Some(Box::new(Node {
									value: C::new(4),
									next: Some(Box::new(Node {
										value: C::new(2),
										next: None,
									})),
								})),
							})),
						})),
					})),
				})),
			})),
		};
		let cloned: List<C> = list.clone();

		assert_eq!(list, cloned);
	}
	// endregion

	// region: list_default_00
	#[test]
	fn list_default_00() {
		let list: List<A> = List::default();

		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_default_01
	#[test]
	fn list_default_01() {
		let list: List<B> = List::default();

		assert_eq!(list, List { head: None });
	}
	// endregion

	// region: list_default_02
	#[test]
	fn list_default_02() {
		let list: List<C> = List::default();

		assert_eq!(list, List { head: None });
	}
	// endregion
}
