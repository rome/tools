use crate::api::RawLanguage;
use crate::green::GreenElement;
use crate::{ast_shape, GreenNode, Language, SyntaxKind};

pub trait AstTreeShape: Language {
	/// Verifies if the `children` fit the expected shape of `kind` node.
	fn forms_exact_shape_for(
		parent: Self::Kind,
		slots: impl ExactSizeIterator<Item = Option<Self::Kind>>,
	) -> bool;

	#[inline]
	fn forms_node_list_shape<I, N>(can_cast: N, mut slots: I) -> bool
	where
		N: Fn(Self::Kind) -> bool,
		I: Iterator<Item = Option<Self::Kind>>,
	{
		slots.all(|slot| slot.is_some() && can_cast(slot.unwrap()))
	}

	#[inline]
	fn forms_separated_list_shape<I, N>(
		can_cast: N,
		separator: Self::Kind,
		allow_trailing: bool,
		slots: I,
	) -> bool
	where
		N: Fn(Self::Kind) -> bool,
		I: ExactSizeIterator<Item = Option<Self::Kind>>,
	{
		let slots_len = slots.len();
		slots.enumerate().all(|(index, slot)| {
			let is_node = index % 2 == 0;

			if is_node {
				slot.is_none() || can_cast(slot.unwrap())
			} else if index == slots_len - 1 {
				allow_trailing
			} else {
				slot.is_none() || slot.unwrap() == separator
			}
		})
	}
}

impl AstTreeShape for RawLanguage {
	fn forms_exact_shape_for(
		_: Self::Kind,
		_: impl ExactSizeIterator<Item = Option<Self::Kind>>,
	) -> bool {
		true
	}
}
