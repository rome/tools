//! Sorted Linked List

use std::{cell::Cell, cmp::Ordering, ptr};

use crate::utility_types::Delta;
/// # Safety
///
/// This trait should be safe to call
pub(crate) unsafe trait Elem {
	fn prev(&self) -> &Cell<*const Self>;
	fn next(&self) -> &Cell<*const Self>;
	fn key(&self) -> &Cell<u32>;
}

pub(crate) enum AddToSllResult<'a, E: Elem> {
	NoHead,
	EmptyHead(&'a Cell<*const E>),
	SmallerThanHead(&'a Cell<*const E>),
	SmallerThanNotHead(*const E),
	AlreadyInSll(*const E),
}

impl<'a, E: Elem> AddToSllResult<'a, E> {
	pub(crate) fn add_to_sll(&self, elem_ptr: *const E) {
		unsafe {
			(*elem_ptr).prev().set(elem_ptr);
			(*elem_ptr).next().set(elem_ptr);

			match self {
				// Case 1: empty head, replace it.
				AddToSllResult::EmptyHead(head) => head.set(elem_ptr),

				// Case 2: we are smaller than the head, replace it.
				AddToSllResult::SmallerThanHead(head) => {
					let old_head = head.get();
					let prev = (*old_head).prev().replace(elem_ptr);
					(*prev).next().set(elem_ptr);
					(*elem_ptr).next().set(old_head);
					(*elem_ptr).prev().set(prev);
					head.set(elem_ptr);
				}

				// Case 3: insert in place found by looping
				AddToSllResult::SmallerThanNotHead(curr) => {
					let next = (**curr).next().replace(elem_ptr);
					(*next).prev().set(elem_ptr);
					(*elem_ptr).prev().set(*curr);
					(*elem_ptr).next().set(next);
				}
				AddToSllResult::NoHead | AddToSllResult::AlreadyInSll(_) => (),
			}
		}
	}
}

#[cold]
pub(crate) fn init<'a, E: Elem>(
	head: Option<&'a Cell<*const E>>,
	elem: &E,
) -> AddToSllResult<'a, E> {
	if let Some(head) = head {
		link(head, elem)
	} else {
		AddToSllResult::NoHead
	}
}

#[cold]
pub(crate) fn unlink<E: Elem>(head: &Cell<*const E>, elem: &E) {
	debug_assert!(!head.get().is_null(), "invalid linked list head");

	let elem_ptr: *const E = elem;

	let prev = elem.prev().replace(elem_ptr);
	let next = elem.next().replace(elem_ptr);
	unsafe {
		debug_assert_eq!((*prev).next().get(), elem_ptr, "invalid linked list links");
		debug_assert_eq!((*next).prev().get(), elem_ptr, "invalid linked list links");
		(*prev).next().set(next);
		(*next).prev().set(prev);
	}

	if head.get() == elem_ptr {
		head.set(if next == elem_ptr { ptr::null() } else { next })
	}
}

#[cold]
pub(crate) fn link<'a, E: Elem>(head: &'a Cell<*const E>, elem: &E) -> AddToSllResult<'a, E> {
	unsafe {
		let old_head = head.get();
		// Case 1: empty head, replace it.
		if old_head.is_null() {
			return AddToSllResult::EmptyHead(head);
		}

		// Case 2: we are smaller than the head, replace it.
		if elem.key() < (*old_head).key() {
			return AddToSllResult::SmallerThanHead(head);
		}

		// Case 3: loop *backward* until we find insertion place. Because of
		// Case 2, we can't loop beyond the head.
		let mut curr = (*old_head).prev().get();
		loop {
			match (*curr).key().cmp(elem.key()) {
				Ordering::Less => return AddToSllResult::SmallerThanNotHead(curr),
				Ordering::Equal => return AddToSllResult::AlreadyInSll(curr),
				Ordering::Greater => curr = (*curr).prev().get(),
			}
		}
	}
}

pub(crate) fn adjust<E: Elem>(elem: &E, from: u32, by: Delta<u32>) {
	let elem_ptr: *const E = elem;

	unsafe {
		let mut curr = elem_ptr;
		loop {
			let mut key = (*curr).key().get();
			if key >= from {
				key += by;
				(*curr).key().set(key);
			}
			curr = (*curr).next().get();
			if curr == elem_ptr {
				break;
			}
		}
	}
}
