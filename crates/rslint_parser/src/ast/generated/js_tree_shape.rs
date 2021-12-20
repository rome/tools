//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{ast::*, JsSyntaxKind::*, T};
use rome_rowan::{
	ParsedChildren, RawSyntaxElement, RawSyntaxNode, SyntaxFactory, SyntaxKind,
	UnknownNodeChildrenIterator,
};
#[derive(Debug)]
pub struct JsSyntaxFactory;
impl SyntaxFactory for JsSyntaxFactory {
	type Kind = JsSyntaxKind;
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[warn(unused_variables)]
	fn make_syntax(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
	) -> RawSyntaxNode<Self::Kind> {
		let actual_len = children.len();
		match kind {
			JS_UNKNOWN
			| JS_UNKNOWN_ASSIGNMENT
			| JS_UNKNOWN_BINDING
			| JS_UNKNOWN_EXPRESSION
			| JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
			| JS_UNKNOWN_MEMBER
			| JS_UNKNOWN_MODIFIER
			| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
			| JS_UNKNOWN_STATEMENT
			| ERROR => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
			CALL_EXPR => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCallArguments::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_DECL => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExportDeclaration::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_DEFAULT_DECL => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if DefaultDecl::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_DEFAULT_EXPR => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_NAMED => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if ExportNamedSpecifierList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_WILDCARD => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			FOR_STMT => {
				if actual_len > 9usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyForInitializer::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if ForStmtTest::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if ForStmtUpdate::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			FOR_STMT_TEST => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			FOR_STMT_UPDATE => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			IDENT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			IMPORT_META => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![meta] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsArrayAssignmentPatternElementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_BINDING_PATTERN => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsArrayBindingPatternElementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsArrayElementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARRAY_HOLE => {
				if actual_len > 0usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 0usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ARROW_FUNCTION_EXPRESSION => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyArrowFunctionParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyArrowFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ASSIGNMENT_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						T ! [=]
							| T ! [+=] | T ! [-=] | T ! [*=]
							| T ! [/=] | T ! [%=] | T ! [**=]
							| T ! [>>=] | T ! [<<=] | T ! [>>>=]
							| T ! [&=] | T ! [|=] | T ! [^=]
							| T ! [&&=] | T ! [||=] | T ! [??=]
					) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ASSIGNMENT_WITH_DEFAULT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_AWAIT_EXPRESSION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![await] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_BIG_INT_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BINARY_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						T ! [<]
							| T ! [>] | T ! [<=] | T ! [>=]
							| T ! [==] | T ! [===] | T ! [!=]
							| T ! [!==] | T ! [+] | T ! [-]
							| T ! [*] | T ! [/] | T ! [%]
							| T ! [**] | T ! [<<] | T ! [>>]
							| T ! [>>>] | T ! [&] | T ! [|]
							| T ! [^] | T![in] | T![instanceof]
					) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BLOCK_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![true] | T![false]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_BREAK_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![break] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CALL_ARGUMENTS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCallArgumentList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CASE_CLAUSE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![case] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CATCH_CLAUSE => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![catch] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCatchDeclaration::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CATCH_DECLARATION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CLASS_DECLARATION => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CLASS_EXPRESSION => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_COMPUTED_MEMBER_NAME => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CONDITIONAL_EXPRESSION => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsLiteralMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsConstructorParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CONSTRUCTOR_PARAMETERS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsConstructorParameterList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_CONTINUE_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![continue] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_DEBUGGER_STATEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![debugger] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_DEFAULT_CLAUSE => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_DEFAULT_IMPORT_SPECIFIER => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [,] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_DIRECTIVE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_DO_WHILE_STATEMENT => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![do] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![while] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_ELSE_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![else] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_EMPTY_CLASS_MEMBER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_EMPTY_STATEMENT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_EXPRESSION_SNIPPED => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_EXPRESSION_STATEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_EXTENDS_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FINALLY_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![finally] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FOR_IN_STATEMENT => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyForInOrOfInitializer::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![in] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FOR_OF_STATEMENT => {
				if actual_len > 8usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![await] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyForInOrOfInitializer::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![of] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FOR_VARIABLE_DECLARATION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![var] | T![let] | T![const]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsVariableDeclaration::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FUNCTION_BODY => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FUNCTION_DECLARATION => {
				if actual_len > 8usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_FUNCTION_EXPRESSION => {
				if actual_len > 8usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_GETTER_CLASS_MEMBER => {
				if actual_len > 9usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![get] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_GETTER_OBJECT_MEMBER => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![get] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IDENTIFIER_ASSIGNMENT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IDENTIFIER_BINDING => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IDENTIFIER_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IF_STATEMENT => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![if] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsElseClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if AnyJsImportClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_ASSERTION => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![assert] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsImportAssertionEntryList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_ASSERTION_ENTRY => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), IDENT | JS_STRING_LITERAL) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_BARE_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_CALL_EXPRESSION => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_NAMED_CLAUSE => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsDefaultImportSpecifier::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyNamedImport::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_INITIALIZER_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_LABELED_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_LITERAL_EXPORT_NAME => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), IDENT | JS_STRING_LITERAL) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_LITERAL_MEMBER_NAME => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						IDENT | JS_STRING_LITERAL | JS_NUMBER_LITERAL
					) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_LOGICAL_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [??] | T ! [||] | T ! [&&]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_METHOD_CLASS_MEMBER => {
				if actual_len > 10usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 10usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_METHOD_OBJECT_MEMBER => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_MODULE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_SHEBANG {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsModuleItemList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_MODULE_SOURCE => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NAME => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsLiteralExportName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NAMED_IMPORT_SPECIFIERS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsNamedImportSpecifierList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NULL_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![null] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_NUMBER_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsObjectAssignmentPatternPropertyList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_BINDING_PATTERN => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsObjectBindingPatternPropertyList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_OBJECT_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsObjectMemberList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PARAMETERS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameterList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PARENTHESIZED_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_POST_UPDATE_EXPRESSION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [++] | T ! [--]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PRE_UPDATE_EXPRESSION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [++] | T ! [--]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [#] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PRIVATE_NAME => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [#] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PROPERTY_CLASS_MEMBER => {
				if actual_len > 11usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 11usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![declare] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_REFERENCE_IDENTIFIER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_REGEX_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_REST_PARAMETER => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_RETURN_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![return] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SCRIPT => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_SHEBANG {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SEQUENCE_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [,] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SETTER_CLASS_MEMBER => {
				if actual_len > 9usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![set] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SETTER_OBJECT_MEMBER => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![set] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SPREAD => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [.] | T ! [?.]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_STRING_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SUPER_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![super] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_SWITCH_STATEMENT => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![switch] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsSwitchCaseList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_THIS_EXPRESSION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![this] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_THROW_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![throw] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_TRY_FINALLY_STATEMENT => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![try] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCatchClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsFinallyClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_TRY_STATEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![try] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCatchClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_UNARY_EXPRESSION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						T![delete] | T![void] | T![typeof] | T ! [+] | T ! [-] | T ! [~] | T![!]
					) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_VARIABLE_DECLARATION => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_VARIABLE_DECLARATIONS => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![var] | T![const] | T![let]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsVariableDeclarationList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_VARIABLE_STATEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsVariableDeclarations::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_WHILE_STATEMENT => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![while] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_WITH_STATEMENT => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![with] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			JS_YIELD_EXPRESSION => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![yield] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			NEW_EXPR => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsCallArguments::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			NEW_TARGET => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![target] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			SPECIFIER => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TEMPLATE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['`'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TemplateElementList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['`'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TEMPLATE_CHUNK_ELEMENT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == TEMPLATE_CHUNK {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TEMPLATE_ELEMENT => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == DOLLAR_CURLY {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_ANY => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![any] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_ARRAY => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_ASSERTION => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_BIGINT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_BOOLEAN => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CALL_SIGNATURE_DECL => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONDITIONAL_TYPE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsExtends::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONST_ASSERTION => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![const] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONSTRAINT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONSTRUCTOR_PARAM => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_CONSTRUCTOR_TYPE => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_DEFAULT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_ENUM => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![const] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![enum] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsEnumMemberList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_ENUM_MEMBER => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_EXPORT_ASSIGNMENT => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_EXPR_WITH_TYPE_ARGS => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_EXTENDS => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_EXTERNAL_MODULE_REF => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![require] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_FN_TYPE => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_IMPLEMENTS_CLAUSE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![implements] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_IMPORT => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_IMPORT_EQUALS_DECL => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsModuleRef::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_INDEX_SIGNATURE => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_INDEXED_ARRAY => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_INFER => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![infer] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_INTERFACE_DECL => {
				if actual_len > 8usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![declare] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![interface] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsExprWithTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeElement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_INTERSECTION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_LITERAL => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_MAPPED_TYPE => {
				if actual_len > 10usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 10usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsMappedTypeReadonly::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [-] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [+] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsMappedTypeParam::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_MAPPED_TYPE_PARAM => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_MAPPED_TYPE_READONLY => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [-] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [+] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_METHOD_SIGNATURE => {
				if actual_len > 7usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_MODULE_BLOCK => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_MODULE_DECL => {
				if actual_len > 6usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![declare] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![global] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![module] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsNamespaceBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NAMESPACE_DECL => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![declare] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsNamespaceBody::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NAMESPACE_EXPORT_DECL => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![namespace] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NEVER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![never] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NON_NULL => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NULL => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![null] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_NUMBER => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_OBJECT => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_OBJECT_TYPE => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsObjectMemberList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_PAREN => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_PREDICATE => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsThisOrMore::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_PROPERTY_SIGNATURE => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_QUALIFIED_PATH => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_STRING => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_SYMBOL => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TEMPLATE => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTemplateElement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TEMPLATE_ELEMENT => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_THIS => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![this] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TUPLE => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTupleElement::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TUPLE_ELEMENT => {
				if actual_len > 5usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_ALIAS_DECL => {
				if actual_len > 4usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_ANNOTATION => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_ARGS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_NAME => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_OPERATOR => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_PARAM => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsConstraint::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsDefault::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_PARAMS => {
				if actual_len > 3usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeParam::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_TYPE_REF => {
				if actual_len > 2usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_UNDEFINED => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![undefined] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_UNION => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_UNKNOWN => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![unknown] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			TS_VOID => {
				if actual_len > 1usize {
					return RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some));
				}
				let mut elements = children.into_iter();
				let mut current_slot_index = 0;
				let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] =
					Default::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![void] {
						slots[current_slot_index] = current_element.take();
						current_slot_index += 1;
						current_element = elements.next();
					} else {
						slots[current_slot_index] = None;
						current_slot_index += 1;
					}
				} else {
					slots[current_slot_index] = None;
					current_slot_index += 1;
				}
				if let Some(element) = current_element {
					return RawSyntaxNode::new(
						kind.to_unknown(),
						UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
					);
				}
				RawSyntaxNode::new(kind, slots)
			}
			EXPORT_NAMED_SPECIFIER_LIST => {
				Self::make_separated_list_syntax(kind, children, Specifier::can_cast, T ! [,], true)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyArrayAssignmentPatternElement::can_cast,
				T ! [,],
				true,
			),
			JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyArrayBindingPatternElement::can_cast,
				T ! [,],
				true,
			),
			JS_ARRAY_ELEMENT_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyArrayElement::can_cast,
				T ! [,],
				true,
			),
			JS_CALL_ARGUMENT_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyExpression::can_cast,
				T ! [,],
				true,
			),
			JS_CLASS_MEMBER_LIST => {
				Self::make_node_list_syntax(kind, children, JsAnyClassMember::can_cast)
			}
			JS_CONSTRUCTOR_PARAMETER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyConstructorParameter::can_cast,
				T ! [,],
				true,
			),
			JS_DIRECTIVE_LIST => Self::make_node_list_syntax(kind, children, JsDirective::can_cast),
			JS_IMPORT_ASSERTION_ENTRY_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyImportAssertionEntry::can_cast,
				T ! [,],
				true,
			),
			JS_MODULE_ITEM_LIST => {
				Self::make_node_list_syntax(kind, children, JsAnyModuleItem::can_cast)
			}
			JS_NAMED_IMPORT_SPECIFIER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyNamedImportSpecifier::can_cast,
				T ! [,],
				true,
			),
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyObjectAssignmentPatternMember::can_cast,
				T ! [,],
				true,
			),
			JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyObjectBindingPatternMember::can_cast,
				T ! [,],
				true,
			),
			JS_OBJECT_MEMBER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyObjectMember::can_cast,
				T ! [,],
				true,
			),
			JS_PARAMETER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyParameter::can_cast,
				T ! [,],
				true,
			),
			JS_STATEMENT_LIST => {
				Self::make_node_list_syntax(kind, children, JsAnyStatement::can_cast)
			}
			JS_SWITCH_CASE_LIST => {
				Self::make_node_list_syntax(kind, children, JsAnySwitchClause::can_cast)
			}
			JS_VARIABLE_DECLARATION_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsVariableDeclaration::can_cast,
				T ! [,],
				false,
			),
			TEMPLATE_ELEMENT_LIST => {
				Self::make_node_list_syntax(kind, children, AnyTemplateElement::can_cast)
			}
			TS_ENUM_MEMBER_LIST => {
				Self::make_node_list_syntax(kind, children, TsEnumMember::can_cast)
			}
			TS_OBJECT_MEMBER_LIST => {
				Self::make_node_list_syntax(kind, children, TsTypeElement::can_cast)
			}
			TS_TYPE_ARG_LIST => {
				Self::make_separated_list_syntax(kind, children, TsType::can_cast, T ! [,], false)
			}
			TS_TYPE_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				TsExprWithTypeArgs::can_cast,
				T ! [,],
				false,
			),
			TS_TYPE_PARAM_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				TsTypeParam::can_cast,
				T ! [,],
				false,
			),
			_ => unreachable!("Is {:?} a token?", kind),
		}
	}
}
