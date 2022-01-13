//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{ast::*, JsSyntaxKind::*, T};
use rome_rowan::{ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};
#[derive(Debug)]
pub struct JsSyntaxFactory;
impl SyntaxFactory for JsSyntaxFactory {
	type Kind = JsSyntaxKind;
	#[allow(unused_mut)]
	fn make_syntax(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
	) -> RawSyntaxNode<Self::Kind> {
		match kind {
			JS_UNKNOWN
			| JS_UNKNOWN_ASSIGNMENT
			| JS_UNKNOWN_BINDING
			| JS_UNKNOWN_EXPRESSION
			| JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
			| JS_UNKNOWN_MEMBER
			| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
			| JS_UNKNOWN_PARAMETER
			| JS_UNKNOWN_STATEMENT => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
			IDENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(IDENT.to_unknown(), children.into_iter().map(Some));
				}
				slots.into_node(IDENT, children)
			}
			IMPORT_META => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![meta] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						IMPORT_META.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(IMPORT_META, children)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsArrayAssignmentPatternElementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_ASSIGNMENT_PATTERN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_ASSIGNMENT_PATTERN, children)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT, children)
			}
			JS_ARRAY_BINDING_PATTERN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsArrayBindingPatternElementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_BINDING_PATTERN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_BINDING_PATTERN, children)
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_BINDING_PATTERN_REST_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_BINDING_PATTERN_REST_ELEMENT, children)
			}
			JS_ARRAY_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsArrayElementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_EXPRESSION, children)
			}
			JS_ARRAY_HOLE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<0usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARRAY_HOLE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARRAY_HOLE, children)
			}
			JS_ARROW_FUNCTION_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyArrowFunctionParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ARROW_FUNCTION_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ARROW_FUNCTION_EXPRESSION, children)
			}
			JS_ASSIGNMENT_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
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
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ASSIGNMENT_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ASSIGNMENT_EXPRESSION, children)
			}
			JS_ASSIGNMENT_WITH_DEFAULT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ASSIGNMENT_WITH_DEFAULT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ASSIGNMENT_WITH_DEFAULT, children)
			}
			JS_AWAIT_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![await] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_AWAIT_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_AWAIT_EXPRESSION, children)
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_BIG_INT_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BIG_INT_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BIG_INT_LITERAL_EXPRESSION, children)
			}
			JS_BINARY_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
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
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BINARY_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BINARY_EXPRESSION, children)
			}
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BINDING_PATTERN_WITH_DEFAULT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BINDING_PATTERN_WITH_DEFAULT, children)
			}
			JS_BLOCK_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BLOCK_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BLOCK_STATEMENT, children)
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![true] | T![false]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BOOLEAN_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BOOLEAN_LITERAL_EXPRESSION, children)
			}
			JS_BREAK_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![break] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_BREAK_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_BREAK_STATEMENT, children)
			}
			JS_CALL_ARGUMENTS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCallArgumentList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CALL_ARGUMENTS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CALL_ARGUMENTS, children)
			}
			JS_CALL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCallArguments::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CALL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CALL_EXPRESSION, children)
			}
			JS_CASE_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![case] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CASE_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CASE_CLAUSE, children)
			}
			JS_CATCH_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![catch] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCatchDeclaration::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CATCH_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CATCH_CLAUSE, children)
			}
			JS_CATCH_DECLARATION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CATCH_DECLARATION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CATCH_DECLARATION, children)
			}
			JS_CLASS_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CLASS_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CLASS_EXPRESSION, children)
			}
			JS_CLASS_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CLASS_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CLASS_STATEMENT, children)
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_COMPUTED_MEMBER_ASSIGNMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_COMPUTED_MEMBER_ASSIGNMENT, children)
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_COMPUTED_MEMBER_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_COMPUTED_MEMBER_EXPRESSION, children)
			}
			JS_COMPUTED_MEMBER_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_COMPUTED_MEMBER_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_COMPUTED_MEMBER_NAME, children)
			}
			JS_CONDITIONAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CONDITIONAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CONDITIONAL_EXPRESSION, children)
			}
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsLiteralMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsConstructorParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CONSTRUCTOR_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CONSTRUCTOR_CLASS_MEMBER, children)
			}
			JS_CONSTRUCTOR_PARAMETERS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsConstructorParameterList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CONSTRUCTOR_PARAMETERS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CONSTRUCTOR_PARAMETERS, children)
			}
			JS_CONTINUE_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![continue] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_CONTINUE_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_CONTINUE_STATEMENT, children)
			}
			JS_DEBUGGER_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![debugger] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_DEBUGGER_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_DEBUGGER_STATEMENT, children)
			}
			JS_DEFAULT_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_DEFAULT_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_DEFAULT_CLAUSE, children)
			}
			JS_DEFAULT_IMPORT_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [,] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_DEFAULT_IMPORT_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_DEFAULT_IMPORT_SPECIFIER, children)
			}
			JS_DIRECTIVE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_DIRECTIVE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_DIRECTIVE, children)
			}
			JS_DO_WHILE_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![do] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![while] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_DO_WHILE_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_DO_WHILE_STATEMENT, children)
			}
			JS_ELSE_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![else] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_ELSE_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_ELSE_CLAUSE, children)
			}
			JS_EMPTY_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EMPTY_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EMPTY_CLASS_MEMBER, children)
			}
			JS_EMPTY_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EMPTY_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EMPTY_STATEMENT, children)
			}
			JS_EXPORT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExportClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT, children)
			}
			JS_EXPORT_AS_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsLiteralExportName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_AS_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_AS_CLAUSE, children)
			}
			JS_EXPORT_CLASS_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_CLASS_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_CLASS_CLAUSE, children)
			}
			JS_EXPORT_DEFAULT_CLASS_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![class] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExtendsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsImplementsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsClassMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_DEFAULT_CLASS_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_DEFAULT_CLASS_CLAUSE, children)
			}
			JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE, children)
			}
			JS_EXPORT_DEFAULT_FUNCTION_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![default] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_DEFAULT_FUNCTION_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_DEFAULT_FUNCTION_CLAUSE, children)
			}
			JS_EXPORT_FROM_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExportAsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_FROM_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_FROM_CLAUSE, children)
			}
			JS_EXPORT_FUNCTION_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_FUNCTION_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_FUNCTION_CLAUSE, children)
			}
			JS_EXPORT_NAMED_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExportNamedSpecifierList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_NAMED_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_NAMED_CLAUSE, children)
			}
			JS_EXPORT_NAMED_FROM_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExportNamedFromSpecifierList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_NAMED_FROM_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_NAMED_FROM_CLAUSE, children)
			}
			JS_EXPORT_NAMED_FROM_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsLiteralExportName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsExportAsClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_NAMED_FROM_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_NAMED_FROM_SPECIFIER, children)
			}
			JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_NAMED_SHORTHAND_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_NAMED_SHORTHAND_SPECIFIER, children)
			}
			JS_EXPORT_NAMED_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![type] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsLiteralExportName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_NAMED_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_NAMED_SPECIFIER, children)
			}
			JS_EXPORT_VARIABLE_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsVariableDeclarations::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPORT_VARIABLE_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPORT_VARIABLE_CLAUSE, children)
			}
			JS_EXPRESSION_SNIPPED => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPRESSION_SNIPPED.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPRESSION_SNIPPED, children)
			}
			JS_EXPRESSION_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXPRESSION_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXPRESSION_STATEMENT, children)
			}
			JS_EXTENDS_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_EXTENDS_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_EXTENDS_CLAUSE, children)
			}
			JS_FINALLY_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![finally] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FINALLY_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FINALLY_CLAUSE, children)
			}
			JS_FOR_IN_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyForInOrOfInitializer::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![in] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FOR_IN_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FOR_IN_STATEMENT, children)
			}
			JS_FOR_OF_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![await] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyForInOrOfInitializer::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![of] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FOR_OF_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FOR_OF_STATEMENT, children)
			}
			JS_FOR_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![for] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyForInitializer::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FOR_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FOR_STATEMENT, children)
			}
			JS_FOR_VARIABLE_DECLARATION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![var] | T![let] | T![const]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsVariableDeclaration::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FOR_VARIABLE_DECLARATION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FOR_VARIABLE_DECLARATION, children)
			}
			JS_FUNCTION_BODY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FUNCTION_BODY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FUNCTION_BODY, children)
			}
			JS_FUNCTION_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FUNCTION_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FUNCTION_EXPRESSION, children)
			}
			JS_FUNCTION_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![function] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_FUNCTION_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_FUNCTION_STATEMENT, children)
			}
			JS_GETTER_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![get] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_GETTER_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_GETTER_CLASS_MEMBER, children)
			}
			JS_GETTER_OBJECT_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![get] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_GETTER_OBJECT_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_GETTER_OBJECT_MEMBER, children)
			}
			JS_IDENTIFIER_ASSIGNMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IDENTIFIER_ASSIGNMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IDENTIFIER_ASSIGNMENT, children)
			}
			JS_IDENTIFIER_BINDING => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IDENTIFIER_BINDING.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IDENTIFIER_BINDING, children)
			}
			JS_IDENTIFIER_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IDENTIFIER_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IDENTIFIER_EXPRESSION, children)
			}
			JS_IF_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![if] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsElseClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IF_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IF_STATEMENT, children)
			}
			JS_IMPORT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if AnyJsImportClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT, children)
			}
			JS_IMPORT_ASSERTION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![assert] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertionEntryList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_ASSERTION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_ASSERTION, children)
			}
			JS_IMPORT_ASSERTION_ENTRY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), IDENT | JS_STRING_LITERAL) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_ASSERTION_ENTRY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_ASSERTION_ENTRY, children)
			}
			JS_IMPORT_BARE_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_BARE_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_BARE_CLAUSE, children)
			}
			JS_IMPORT_CALL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_CALL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_CALL_EXPRESSION, children)
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_DEFAULT_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_DEFAULT_CLAUSE, children)
			}
			JS_IMPORT_NAMED_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsDefaultImportSpecifier::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyNamedImport::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_NAMED_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_NAMED_CLAUSE, children)
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![from] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleSource::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsImportAssertion::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_IMPORT_NAMESPACE_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_IMPORT_NAMESPACE_CLAUSE, children)
			}
			JS_INITIALIZER_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_INITIALIZER_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_INITIALIZER_CLAUSE, children)
			}
			JS_LABELED_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_LABELED_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_LABELED_STATEMENT, children)
			}
			JS_LITERAL_EXPORT_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), IDENT | JS_STRING_LITERAL) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_LITERAL_EXPORT_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_LITERAL_EXPORT_NAME, children)
			}
			JS_LITERAL_MEMBER_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						IDENT | JS_STRING_LITERAL | JS_NUMBER_LITERAL
					) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_LITERAL_MEMBER_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_LITERAL_MEMBER_NAME, children)
			}
			JS_LOGICAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [??] | T ! [||] | T ! [&&]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_LOGICAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_LOGICAL_EXPRESSION, children)
			}
			JS_METHOD_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<10usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_METHOD_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_METHOD_CLASS_MEMBER, children)
			}
			JS_METHOD_OBJECT_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![async] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_METHOD_OBJECT_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_METHOD_OBJECT_MEMBER, children)
			}
			JS_MODULE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_SHEBANG {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsModuleItemList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_MODULE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_MODULE, children)
			}
			JS_MODULE_SOURCE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_MODULE_SOURCE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_MODULE_SOURCE, children)
			}
			JS_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NAME, children)
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsLiteralExportName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NAMED_IMPORT_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NAMED_IMPORT_SPECIFIER, children)
			}
			JS_NAMED_IMPORT_SPECIFIERS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsNamedImportSpecifierList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NAMED_IMPORT_SPECIFIERS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NAMED_IMPORT_SPECIFIERS, children)
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![as] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NAMESPACE_IMPORT_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NAMESPACE_IMPORT_SPECIFIER, children)
			}
			JS_NEW_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCallArguments::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NEW_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NEW_EXPRESSION, children)
			}
			JS_NULL_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![null] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NULL_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NULL_LITERAL_EXPRESSION, children)
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_NUMBER_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_NUMBER_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_NUMBER_LITERAL_EXPRESSION, children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsObjectAssignmentPatternPropertyList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_ASSIGNMENT_PATTERN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_ASSIGNMENT_PATTERN, children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyAssignmentPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY, children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_ASSIGNMENT_PATTERN_REST.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_ASSIGNMENT_PATTERN_REST, children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY, children)
			}
			JS_OBJECT_BINDING_PATTERN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsObjectBindingPatternPropertyList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_BINDING_PATTERN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_BINDING_PATTERN, children)
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_BINDING_PATTERN_PROPERTY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_BINDING_PATTERN_PROPERTY, children)
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_BINDING_PATTERN_REST.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_BINDING_PATTERN_REST, children)
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY, children)
			}
			JS_OBJECT_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsObjectMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_OBJECT_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_OBJECT_EXPRESSION, children)
			}
			JS_PARAMETER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PARAMETER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PARAMETER, children)
			}
			JS_PARAMETERS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameterList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PARAMETERS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PARAMETERS, children)
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PARENTHESIZED_ASSIGNMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PARENTHESIZED_ASSIGNMENT, children)
			}
			JS_PARENTHESIZED_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PARENTHESIZED_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PARENTHESIZED_EXPRESSION, children)
			}
			JS_POST_UPDATE_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [++] | T ! [--]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_POST_UPDATE_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_POST_UPDATE_EXPRESSION, children)
			}
			JS_PRE_UPDATE_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [++] | T ! [--]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyAssignment::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PRE_UPDATE_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PRE_UPDATE_EXPRESSION, children)
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [#] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PRIVATE_CLASS_MEMBER_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PRIVATE_CLASS_MEMBER_NAME, children)
			}
			JS_PRIVATE_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [#] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PRIVATE_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PRIVATE_NAME, children)
			}
			JS_PROPERTY_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<11usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![declare] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PROPERTY_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PROPERTY_CLASS_MEMBER, children)
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_PROPERTY_OBJECT_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_PROPERTY_OBJECT_MEMBER, children)
			}
			JS_REFERENCE_IDENTIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_REFERENCE_IDENTIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_REFERENCE_IDENTIFIER, children)
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_REGEX_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_REGEX_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_REGEX_LITERAL_EXPRESSION, children)
			}
			JS_REST_PARAMETER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_REST_PARAMETER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_REST_PARAMETER, children)
			}
			JS_RETURN_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![return] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_RETURN_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_RETURN_STATEMENT, children)
			}
			JS_SCRIPT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_SHEBANG {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsDirectiveList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsStatementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![EOF] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SCRIPT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SCRIPT, children)
			}
			JS_SEQUENCE_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [,] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SEQUENCE_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SEQUENCE_EXPRESSION, children)
			}
			JS_SETTER_CLASS_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![static] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![abstract] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![set] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyClassMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameter::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SETTER_CLASS_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SETTER_CLASS_MEMBER, children)
			}
			JS_SETTER_OBJECT_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![set] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyObjectMemberName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameter::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFunctionBody::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SETTER_OBJECT_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SETTER_OBJECT_MEMBER, children)
			}
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SHORTHAND_NAMED_IMPORT_SPECIFIER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SHORTHAND_NAMED_IMPORT_SPECIFIER, children)
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsReferenceIdentifier::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SHORTHAND_PROPERTY_OBJECT_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SHORTHAND_PROPERTY_OBJECT_MEMBER, children)
			}
			JS_SPREAD => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SPREAD.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SPREAD, children)
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_STATIC_MEMBER_ASSIGNMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_STATIC_MEMBER_ASSIGNMENT, children)
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T ! [.] | T ! [?.]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_STATIC_MEMBER_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_STATIC_MEMBER_EXPRESSION, children)
			}
			JS_STRING_LITERAL_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_STRING_LITERAL_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_STRING_LITERAL_EXPRESSION, children)
			}
			JS_SUPER_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![super] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SUPER_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SUPER_EXPRESSION, children)
			}
			JS_SWITCH_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![switch] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsSwitchCaseList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_SWITCH_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_SWITCH_STATEMENT, children)
			}
			JS_THIS_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![this] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_THIS_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_THIS_EXPRESSION, children)
			}
			JS_THROW_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![throw] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_THROW_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_THROW_STATEMENT, children)
			}
			JS_TRY_FINALLY_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![try] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCatchClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsFinallyClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_TRY_FINALLY_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_TRY_FINALLY_STATEMENT, children)
			}
			JS_TRY_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![try] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsBlockStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsCatchClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_TRY_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_TRY_STATEMENT, children)
			}
			JS_UNARY_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(
						element.kind(),
						T![delete] | T![void] | T![typeof] | T ! [+] | T ! [-] | T ! [~] | T![!]
					) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_UNARY_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_UNARY_EXPRESSION, children)
			}
			JS_VARIABLE_DECLARATION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_VARIABLE_DECLARATION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_VARIABLE_DECLARATION, children)
			}
			JS_VARIABLE_DECLARATIONS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![var] | T![const] | T![let]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsVariableDeclarationList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_VARIABLE_DECLARATIONS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_VARIABLE_DECLARATIONS, children)
			}
			JS_VARIABLE_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsVariableDeclarations::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_VARIABLE_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_VARIABLE_STATEMENT, children)
			}
			JS_WHILE_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![while] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_WHILE_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_WHILE_STATEMENT, children)
			}
			JS_WITH_STATEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![with] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyStatement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_WITH_STATEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_WITH_STATEMENT, children)
			}
			JS_YIELD_ARGUMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [*] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_YIELD_ARGUMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_YIELD_ARGUMENT, children)
			}
			JS_YIELD_EXPRESSION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![yield] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsYieldArgument::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						JS_YIELD_EXPRESSION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(JS_YIELD_EXPRESSION, children)
			}
			NEW_TARGET => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![target] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						NEW_TARGET.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(NEW_TARGET, children)
			}
			TEMPLATE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['`'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TemplateElementList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['`'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TEMPLATE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TEMPLATE, children)
			}
			TEMPLATE_CHUNK_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == TEMPLATE_CHUNK {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TEMPLATE_CHUNK_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TEMPLATE_CHUNK_ELEMENT, children)
			}
			TEMPLATE_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == DOLLAR_CURLY {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TEMPLATE_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TEMPLATE_ELEMENT, children)
			}
			TS_ANY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![any] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(TS_ANY.to_unknown(), children.into_iter().map(Some));
				}
				slots.into_node(TS_ANY, children)
			}
			TS_ARRAY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_ARRAY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_ARRAY, children)
			}
			TS_ASSERTION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_ASSERTION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_ASSERTION, children)
			}
			TS_BIGINT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_BIGINT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_BIGINT, children)
			}
			TS_BOOLEAN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_BOOLEAN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_BOOLEAN, children)
			}
			TS_CALL_SIGNATURE_DECL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CALL_SIGNATURE_DECL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CALL_SIGNATURE_DECL, children)
			}
			TS_CONDITIONAL_TYPE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsExtends::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONDITIONAL_TYPE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONDITIONAL_TYPE, children)
			}
			TS_CONST_ASSERTION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![const] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONST_ASSERTION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONST_ASSERTION, children)
			}
			TS_CONSTRAINT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONSTRAINT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONSTRAINT, children)
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONSTRUCT_SIGNATURE_DECL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONSTRUCT_SIGNATURE_DECL, children)
			}
			TS_CONSTRUCTOR_PARAM => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if matches!(element.kind(), T![private] | T![protected] | T![public]) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBindingPattern::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeAnnotation::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsInitializerClause::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONSTRUCTOR_PARAM.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONSTRUCTOR_PARAM, children)
			}
			TS_CONSTRUCTOR_TYPE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![new] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_CONSTRUCTOR_TYPE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_CONSTRUCTOR_TYPE, children)
			}
			TS_DEFAULT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_DEFAULT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_DEFAULT, children)
			}
			TS_ENUM => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![const] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![enum] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsEnumMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_ENUM.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_ENUM, children)
			}
			TS_ENUM_MEMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_ENUM_MEMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_ENUM_MEMBER, children)
			}
			TS_EXPR_WITH_TYPE_ARGS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_EXPR_WITH_TYPE_ARGS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_EXPR_WITH_TYPE_ARGS, children)
			}
			TS_EXTENDS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![extends] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_EXTENDS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_EXTENDS, children)
			}
			TS_EXTERNAL_MODULE_REF => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![require] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == JS_STRING_LITERAL {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_EXTERNAL_MODULE_REF.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_EXTERNAL_MODULE_REF, children)
			}
			TS_FN_TYPE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_FN_TYPE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_FN_TYPE, children)
			}
			TS_IMPLEMENTS_CLAUSE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![implements] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_IMPLEMENTS_CLAUSE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_IMPLEMENTS_CLAUSE, children)
			}
			TS_IMPORT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_IMPORT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_IMPORT, children)
			}
			TS_IMPORT_EQUALS_DECL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![import] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![export] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [=] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsModuleRef::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_IMPORT_EQUALS_DECL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_IMPORT_EQUALS_DECL, children)
			}
			TS_INDEX_SIGNATURE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyBinding::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_INDEX_SIGNATURE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_INDEX_SIGNATURE, children)
			}
			TS_INDEXED_ARRAY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_INDEXED_ARRAY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_INDEXED_ARRAY, children)
			}
			TS_INFER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![infer] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_INFER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_INFER, children)
			}
			TS_INTERSECTION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_INTERSECTION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_INTERSECTION, children)
			}
			TS_LITERAL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_LITERAL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_LITERAL, children)
			}
			TS_MAPPED_TYPE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<10usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsMappedTypeReadonly::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [-] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [+] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsMappedTypeParam::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [;] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_MAPPED_TYPE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_MAPPED_TYPE, children)
			}
			TS_MAPPED_TYPE_PARAM => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_MAPPED_TYPE_PARAM.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_MAPPED_TYPE_PARAM, children)
			}
			TS_MAPPED_TYPE_READONLY => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [-] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [+] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_MAPPED_TYPE_READONLY.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_MAPPED_TYPE_READONLY, children)
			}
			TS_METHOD_SIGNATURE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParams::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsParameters::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_METHOD_SIGNATURE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_METHOD_SIGNATURE, children)
			}
			TS_NEVER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![never] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_NEVER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_NEVER, children)
			}
			TS_NON_NULL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![!] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_NON_NULL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_NON_NULL, children)
			}
			TS_NULL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![null] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_NULL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_NULL, children)
			}
			TS_NUMBER => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_NUMBER.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_NUMBER, children)
			}
			TS_OBJECT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_OBJECT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_OBJECT, children)
			}
			TS_OBJECT_TYPE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['{'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsObjectMemberList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_OBJECT_TYPE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_OBJECT_TYPE, children)
			}
			TS_PAREN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['('] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![')'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_PAREN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_PAREN, children)
			}
			TS_PREDICATE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsThisOrMore::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_PREDICATE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_PREDICATE, children)
			}
			TS_PROPERTY_SIGNATURE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![readonly] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if JsAnyExpression::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_PROPERTY_SIGNATURE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_PROPERTY_SIGNATURE, children)
			}
			TS_QUALIFIED_PATH => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [.] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_QUALIFIED_PATH.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_QUALIFIED_PATH, children)
			}
			TS_STRING => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_STRING.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_STRING, children)
			}
			TS_SYMBOL => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_SYMBOL.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_SYMBOL, children)
			}
			TS_TEMPLATE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTemplateElement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TEMPLATE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TEMPLATE, children)
			}
			TS_TEMPLATE_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T!['}'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TEMPLATE_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TEMPLATE_ELEMENT, children)
			}
			TS_THIS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![this] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_THIS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_THIS, children)
			}
			TS_TUPLE => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T!['['] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTupleElement::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T![']'] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TUPLE.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TUPLE, children)
			}
			TS_TUPLE_ELEMENT => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [?] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [...] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TUPLE_ELEMENT.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TUPLE_ELEMENT, children)
			}
			TS_TYPE_ANNOTATION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [:] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_ANNOTATION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_ANNOTATION, children)
			}
			TS_TYPE_ARGS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_ARGS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_ARGS, children)
			}
			TS_TYPE_NAME => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == IDENT {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_NAME.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_NAME, children)
			}
			TS_TYPE_OPERATOR => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsType::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_OPERATOR.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_OPERATOR, children)
			}
			TS_TYPE_PARAM => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if Ident::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsConstraint::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsDefault::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_PARAM.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_PARAM, children)
			}
			TS_TYPE_PARAMS => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T ! [<] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeParam::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if element.kind() == T ! [>] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_PARAMS.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_PARAMS, children)
			}
			TS_TYPE_REF => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsEntityName::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if let Some(element) = &current_element {
					if TsTypeArgs::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_TYPE_REF.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_TYPE_REF, children)
			}
			TS_UNDEFINED => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![undefined] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_UNDEFINED.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_UNDEFINED, children)
			}
			TS_UNION => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if TsTypeList::can_cast(element.kind()) {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_UNION.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_UNION, children)
			}
			TS_UNKNOWN => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![unknown] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_UNKNOWN.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_UNKNOWN, children)
			}
			TS_VOID => {
				let mut elements = (&children).into_iter();
				let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
				let mut current_element = elements.next();
				if let Some(element) = &current_element {
					if element.kind() == T![void] {
						slots.mark_present();
						current_element = elements.next();
					}
				}
				slots.next_slot();
				if current_element.is_some() {
					return RawSyntaxNode::new(
						TS_VOID.to_unknown(),
						children.into_iter().map(Some),
					);
				}
				slots.into_node(TS_VOID, children)
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
			JS_EXPORT_NAMED_FROM_SPECIFIER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsExportNamedFromSpecifier::can_cast,
				T ! [,],
				true,
			),
			JS_EXPORT_NAMED_SPECIFIER_LIST => Self::make_separated_list_syntax(
				kind,
				children,
				JsAnyExportNamedSpecifier::can_cast,
				T ! [,],
				true,
			),
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
