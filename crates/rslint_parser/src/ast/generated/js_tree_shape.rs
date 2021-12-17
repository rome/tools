//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{ast::*, JsLanguage, JsSyntaxKind::*, T};
use rome_rowan::{AstTreeShape, NodeShapCommands, NodeShape, NodeShapeCommand, ParsedElements};
impl AstTreeShape for JsLanguage {
	fn forms_exact_shape_for<F, R>(
		parent: Self::Kind,
		slots: ParsedElements<Self>,
		receiver: F,
	) -> R
	where
		F: FnOnce(Result<NodeShape<'_, Self>, ParsedElements<'_, Self>>) -> R,
	{
		let actual_len = slots.len();
		match parent {
			JS_UNKNOWN
			| JS_UNKNOWN_ASSIGNMENT
			| JS_UNKNOWN_BINDING
			| JS_UNKNOWN_EXPRESSION
			| JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
			| JS_UNKNOWN_MEMBER
			| JS_UNKNOWN_MODIFIER
			| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
			| JS_UNKNOWN_STATEMENT
			| ERROR => receiver(Ok(NodeShape::List(slots))),
			CALL_EXPR => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsCallArguments::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_DECL => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![type] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyExportDeclaration::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_DEFAULT_DECL => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![default] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![type] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if DefaultDecl::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_DEFAULT_EXPR => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![type] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![default] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_NAMED => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if ExportNamedSpecifierList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![from] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_WILDCARD => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![type] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![from] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			FOR_STMT => {
				if actual_len > 9usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<9usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![for] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyForInitializer::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if ForStmtTest::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if ForStmtUpdate::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			FOR_STMT_TEST => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			FOR_STMT_UPDATE => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			IDENT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			IMPORT_META => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![import] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![meta] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsArrayAssignmentPatternElementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyAssignmentPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_BINDING_PATTERN => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsArrayBindingPatternElementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsArrayElementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARRAY_HOLE => {
				if actual_len > 0usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<0usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ARROW_FUNCTION_EXPRESSION => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![async] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyArrowFunctionParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyArrowFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ASSIGNMENT_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyAssignmentPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if matches!(
						*current,
						T ! [=]
							| T ! [+=] | T ! [-=] | T ! [*=]
							| T ! [/=] | T ! [%=] | T ! [**=]
							| T ! [>>=] | T ! [<<=] | T ! [>>>=]
							| T ! [&=] | T ! [|=] | T ! [^=]
							| T ! [&&=] | T ! [||=] | T ! [??=]
					) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ASSIGNMENT_WITH_DEFAULT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyAssignmentPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_AWAIT_EXPRESSION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![await] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_BIG_INT_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BINARY_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if matches!(
						*current,
						T ! [<]
							| T ! [>] | T ! [<=] | T ! [>=]
							| T ! [==] | T ! [===] | T ! [!=]
							| T ! [!==] | T ! [+] | T ! [-]
							| T ! [*] | T ! [/] | T ! [%]
							| T ! [**] | T ! [<<] | T ! [>>]
							| T ! [>>>] | T ! [&] | T ! [|]
							| T ! [^] | T![in] | T![instanceof]
					) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BLOCK_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsStatementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![true] | T![false]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_BREAK_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![break] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CALL_ARGUMENTS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsCallArgumentList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CASE_CLAUSE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![case] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsStatementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CATCH_CLAUSE => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![catch] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsCatchDeclaration::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsBlockStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CATCH_DECLARATION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CLASS_DECLARATION => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![class] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsExtendsClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsImplementsClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsClassMemberList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CLASS_EXPRESSION => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![class] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsExtendsClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsImplementsClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsClassMemberList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_COMPUTED_MEMBER_NAME => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CONDITIONAL_EXPRESSION => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsLiteralMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsConstructorParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CONSTRUCTOR_PARAMETERS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsConstructorParameterList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_CONTINUE_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![continue] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_DEBUGGER_STATEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![debugger] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_DEFAULT_CLAUSE => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![default] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsStatementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_DEFAULT_IMPORT_SPECIFIER => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [,] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_DIRECTIVE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_DO_WHILE_STATEMENT => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![do] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![while] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_ELSE_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![else] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_EMPTY_CLASS_MEMBER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_EMPTY_STATEMENT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_EXPRESSION_SNIPPED => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![EOF] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_EXPRESSION_STATEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_EXTENDS_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![extends] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FINALLY_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![finally] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsBlockStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FOR_IN_STATEMENT => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![for] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyForInOrOfInitializer::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![in] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FOR_OF_STATEMENT => {
				if actual_len > 8usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<8usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![for] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![await] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyForInOrOfInitializer::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![of] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FOR_VARIABLE_DECLARATION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![var] | T![let] | T![const]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsVariableDeclaration::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FUNCTION_BODY => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsDirectiveList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsStatementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FUNCTION_DECLARATION => {
				if actual_len > 8usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<8usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![async] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![function] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_FUNCTION_EXPRESSION => {
				if actual_len > 8usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<8usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![async] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![function] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_GETTER_CLASS_MEMBER => {
				if actual_len > 9usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<9usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![static] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![abstract] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![get] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyClassMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_GETTER_OBJECT_MEMBER => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![get] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyObjectMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IDENTIFIER_ASSIGNMENT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IDENTIFIER_BINDING => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IDENTIFIER_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsReferenceIdentifier::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IF_STATEMENT => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![if] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsElseClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![import] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if AnyJsImportClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_ASSERTION => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![assert] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsImportAssertionEntryList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_ASSERTION_ENTRY => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, IDENT | JS_STRING_LITERAL) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_BARE_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsModuleSource::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsImportAssertion::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_CALL_EXPRESSION => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![import] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![from] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsModuleSource::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsImportAssertion::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_NAMED_CLAUSE => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsDefaultImportSpecifier::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyNamedImport::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![from] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsModuleSource::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsImportAssertion::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![from] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsModuleSource::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsImportAssertion::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_INITIALIZER_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_LABELED_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_LITERAL_EXPORT_NAME => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, IDENT | JS_STRING_LITERAL) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_LITERAL_MEMBER_NAME => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, IDENT | JS_STRING_LITERAL | JS_NUMBER_LITERAL) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_LOGICAL_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if matches!(*current, T ! [??] | T ! [||] | T ! [&&]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_METHOD_CLASS_MEMBER => {
				if actual_len > 10usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<10usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![static] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![abstract] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![async] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyClassMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_METHOD_OBJECT_MEMBER => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![async] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyObjectMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_MODULE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_SHEBANG {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsDirectiveList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsModuleItemList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![EOF] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_MODULE_SOURCE => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NAME => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsLiteralExportName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NAMED_IMPORT_SPECIFIERS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsNamedImportSpecifierList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NULL_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![null] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_NUMBER_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsObjectAssignmentPatternPropertyList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyAssignmentPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyAssignment::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyAssignment::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_BINDING_PATTERN => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsObjectBindingPatternPropertyList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyObjectMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_OBJECT_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsObjectMemberList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PARAMETERS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsParameterList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyAssignment::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PARENTHESIZED_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_POST_UPDATE_EXPRESSION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyAssignment::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if matches!(*current, T ! [++] | T ! [--]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PRE_UPDATE_EXPRESSION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T ! [++] | T ! [--]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyAssignment::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [#] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PRIVATE_NAME => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [#] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PROPERTY_CLASS_MEMBER => {
				if actual_len > 11usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<11usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![declare] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![static] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![abstract] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyClassMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![!] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyObjectMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_REFERENCE_IDENTIFIER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == IDENT {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_REGEX_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_REST_PARAMETER => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_RETURN_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![return] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SCRIPT => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_SHEBANG {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsDirectiveList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsStatementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![EOF] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SEQUENCE_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [,] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SETTER_CLASS_MEMBER => {
				if actual_len > 9usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<9usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![static] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![abstract] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![set] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyClassMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SETTER_OBJECT_MEMBER => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![set] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyObjectMemberName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsFunctionBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsReferenceIdentifier::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SPREAD => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if matches!(*current, T ! [.] | T ! [?.]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_STRING_LITERAL_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SUPER_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![super] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_SWITCH_STATEMENT => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![switch] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsSwitchCaseList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_THIS_EXPRESSION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![this] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_THROW_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![throw] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_TRY_FINALLY_STATEMENT => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![try] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsBlockStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsCatchClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsFinallyClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_TRY_STATEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![try] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsBlockStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsCatchClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_UNARY_EXPRESSION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(
						*current,
						T![delete] | T![void] | T![typeof] | T ! [+] | T ! [-] | T ! [~] | T![!]
					) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_VARIABLE_DECLARATION => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![!] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeAnnotation::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsInitializerClause::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_VARIABLE_DECLARATIONS => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![var] | T![const] | T![let]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsVariableDeclarationList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_VARIABLE_STATEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsVariableDeclarations::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_WHILE_STATEMENT => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![while] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_WITH_STATEMENT => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![with] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			JS_YIELD_EXPRESSION => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![yield] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [*] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			NEW_EXPR => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![new] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsCallArguments::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			NEW_TARGET => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![new] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![target] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			SPECIFIER => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TEMPLATE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['`'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TemplateElementList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['`'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TEMPLATE_CHUNK_ELEMENT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == TEMPLATE_CHUNK {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TEMPLATE_ELEMENT => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == DOLLAR_CURLY {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_ANY => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![any] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_ARRAY => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_ASSERTION => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [<] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_BIGINT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_BOOLEAN => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CALL_SIGNATURE_DECL => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONDITIONAL_TYPE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsExtends::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONST_ASSERTION => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [<] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![const] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONSTRAINT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![extends] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![new] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONSTRUCTOR_PARAM => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if matches!(*current, T![private] | T![protected] | T![public]) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyBindingPattern::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_CONSTRUCTOR_TYPE => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![new] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_DEFAULT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_ENUM => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![const] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![enum] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsEnumMemberList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_ENUM_MEMBER => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_EXPORT_ASSIGNMENT => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_EXPR_WITH_TYPE_ARGS => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsEntityName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_EXTENDS => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![extends] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_EXTERNAL_MODULE_REF => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![require] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == JS_STRING_LITERAL {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_FN_TYPE => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_IMPLEMENTS_CLAUSE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![implements] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_IMPORT => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![import] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsEntityName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_IMPORT_EQUALS_DECL => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![import] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsModuleRef::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_INDEX_SIGNATURE => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyBinding::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_INDEXED_ARRAY => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_INFER => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![infer] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_INTERFACE_DECL => {
				if actual_len > 8usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<8usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![declare] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![interface] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![extends] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsExprWithTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeElement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_INTERSECTION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsTypeList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_LITERAL => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_MAPPED_TYPE => {
				if actual_len > 10usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<10usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsMappedTypeReadonly::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [-] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [+] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsMappedTypeParam::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_MAPPED_TYPE_PARAM => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_MAPPED_TYPE_READONLY => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [-] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [+] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_METHOD_SIGNATURE => {
				if actual_len > 7usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<7usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsParameters::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_MODULE_BLOCK => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if JsAnyStatement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_MODULE_DECL => {
				if actual_len > 6usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<6usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![declare] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![global] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T![module] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsNamespaceBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NAMESPACE_DECL => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![declare] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsNamespaceBody::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NAMESPACE_EXPORT_DECL => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![export] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![as] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![namespace] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if *current == T ! [;] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NEVER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![never] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NON_NULL => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![!] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NULL => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![null] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_NUMBER => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_OBJECT => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_OBJECT_TYPE => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['{'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsObjectMemberList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_PAREN => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['('] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![')'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_PREDICATE => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsThisOrMore::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_PROPERTY_SIGNATURE => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![readonly] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if JsAnyExpression::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_QUALIFIED_PATH => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsEntityName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [.] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_STRING => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_SYMBOL => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TEMPLATE => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsTemplateElement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TEMPLATE_ELEMENT => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T!['}'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_THIS => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![this] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TUPLE => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T!['['] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTupleElement::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T![']'] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TUPLE_ELEMENT => {
				if actual_len > 5usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<5usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [?] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [...] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_ALIAS_DECL => {
				if actual_len > 4usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<4usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![type] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeParams::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [=] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_ANNOTATION => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [:] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_ARGS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [<] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_NAME => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_OPERATOR => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsType::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_PARAM => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if Ident::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsConstraint::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsDefault::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_PARAMS => {
				if actual_len > 3usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<3usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T ! [<] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if let Some(current) = &current_kind {
					if TsTypeParam::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if *current == T ! [>] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						shape.empty()
					}
				} else {
					shape.empty()
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_TYPE_REF => {
				if actual_len > 2usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<2usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsEntityName::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if let Some(current) = &current_kind {
					if TsTypeArgs::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_UNDEFINED => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![undefined] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_UNION => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if TsTypeList::can_cast(*current) {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_UNKNOWN => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![unknown] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			TS_VOID => {
				if actual_len > 1usize {
					return receiver(Err(slots));
				}
				let mut shape = NodeShapCommands::<1usize>::default();
				let mut kinds = slots.kinds();
				let mut current_kind = kinds.next();
				if let Some(current) = &current_kind {
					if *current == T![void] {
						shape.occupied();
						current_kind = kinds.next();
					} else {
						drop(kinds);
						return receiver(Err(slots));
					}
				} else {
					drop(kinds);
					return receiver(Err(slots));
				}
				if current_kind.is_some() {
					drop(kinds);
					return receiver(Err(slots));
				}
				drop(kinds);
				receiver(Ok(NodeShape::Normal {
					commands: shape.as_slice(),
					parsed_elements: slots,
				}))
			}
			EXPORT_NAMED_SPECIFIER_LIST => receiver(
				if Self::forms_separated_list_shape(
					Specifier::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyArrayAssignmentPatternElement::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyArrayBindingPatternElement::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_ARRAY_ELEMENT_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyArrayElement::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_CALL_ARGUMENT_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyExpression::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_CLASS_MEMBER_LIST => receiver(
				if Self::forms_node_list_shape(JsAnyClassMember::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_CONSTRUCTOR_PARAMETER_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyConstructorParameter::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_DIRECTIVE_LIST => receiver(
				if Self::forms_node_list_shape(JsDirective::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_IMPORT_ASSERTION_ENTRY_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyImportAssertionEntry::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_MODULE_ITEM_LIST => receiver(
				if Self::forms_node_list_shape(JsAnyModuleItem::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_NAMED_IMPORT_SPECIFIER_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyNamedImportSpecifier::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyObjectAssignmentPatternMember::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyObjectBindingPatternMember::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_OBJECT_MEMBER_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyObjectMember::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_PARAMETER_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsAnyParameter::can_cast,
					T ! [,],
					true,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_STATEMENT_LIST => receiver(
				if Self::forms_node_list_shape(JsAnyStatement::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_SWITCH_CASE_LIST => receiver(
				if Self::forms_node_list_shape(JsAnySwitchClause::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			JS_VARIABLE_DECLARATION_LIST => receiver(
				if Self::forms_separated_list_shape(
					JsVariableDeclaration::can_cast,
					T ! [,],
					false,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TEMPLATE_ELEMENT_LIST => receiver(
				if Self::forms_node_list_shape(AnyTemplateElement::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TS_ENUM_MEMBER_LIST => receiver(
				if Self::forms_node_list_shape(TsEnumMember::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TS_OBJECT_MEMBER_LIST => receiver(
				if Self::forms_node_list_shape(TsTypeElement::can_cast, slots.kinds()) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TS_TYPE_ARG_LIST => receiver(
				if Self::forms_separated_list_shape(TsType::can_cast, T ! [,], false, slots.kinds())
				{
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TS_TYPE_LIST => receiver(
				if Self::forms_separated_list_shape(
					TsExprWithTypeArgs::can_cast,
					T ! [,],
					false,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			TS_TYPE_PARAM_LIST => receiver(
				if Self::forms_separated_list_shape(
					TsTypeParam::can_cast,
					T ! [,],
					false,
					slots.kinds(),
				) {
					Ok(NodeShape::List(slots))
				} else {
					Err(slots)
				},
			),
			_ => unreachable!("Is {:?} a token?", parent),
		}
	}
}
