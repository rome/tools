use crate::api::RawLanguage;
use crate::Language;

pub trait AstTreeShape: Language {
	/// Verifies if the `children` fit the expected shape of `kind` node.
	fn fits_shape_of(
		kind: &Self::Kind,
		children_len: usize,
		children_kinds: impl Iterator<Item = Option<Self::Kind>>,
	) -> bool;

	#[inline]
	fn fits_list_shape<I, N>(can_cast: N, actual: I) -> bool
	where
		N: Fn(Self::Kind) -> bool,
		I: Iterator<Item = Option<Self::Kind>>,
	{
		for actual_child in actual {
			if actual_child.is_some() && !can_cast(actual_child.unwrap()) {
				return false;
			}
		}

		true
	}

	#[inline]
	fn fits_separated_list_shape<I, N>(
		can_cast: N,
		separator: Self::Kind,
		allow_trailing: bool,
		actual: I,
	) -> bool
	where
		N: Fn(Self::Kind) -> bool,
		I: Iterator<Item = Option<Self::Kind>>,
	{
		// Is the next expected element a separator?
		let mut next_element = true;
		let mut empty = true;

		for actual_child in actual {
			if next_element {
				if actual_child.is_some() && !can_cast(actual_child.unwrap()) {
					return false;
				}

				next_element = false;
			} else {
				if actual_child.is_some() && actual_child.unwrap() != separator {
					return false;
				}

				next_element = true;
			}
			empty = false;
		}

		// OK if the list allows trailing separator or is empty or if the last element wasn't a trailing separator
		allow_trailing || empty || !next_element
	}
}

impl AstTreeShape for RawLanguage {
	fn fits_shape_of(
		_: &Self::Kind,
		_: usize,
		_: impl Iterator<Item = Option<Self::Kind>>,
	) -> bool {
		true
	}
}
