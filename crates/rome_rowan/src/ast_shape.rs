use crate::api::RawLanguage;
use crate::green::GreenElement;
use crate::Language;
use std::marker::PhantomData;

pub enum NodeShape<'a, L: Language> {
	List(ParsedElements<'a, L>),
	Normal {
		parsed_elements: ParsedElements<'a, L>,
		commands: &'a [NodeShapeCommand],
	},
}

pub trait AstTreeShape: Language {
	/// Verifies if the `children` fit the expected shape of `kind` node.
	fn forms_exact_shape_for<F, R>(
		parent: Self::Kind,
		slots: ParsedElements<Self>,
		receive: F,
	) -> R
	where
		F: FnOnce(Result<NodeShape<'_, Self>, ParsedElements<'_, Self>>) -> R;

	#[inline]
	fn forms_node_list_shape<I, N>(can_cast: N, mut slots: I) -> bool
	where
		N: Fn(Self::Kind) -> bool,
		I: Iterator<Item = Self::Kind>,
	{
		slots.all(can_cast)
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
		I: ExactSizeIterator<Item = Self::Kind>,
	{
		let slots_len = slots.len();
		slots.enumerate().all(|(index, slot)| {
			let is_node = index % 2 == 0;

			if is_node {
				can_cast(slot)
			} else if index == slots_len - 1 {
				allow_trailing && slot == separator
			} else {
				slot == separator
			}
		})
	}
}

pub struct ParsedElements<'a, L> {
	children: &'a mut Vec<(u64, GreenElement)>,
	first_child: usize,
	ph: PhantomData<L>,
}

impl<'a, L: Language> ParsedElements<'a, L> {
	pub(crate) fn new(elements: &'a mut Vec<(u64, GreenElement)>, first_child: usize) -> Self {
		Self {
			children: elements,
			first_child,
			ph: PhantomData,
		}
	}

	pub fn kinds<'b>(&'b self) -> impl ExactSizeIterator<Item = L::Kind> + 'b {
		self.children[self.first_child..]
			.iter()
			.map(|(_, element)| L::kind_from_raw(element.kind()))
	}

	pub fn len(&self) -> usize {
		(self.first_child..self.children.len()).len()
	}

	pub(crate) fn elements(self) -> std::vec::Drain<'a, (u64, GreenElement)> {
		self.children.drain(self.first_child..)
	}
}

#[derive(Copy, Clone)]
pub enum NodeShapeCommand {
	Occupied,
	Empty,
}

pub struct NodeShapCommands<const COUNT: usize> {
	commands: [NodeShapeCommand; COUNT],
	current_slot: usize,
}

impl<const COUNT: usize> Default for NodeShapCommands<COUNT> {
	fn default() -> Self {
		Self {
			commands: [NodeShapeCommand::Occupied; COUNT],
			current_slot: 0,
		}
	}
}

impl<const COUNT: usize> NodeShapCommands<COUNT> {
	pub fn empty(&mut self) {
		self.commands[self.current_slot] = NodeShapeCommand::Empty;
		self.current_slot += 1;
	}

	pub fn occupied(&mut self) {
		self.commands[self.current_slot] = NodeShapeCommand::Occupied;
		self.current_slot += 1;
	}

	pub fn as_slice(&self) -> &[NodeShapeCommand] {
		self.commands.as_slice()
	}
}

impl AstTreeShape for RawLanguage {
	fn forms_exact_shape_for<F, R>(_: Self::Kind, slots: ParsedElements<Self>, receive: F) -> R
	where
		F: FnOnce(Result<NodeShape<'_, Self>, ParsedElements<'_, Self>>) -> R,
	{
		receive(Ok(NodeShape::List(slots)))
	}
}
