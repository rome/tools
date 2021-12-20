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
			| JS_UNKNOWN_MODIFIER
			| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
			| JS_UNKNOWN_STATEMENT
			| ERROR => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
			CALL_EXPR => Self::make_call_expr(children),
			EXPORT_DECL => Self::make_export_decl(children),
			EXPORT_DEFAULT_DECL => Self::make_export_default_decl(children),
			EXPORT_DEFAULT_EXPR => Self::make_export_default_expr(children),
			EXPORT_NAMED => Self::make_export_named(children),
			EXPORT_WILDCARD => Self::make_export_wildcard(children),
			FOR_STMT => Self::make_for_stmt(children),
			FOR_STMT_TEST => Self::make_for_stmt_test(children),
			FOR_STMT_UPDATE => Self::make_for_stmt_update(children),
			IDENT => Self::make_ident(children),
			IMPORT_META => Self::make_import_meta(children),
			JS_ARRAY_ASSIGNMENT_PATTERN => Self::make_js_array_assignment_pattern(children),
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				Self::make_js_array_assignment_pattern_rest_element(children)
			}
			JS_ARRAY_BINDING_PATTERN => Self::make_js_array_binding_pattern(children),
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				Self::make_js_array_binding_pattern_rest_element(children)
			}
			JS_ARRAY_EXPRESSION => Self::make_js_array_expression(children),
			JS_ARRAY_HOLE => Self::make_js_array_hole(children),
			JS_ARROW_FUNCTION_EXPRESSION => Self::make_js_arrow_function_expression(children),
			JS_ASSIGNMENT_EXPRESSION => Self::make_js_assignment_expression(children),
			JS_ASSIGNMENT_WITH_DEFAULT => Self::make_js_assignment_with_default(children),
			JS_AWAIT_EXPRESSION => Self::make_js_await_expression(children),
			JS_BIG_INT_LITERAL_EXPRESSION => Self::make_js_big_int_literal_expression(children),
			JS_BINARY_EXPRESSION => Self::make_js_binary_expression(children),
			JS_BINDING_PATTERN_WITH_DEFAULT => Self::make_js_binding_pattern_with_default(children),
			JS_BLOCK_STATEMENT => Self::make_js_block_statement(children),
			JS_BOOLEAN_LITERAL_EXPRESSION => Self::make_js_boolean_literal_expression(children),
			JS_BREAK_STATEMENT => Self::make_js_break_statement(children),
			JS_CALL_ARGUMENTS => Self::make_js_call_arguments(children),
			JS_CASE_CLAUSE => Self::make_js_case_clause(children),
			JS_CATCH_CLAUSE => Self::make_js_catch_clause(children),
			JS_CATCH_DECLARATION => Self::make_js_catch_declaration(children),
			JS_CLASS_DECLARATION => Self::make_js_class_declaration(children),
			JS_CLASS_EXPRESSION => Self::make_js_class_expression(children),
			JS_COMPUTED_MEMBER_ASSIGNMENT => Self::make_js_computed_member_assignment(children),
			JS_COMPUTED_MEMBER_EXPRESSION => Self::make_js_computed_member_expression(children),
			JS_COMPUTED_MEMBER_NAME => Self::make_js_computed_member_name(children),
			JS_CONDITIONAL_EXPRESSION => Self::make_js_conditional_expression(children),
			JS_CONSTRUCTOR_CLASS_MEMBER => Self::make_js_constructor_class_member(children),
			JS_CONSTRUCTOR_PARAMETERS => Self::make_js_constructor_parameters(children),
			JS_CONTINUE_STATEMENT => Self::make_js_continue_statement(children),
			JS_DEBUGGER_STATEMENT => Self::make_js_debugger_statement(children),
			JS_DEFAULT_CLAUSE => Self::make_js_default_clause(children),
			JS_DEFAULT_IMPORT_SPECIFIER => Self::make_js_default_import_specifier(children),
			JS_DIRECTIVE => Self::make_js_directive(children),
			JS_DO_WHILE_STATEMENT => Self::make_js_do_while_statement(children),
			JS_ELSE_CLAUSE => Self::make_js_else_clause(children),
			JS_EMPTY_CLASS_MEMBER => Self::make_js_empty_class_member(children),
			JS_EMPTY_STATEMENT => Self::make_js_empty_statement(children),
			JS_EXPRESSION_SNIPPED => Self::make_js_expression_snipped(children),
			JS_EXPRESSION_STATEMENT => Self::make_js_expression_statement(children),
			JS_EXTENDS_CLAUSE => Self::make_js_extends_clause(children),
			JS_FINALLY_CLAUSE => Self::make_js_finally_clause(children),
			JS_FOR_IN_STATEMENT => Self::make_js_for_in_statement(children),
			JS_FOR_OF_STATEMENT => Self::make_js_for_of_statement(children),
			JS_FOR_VARIABLE_DECLARATION => Self::make_js_for_variable_declaration(children),
			JS_FUNCTION_BODY => Self::make_js_function_body(children),
			JS_FUNCTION_DECLARATION => Self::make_js_function_declaration(children),
			JS_FUNCTION_EXPRESSION => Self::make_js_function_expression(children),
			JS_GETTER_CLASS_MEMBER => Self::make_js_getter_class_member(children),
			JS_GETTER_OBJECT_MEMBER => Self::make_js_getter_object_member(children),
			JS_IDENTIFIER_ASSIGNMENT => Self::make_js_identifier_assignment(children),
			JS_IDENTIFIER_BINDING => Self::make_js_identifier_binding(children),
			JS_IDENTIFIER_EXPRESSION => Self::make_js_identifier_expression(children),
			JS_IF_STATEMENT => Self::make_js_if_statement(children),
			JS_IMPORT => Self::make_js_import(children),
			JS_IMPORT_ASSERTION => Self::make_js_import_assertion(children),
			JS_IMPORT_ASSERTION_ENTRY => Self::make_js_import_assertion_entry(children),
			JS_IMPORT_BARE_CLAUSE => Self::make_js_import_bare_clause(children),
			JS_IMPORT_CALL_EXPRESSION => Self::make_js_import_call_expression(children),
			JS_IMPORT_DEFAULT_CLAUSE => Self::make_js_import_default_clause(children),
			JS_IMPORT_NAMED_CLAUSE => Self::make_js_import_named_clause(children),
			JS_IMPORT_NAMESPACE_CLAUSE => Self::make_js_import_namespace_clause(children),
			JS_INITIALIZER_CLAUSE => Self::make_js_initializer_clause(children),
			JS_LABELED_STATEMENT => Self::make_js_labeled_statement(children),
			JS_LITERAL_EXPORT_NAME => Self::make_js_literal_export_name(children),
			JS_LITERAL_MEMBER_NAME => Self::make_js_literal_member_name(children),
			JS_LOGICAL_EXPRESSION => Self::make_js_logical_expression(children),
			JS_METHOD_CLASS_MEMBER => Self::make_js_method_class_member(children),
			JS_METHOD_OBJECT_MEMBER => Self::make_js_method_object_member(children),
			JS_MODULE => Self::make_js_module(children),
			JS_MODULE_SOURCE => Self::make_js_module_source(children),
			JS_NAME => Self::make_js_name(children),
			JS_NAMED_IMPORT_SPECIFIER => Self::make_js_named_import_specifier(children),
			JS_NAMED_IMPORT_SPECIFIERS => Self::make_js_named_import_specifiers(children),
			JS_NAMESPACE_IMPORT_SPECIFIER => Self::make_js_namespace_import_specifier(children),
			JS_NULL_LITERAL_EXPRESSION => Self::make_js_null_literal_expression(children),
			JS_NUMBER_LITERAL_EXPRESSION => Self::make_js_number_literal_expression(children),
			JS_OBJECT_ASSIGNMENT_PATTERN => Self::make_js_object_assignment_pattern(children),
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				Self::make_js_object_assignment_pattern_property(children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				Self::make_js_object_assignment_pattern_rest(children)
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				Self::make_js_object_assignment_pattern_shorthand_property(children)
			}
			JS_OBJECT_BINDING_PATTERN => Self::make_js_object_binding_pattern(children),
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				Self::make_js_object_binding_pattern_property(children)
			}
			JS_OBJECT_BINDING_PATTERN_REST => Self::make_js_object_binding_pattern_rest(children),
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				Self::make_js_object_binding_pattern_shorthand_property(children)
			}
			JS_OBJECT_EXPRESSION => Self::make_js_object_expression(children),
			JS_PARAMETERS => Self::make_js_parameters(children),
			JS_PARENTHESIZED_ASSIGNMENT => Self::make_js_parenthesized_assignment(children),
			JS_PARENTHESIZED_EXPRESSION => Self::make_js_parenthesized_expression(children),
			JS_POST_UPDATE_EXPRESSION => Self::make_js_post_update_expression(children),
			JS_PRE_UPDATE_EXPRESSION => Self::make_js_pre_update_expression(children),
			JS_PRIVATE_CLASS_MEMBER_NAME => Self::make_js_private_class_member_name(children),
			JS_PRIVATE_NAME => Self::make_js_private_name(children),
			JS_PROPERTY_CLASS_MEMBER => Self::make_js_property_class_member(children),
			JS_PROPERTY_OBJECT_MEMBER => Self::make_js_property_object_member(children),
			JS_REFERENCE_IDENTIFIER => Self::make_js_reference_identifier(children),
			JS_REGEX_LITERAL_EXPRESSION => Self::make_js_regex_literal_expression(children),
			JS_REST_PARAMETER => Self::make_js_rest_parameter(children),
			JS_RETURN_STATEMENT => Self::make_js_return_statement(children),
			JS_SCRIPT => Self::make_js_script(children),
			JS_SEQUENCE_EXPRESSION => Self::make_js_sequence_expression(children),
			JS_SETTER_CLASS_MEMBER => Self::make_js_setter_class_member(children),
			JS_SETTER_OBJECT_MEMBER => Self::make_js_setter_object_member(children),
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				Self::make_js_shorthand_named_import_specifier(children)
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				Self::make_js_shorthand_property_object_member(children)
			}
			JS_SPREAD => Self::make_js_spread(children),
			JS_STATIC_MEMBER_ASSIGNMENT => Self::make_js_static_member_assignment(children),
			JS_STATIC_MEMBER_EXPRESSION => Self::make_js_static_member_expression(children),
			JS_STRING_LITERAL_EXPRESSION => Self::make_js_string_literal_expression(children),
			JS_SUPER_EXPRESSION => Self::make_js_super_expression(children),
			JS_SWITCH_STATEMENT => Self::make_js_switch_statement(children),
			JS_THIS_EXPRESSION => Self::make_js_this_expression(children),
			JS_THROW_STATEMENT => Self::make_js_throw_statement(children),
			JS_TRY_FINALLY_STATEMENT => Self::make_js_try_finally_statement(children),
			JS_TRY_STATEMENT => Self::make_js_try_statement(children),
			JS_UNARY_EXPRESSION => Self::make_js_unary_expression(children),
			JS_VARIABLE_DECLARATION => Self::make_js_variable_declaration(children),
			JS_VARIABLE_DECLARATIONS => Self::make_js_variable_declarations(children),
			JS_VARIABLE_STATEMENT => Self::make_js_variable_statement(children),
			JS_WHILE_STATEMENT => Self::make_js_while_statement(children),
			JS_WITH_STATEMENT => Self::make_js_with_statement(children),
			JS_YIELD_EXPRESSION => Self::make_js_yield_expression(children),
			NEW_EXPR => Self::make_new_expr(children),
			NEW_TARGET => Self::make_new_target(children),
			SPECIFIER => Self::make_specifier(children),
			TEMPLATE => Self::make_template(children),
			TEMPLATE_CHUNK_ELEMENT => Self::make_template_chunk_element(children),
			TEMPLATE_ELEMENT => Self::make_template_element(children),
			TS_ANY => Self::make_ts_any(children),
			TS_ARRAY => Self::make_ts_array(children),
			TS_ASSERTION => Self::make_ts_assertion(children),
			TS_BIGINT => Self::make_ts_bigint(children),
			TS_BOOLEAN => Self::make_ts_boolean(children),
			TS_CALL_SIGNATURE_DECL => Self::make_ts_call_signature_decl(children),
			TS_CONDITIONAL_TYPE => Self::make_ts_conditional_type(children),
			TS_CONST_ASSERTION => Self::make_ts_const_assertion(children),
			TS_CONSTRAINT => Self::make_ts_constraint(children),
			TS_CONSTRUCT_SIGNATURE_DECL => Self::make_ts_construct_signature_decl(children),
			TS_CONSTRUCTOR_PARAM => Self::make_ts_constructor_param(children),
			TS_CONSTRUCTOR_TYPE => Self::make_ts_constructor_type(children),
			TS_DEFAULT => Self::make_ts_default(children),
			TS_ENUM => Self::make_ts_enum(children),
			TS_ENUM_MEMBER => Self::make_ts_enum_member(children),
			TS_EXPORT_ASSIGNMENT => Self::make_ts_export_assignment(children),
			TS_EXPR_WITH_TYPE_ARGS => Self::make_ts_expr_with_type_args(children),
			TS_EXTENDS => Self::make_ts_extends(children),
			TS_EXTERNAL_MODULE_REF => Self::make_ts_external_module_ref(children),
			TS_FN_TYPE => Self::make_ts_fn_type(children),
			TS_IMPLEMENTS_CLAUSE => Self::make_ts_implements_clause(children),
			TS_IMPORT => Self::make_ts_import(children),
			TS_IMPORT_EQUALS_DECL => Self::make_ts_import_equals_decl(children),
			TS_INDEX_SIGNATURE => Self::make_ts_index_signature(children),
			TS_INDEXED_ARRAY => Self::make_ts_indexed_array(children),
			TS_INFER => Self::make_ts_infer(children),
			TS_INTERFACE_DECL => Self::make_ts_interface_decl(children),
			TS_INTERSECTION => Self::make_ts_intersection(children),
			TS_LITERAL => Self::make_ts_literal(children),
			TS_MAPPED_TYPE => Self::make_ts_mapped_type(children),
			TS_MAPPED_TYPE_PARAM => Self::make_ts_mapped_type_param(children),
			TS_MAPPED_TYPE_READONLY => Self::make_ts_mapped_type_readonly(children),
			TS_METHOD_SIGNATURE => Self::make_ts_method_signature(children),
			TS_MODULE_BLOCK => Self::make_ts_module_block(children),
			TS_MODULE_DECL => Self::make_ts_module_decl(children),
			TS_NAMESPACE_DECL => Self::make_ts_namespace_decl(children),
			TS_NAMESPACE_EXPORT_DECL => Self::make_ts_namespace_export_decl(children),
			TS_NEVER => Self::make_ts_never(children),
			TS_NON_NULL => Self::make_ts_non_null(children),
			TS_NULL => Self::make_ts_null(children),
			TS_NUMBER => Self::make_ts_number(children),
			TS_OBJECT => Self::make_ts_object(children),
			TS_OBJECT_TYPE => Self::make_ts_object_type(children),
			TS_PAREN => Self::make_ts_paren(children),
			TS_PREDICATE => Self::make_ts_predicate(children),
			TS_PROPERTY_SIGNATURE => Self::make_ts_property_signature(children),
			TS_QUALIFIED_PATH => Self::make_ts_qualified_path(children),
			TS_STRING => Self::make_ts_string(children),
			TS_SYMBOL => Self::make_ts_symbol(children),
			TS_TEMPLATE => Self::make_ts_template(children),
			TS_TEMPLATE_ELEMENT => Self::make_ts_template_element(children),
			TS_THIS => Self::make_ts_this(children),
			TS_TUPLE => Self::make_ts_tuple(children),
			TS_TUPLE_ELEMENT => Self::make_ts_tuple_element(children),
			TS_TYPE_ALIAS_DECL => Self::make_ts_type_alias_decl(children),
			TS_TYPE_ANNOTATION => Self::make_ts_type_annotation(children),
			TS_TYPE_ARGS => Self::make_ts_type_args(children),
			TS_TYPE_NAME => Self::make_ts_type_name(children),
			TS_TYPE_OPERATOR => Self::make_ts_type_operator(children),
			TS_TYPE_PARAM => Self::make_ts_type_param(children),
			TS_TYPE_PARAMS => Self::make_ts_type_params(children),
			TS_TYPE_REF => Self::make_ts_type_ref(children),
			TS_UNDEFINED => Self::make_ts_undefined(children),
			TS_UNION => Self::make_ts_union(children),
			TS_UNKNOWN => Self::make_ts_unknown(children),
			TS_VOID => Self::make_ts_void(children),
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
impl JsSyntaxFactory {
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_call_expr(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(CALL_EXPR.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				CALL_EXPR.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(CALL_EXPR, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_export_decl(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(EXPORT_DECL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				EXPORT_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(EXPORT_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_export_default_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				EXPORT_DEFAULT_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				EXPORT_DEFAULT_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(EXPORT_DEFAULT_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_export_default_expr(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				EXPORT_DEFAULT_EXPR.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				EXPORT_DEFAULT_EXPR.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(EXPORT_DEFAULT_EXPR, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_export_named(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(EXPORT_NAMED.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				EXPORT_NAMED.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(EXPORT_NAMED, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_export_wildcard(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				EXPORT_WILDCARD.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				EXPORT_WILDCARD.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(EXPORT_WILDCARD, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_for_stmt(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 9usize {
			return RawSyntaxNode::new(FOR_STMT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] = Default::default();
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
				FOR_STMT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(FOR_STMT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_for_stmt_test(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(FOR_STMT_TEST.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				FOR_STMT_TEST.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(FOR_STMT_TEST, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_for_stmt_update(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				FOR_STMT_UPDATE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				FOR_STMT_UPDATE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(FOR_STMT_UPDATE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ident(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(IDENT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				IDENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(IDENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_import_meta(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(IMPORT_META.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				IMPORT_META.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(IMPORT_META, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_assignment_pattern(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_ARRAY_ASSIGNMENT_PATTERN.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_ARRAY_ASSIGNMENT_PATTERN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_ASSIGNMENT_PATTERN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_assignment_pattern_rest_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_binding_pattern(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_ARRAY_BINDING_PATTERN.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_ARRAY_BINDING_PATTERN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_BINDING_PATTERN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_binding_pattern_rest_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_ARRAY_BINDING_PATTERN_REST_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_ARRAY_BINDING_PATTERN_REST_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_BINDING_PATTERN_REST_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_ARRAY_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_ARRAY_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_array_hole(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 0usize {
			return RawSyntaxNode::new(JS_ARRAY_HOLE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 0usize] = Default::default();
		let mut current_element = elements.next();
		if let Some(element) = current_element {
			return RawSyntaxNode::new(
				JS_ARRAY_HOLE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARRAY_HOLE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_arrow_function_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				JS_ARROW_FUNCTION_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				JS_ARROW_FUNCTION_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ARROW_FUNCTION_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_assignment_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_ASSIGNMENT_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
					| T ! [+=] | T ! [-=]
					| T ! [*=] | T ! [/=]
					| T ! [%=] | T ! [**=]
					| T ! [>>=] | T ! [<<=]
					| T ! [>>>=] | T ! [&=]
					| T ! [|=] | T ! [^=]
					| T ! [&&=] | T ! [||=]
					| T ! [??=]
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
				JS_ASSIGNMENT_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ASSIGNMENT_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_assignment_with_default(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_ASSIGNMENT_WITH_DEFAULT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_ASSIGNMENT_WITH_DEFAULT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ASSIGNMENT_WITH_DEFAULT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_await_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_AWAIT_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_AWAIT_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_AWAIT_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_big_int_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_BIG_INT_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_BIG_INT_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BIG_INT_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_binary_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_BINARY_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
					| T ! [==] | T ! [===]
					| T ! [!=] | T ! [!==]
					| T ! [+] | T ! [-] | T ! [*]
					| T ! [/] | T ! [%] | T ! [**]
					| T ! [<<] | T ! [>>]
					| T ! [>>>] | T ! [&]
					| T ! [|] | T ! [^] | T![in]
					| T![instanceof]
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
				JS_BINARY_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BINARY_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_binding_pattern_with_default(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_BINDING_PATTERN_WITH_DEFAULT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_BINDING_PATTERN_WITH_DEFAULT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BINDING_PATTERN_WITH_DEFAULT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_block_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_BLOCK_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_BLOCK_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BLOCK_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_boolean_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_BOOLEAN_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_BOOLEAN_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BOOLEAN_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_break_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_BREAK_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_BREAK_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_BREAK_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_call_arguments(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_CALL_ARGUMENTS.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_CALL_ARGUMENTS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CALL_ARGUMENTS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_case_clause(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(JS_CASE_CLAUSE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_CASE_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CASE_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_catch_clause(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_CATCH_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_CATCH_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CATCH_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_catch_declaration(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_CATCH_DECLARATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_CATCH_DECLARATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CATCH_DECLARATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_class_declaration(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_CLASS_DECLARATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_CLASS_DECLARATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CLASS_DECLARATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_class_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_CLASS_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_CLASS_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CLASS_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_computed_member_assignment(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_COMPUTED_MEMBER_ASSIGNMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_COMPUTED_MEMBER_ASSIGNMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_COMPUTED_MEMBER_ASSIGNMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_computed_member_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_COMPUTED_MEMBER_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_COMPUTED_MEMBER_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_COMPUTED_MEMBER_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_computed_member_name(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_COMPUTED_MEMBER_NAME.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_COMPUTED_MEMBER_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_COMPUTED_MEMBER_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_conditional_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_CONDITIONAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_CONDITIONAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CONDITIONAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_constructor_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_CONSTRUCTOR_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_CONSTRUCTOR_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CONSTRUCTOR_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_constructor_parameters(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_CONSTRUCTOR_PARAMETERS.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_CONSTRUCTOR_PARAMETERS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CONSTRUCTOR_PARAMETERS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_continue_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_CONTINUE_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_CONTINUE_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_CONTINUE_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_debugger_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_DEBUGGER_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_DEBUGGER_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_DEBUGGER_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_default_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_DEFAULT_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_DEFAULT_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_DEFAULT_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_default_import_specifier(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_DEFAULT_IMPORT_SPECIFIER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_DEFAULT_IMPORT_SPECIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_DEFAULT_IMPORT_SPECIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_directive(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(JS_DIRECTIVE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_DIRECTIVE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_DIRECTIVE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_do_while_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_DO_WHILE_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_DO_WHILE_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_DO_WHILE_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_else_clause(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(JS_ELSE_CLAUSE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_ELSE_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_ELSE_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_empty_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_EMPTY_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_EMPTY_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_EMPTY_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_empty_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_EMPTY_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_EMPTY_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_EMPTY_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_expression_snipped(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_EXPRESSION_SNIPPED.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_EXPRESSION_SNIPPED.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_EXPRESSION_SNIPPED, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_expression_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_EXPRESSION_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_EXPRESSION_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_EXPRESSION_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_extends_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_EXTENDS_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_EXTENDS_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_EXTENDS_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_finally_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_FINALLY_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_FINALLY_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FINALLY_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_for_in_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_FOR_IN_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_FOR_IN_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FOR_IN_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_for_of_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 8usize {
			return RawSyntaxNode::new(
				JS_FOR_OF_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] = Default::default();
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
				JS_FOR_OF_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FOR_OF_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_for_variable_declaration(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_FOR_VARIABLE_DECLARATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_FOR_VARIABLE_DECLARATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FOR_VARIABLE_DECLARATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_function_body(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_FUNCTION_BODY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_FUNCTION_BODY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FUNCTION_BODY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_function_declaration(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 8usize {
			return RawSyntaxNode::new(
				JS_FUNCTION_DECLARATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] = Default::default();
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
				JS_FUNCTION_DECLARATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FUNCTION_DECLARATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_function_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 8usize {
			return RawSyntaxNode::new(
				JS_FUNCTION_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] = Default::default();
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
				JS_FUNCTION_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_FUNCTION_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_getter_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 9usize {
			return RawSyntaxNode::new(
				JS_GETTER_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] = Default::default();
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
				JS_GETTER_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_GETTER_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_getter_object_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				JS_GETTER_OBJECT_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				JS_GETTER_OBJECT_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_GETTER_OBJECT_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_identifier_assignment(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_IDENTIFIER_ASSIGNMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_IDENTIFIER_ASSIGNMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IDENTIFIER_ASSIGNMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_identifier_binding(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_IDENTIFIER_BINDING.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_IDENTIFIER_BINDING.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IDENTIFIER_BINDING, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_identifier_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_IDENTIFIER_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_IDENTIFIER_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IDENTIFIER_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_if_statement(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				JS_IF_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				JS_IF_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IF_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(JS_IMPORT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_IMPORT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_assertion(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_IMPORT_ASSERTION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_IMPORT_ASSERTION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_ASSERTION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_assertion_entry(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_IMPORT_ASSERTION_ENTRY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_IMPORT_ASSERTION_ENTRY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_ASSERTION_ENTRY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_bare_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_IMPORT_BARE_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_IMPORT_BARE_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_BARE_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_call_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_IMPORT_CALL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_IMPORT_CALL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_CALL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_default_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_IMPORT_DEFAULT_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_IMPORT_DEFAULT_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_DEFAULT_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_named_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_IMPORT_NAMED_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_IMPORT_NAMED_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_NAMED_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_import_namespace_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				JS_IMPORT_NAMESPACE_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				JS_IMPORT_NAMESPACE_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_IMPORT_NAMESPACE_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_initializer_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_INITIALIZER_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_INITIALIZER_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_INITIALIZER_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_labeled_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_LABELED_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_LABELED_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_LABELED_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_literal_export_name(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_LITERAL_EXPORT_NAME.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_LITERAL_EXPORT_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_LITERAL_EXPORT_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_literal_member_name(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_LITERAL_MEMBER_NAME.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_LITERAL_MEMBER_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_LITERAL_MEMBER_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_logical_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_LOGICAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_LOGICAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_LOGICAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_method_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 10usize {
			return RawSyntaxNode::new(
				JS_METHOD_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 10usize] = Default::default();
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
				JS_METHOD_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_METHOD_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_method_object_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_METHOD_OBJECT_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_METHOD_OBJECT_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_METHOD_OBJECT_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_module(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(JS_MODULE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_MODULE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_MODULE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_module_source(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_MODULE_SOURCE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_MODULE_SOURCE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_MODULE_SOURCE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_name(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(JS_NAME.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_named_import_specifier(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_NAMED_IMPORT_SPECIFIER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_NAMED_IMPORT_SPECIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NAMED_IMPORT_SPECIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_named_import_specifiers(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_NAMED_IMPORT_SPECIFIERS.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_NAMED_IMPORT_SPECIFIERS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NAMED_IMPORT_SPECIFIERS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_namespace_import_specifier(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_NAMESPACE_IMPORT_SPECIFIER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_NAMESPACE_IMPORT_SPECIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NAMESPACE_IMPORT_SPECIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_null_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_NULL_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_NULL_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NULL_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_number_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_NUMBER_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_NUMBER_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_NUMBER_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_assignment_pattern(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_OBJECT_ASSIGNMENT_PATTERN.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_OBJECT_ASSIGNMENT_PATTERN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_ASSIGNMENT_PATTERN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_assignment_pattern_property(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_assignment_pattern_rest(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_OBJECT_ASSIGNMENT_PATTERN_REST.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_OBJECT_ASSIGNMENT_PATTERN_REST.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_ASSIGNMENT_PATTERN_REST, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_assignment_pattern_shorthand_property(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_binding_pattern(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_OBJECT_BINDING_PATTERN.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_OBJECT_BINDING_PATTERN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_BINDING_PATTERN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_binding_pattern_property(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_OBJECT_BINDING_PATTERN_PROPERTY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_OBJECT_BINDING_PATTERN_PROPERTY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_BINDING_PATTERN_PROPERTY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_binding_pattern_rest(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_OBJECT_BINDING_PATTERN_REST.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_OBJECT_BINDING_PATTERN_REST.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_BINDING_PATTERN_REST, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_binding_pattern_shorthand_property(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_object_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_OBJECT_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_OBJECT_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_OBJECT_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_parameters(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(JS_PARAMETERS.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_PARAMETERS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PARAMETERS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_parenthesized_assignment(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_PARENTHESIZED_ASSIGNMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_PARENTHESIZED_ASSIGNMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PARENTHESIZED_ASSIGNMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_parenthesized_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_PARENTHESIZED_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_PARENTHESIZED_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PARENTHESIZED_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_post_update_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_POST_UPDATE_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_POST_UPDATE_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_POST_UPDATE_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_pre_update_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_PRE_UPDATE_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_PRE_UPDATE_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PRE_UPDATE_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_private_class_member_name(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_PRIVATE_CLASS_MEMBER_NAME.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_PRIVATE_CLASS_MEMBER_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PRIVATE_CLASS_MEMBER_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_private_name(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_PRIVATE_NAME.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_PRIVATE_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PRIVATE_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_property_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 11usize {
			return RawSyntaxNode::new(
				JS_PROPERTY_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 11usize] = Default::default();
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
				JS_PROPERTY_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PROPERTY_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_property_object_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_PROPERTY_OBJECT_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_PROPERTY_OBJECT_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_PROPERTY_OBJECT_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_reference_identifier(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_REFERENCE_IDENTIFIER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_REFERENCE_IDENTIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_REFERENCE_IDENTIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_regex_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_REGEX_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_REGEX_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_REGEX_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_rest_parameter(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_REST_PARAMETER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_REST_PARAMETER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_REST_PARAMETER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_return_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_RETURN_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_RETURN_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_RETURN_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_script(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(JS_SCRIPT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_SCRIPT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SCRIPT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_sequence_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_SEQUENCE_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_SEQUENCE_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SEQUENCE_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_setter_class_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 9usize {
			return RawSyntaxNode::new(
				JS_SETTER_CLASS_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 9usize] = Default::default();
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
				JS_SETTER_CLASS_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SETTER_CLASS_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_setter_object_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				JS_SETTER_OBJECT_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				JS_SETTER_OBJECT_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SETTER_OBJECT_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_shorthand_named_import_specifier(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_SHORTHAND_NAMED_IMPORT_SPECIFIER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_SHORTHAND_NAMED_IMPORT_SPECIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SHORTHAND_NAMED_IMPORT_SPECIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_shorthand_property_object_member(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_SHORTHAND_PROPERTY_OBJECT_MEMBER.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_SHORTHAND_PROPERTY_OBJECT_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SHORTHAND_PROPERTY_OBJECT_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_spread(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(JS_SPREAD.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_SPREAD.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SPREAD, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_static_member_assignment(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_STATIC_MEMBER_ASSIGNMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_STATIC_MEMBER_ASSIGNMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_STATIC_MEMBER_ASSIGNMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_static_member_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_STATIC_MEMBER_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_STATIC_MEMBER_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_STATIC_MEMBER_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_string_literal_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_STRING_LITERAL_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_STRING_LITERAL_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_STRING_LITERAL_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_super_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_SUPER_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_SUPER_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SUPER_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_switch_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				JS_SWITCH_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				JS_SWITCH_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_SWITCH_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_this_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				JS_THIS_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				JS_THIS_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_THIS_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_throw_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_THROW_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_THROW_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_THROW_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_try_finally_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_TRY_FINALLY_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_TRY_FINALLY_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_TRY_FINALLY_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_try_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_TRY_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_TRY_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_TRY_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_unary_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_UNARY_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_UNARY_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_UNARY_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_variable_declaration(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				JS_VARIABLE_DECLARATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				JS_VARIABLE_DECLARATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_VARIABLE_DECLARATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_variable_declarations(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_VARIABLE_DECLARATIONS.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_VARIABLE_DECLARATIONS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_VARIABLE_DECLARATIONS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_variable_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				JS_VARIABLE_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				JS_VARIABLE_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_VARIABLE_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_while_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_WHILE_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_WHILE_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_WHILE_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_with_statement(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				JS_WITH_STATEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				JS_WITH_STATEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_WITH_STATEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_js_yield_expression(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				JS_YIELD_EXPRESSION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				JS_YIELD_EXPRESSION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(JS_YIELD_EXPRESSION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_new_expr(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(NEW_EXPR.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				NEW_EXPR.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(NEW_EXPR, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_new_target(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(NEW_TARGET.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				NEW_TARGET.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(NEW_TARGET, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_specifier(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(SPECIFIER.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				SPECIFIER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(SPECIFIER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_template(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(TEMPLATE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TEMPLATE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TEMPLATE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_template_chunk_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				TEMPLATE_CHUNK_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TEMPLATE_CHUNK_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TEMPLATE_CHUNK_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_template_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TEMPLATE_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TEMPLATE_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TEMPLATE_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_any(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_ANY.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_ANY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_ANY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_array(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_ARRAY.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_ARRAY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_ARRAY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_assertion(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(TS_ASSERTION.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_ASSERTION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_ASSERTION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_bigint(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_BIGINT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_BIGINT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_BIGINT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_boolean(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_BOOLEAN.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_BOOLEAN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_BOOLEAN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_call_signature_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_CALL_SIGNATURE_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_CALL_SIGNATURE_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CALL_SIGNATURE_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_conditional_type(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_CONDITIONAL_TYPE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_CONDITIONAL_TYPE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONDITIONAL_TYPE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_const_assertion(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_CONST_ASSERTION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_CONST_ASSERTION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONST_ASSERTION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_constraint(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_CONSTRAINT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_CONSTRAINT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONSTRAINT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_construct_signature_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_CONSTRUCT_SIGNATURE_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_CONSTRUCT_SIGNATURE_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONSTRUCT_SIGNATURE_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_constructor_param(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TS_CONSTRUCTOR_PARAM.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_CONSTRUCTOR_PARAM.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONSTRUCTOR_PARAM, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_constructor_type(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_CONSTRUCTOR_TYPE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_CONSTRUCTOR_TYPE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_CONSTRUCTOR_TYPE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_default(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_DEFAULT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_DEFAULT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_DEFAULT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_enum(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(TS_ENUM.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				TS_ENUM.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_ENUM, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_enum_member(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_ENUM_MEMBER.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_ENUM_MEMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_ENUM_MEMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_export_assignment(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_EXPORT_ASSIGNMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_EXPORT_ASSIGNMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_EXPORT_ASSIGNMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_expr_with_type_args(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				TS_EXPR_WITH_TYPE_ARGS.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_EXPR_WITH_TYPE_ARGS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_EXPR_WITH_TYPE_ARGS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_extends(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_EXTENDS.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_EXTENDS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_EXTENDS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_external_module_ref(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_EXTERNAL_MODULE_REF.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_EXTERNAL_MODULE_REF.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_EXTERNAL_MODULE_REF, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_fn_type(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_FN_TYPE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_FN_TYPE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_FN_TYPE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_implements_clause(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				TS_IMPLEMENTS_CLAUSE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_IMPLEMENTS_CLAUSE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_IMPLEMENTS_CLAUSE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_import(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(TS_IMPORT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				TS_IMPORT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_IMPORT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_import_equals_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				TS_IMPORT_EQUALS_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				TS_IMPORT_EQUALS_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_IMPORT_EQUALS_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_index_signature(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(
				TS_INDEX_SIGNATURE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				TS_INDEX_SIGNATURE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_INDEX_SIGNATURE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_indexed_array(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TS_INDEXED_ARRAY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_INDEXED_ARRAY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_INDEXED_ARRAY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_infer(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_INFER.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_INFER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_INFER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_interface_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 8usize {
			return RawSyntaxNode::new(
				TS_INTERFACE_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 8usize] = Default::default();
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
				TS_INTERFACE_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_INTERFACE_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_intersection(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				TS_INTERSECTION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_INTERSECTION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_INTERSECTION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_literal(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_LITERAL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_LITERAL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_LITERAL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_mapped_type(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 10usize {
			return RawSyntaxNode::new(TS_MAPPED_TYPE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 10usize] = Default::default();
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
				TS_MAPPED_TYPE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_MAPPED_TYPE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_mapped_type_param(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_MAPPED_TYPE_PARAM.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_MAPPED_TYPE_PARAM.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_MAPPED_TYPE_PARAM, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_mapped_type_readonly(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TS_MAPPED_TYPE_READONLY.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_MAPPED_TYPE_READONLY.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_MAPPED_TYPE_READONLY, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_method_signature(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 7usize {
			return RawSyntaxNode::new(
				TS_METHOD_SIGNATURE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 7usize] = Default::default();
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
				TS_METHOD_SIGNATURE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_METHOD_SIGNATURE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_module_block(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TS_MODULE_BLOCK.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_MODULE_BLOCK.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_MODULE_BLOCK, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_module_decl(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 6usize {
			return RawSyntaxNode::new(TS_MODULE_DECL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 6usize] = Default::default();
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
				TS_MODULE_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_MODULE_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_namespace_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_NAMESPACE_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_NAMESPACE_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NAMESPACE_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_namespace_export_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_NAMESPACE_EXPORT_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_NAMESPACE_EXPORT_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NAMESPACE_EXPORT_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_never(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_NEVER.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_NEVER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NEVER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_non_null(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_NON_NULL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_NON_NULL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NON_NULL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_null(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_NULL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_NULL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NULL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_number(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_NUMBER.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_NUMBER.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_NUMBER, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_object(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_OBJECT.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_OBJECT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_OBJECT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_object_type(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_OBJECT_TYPE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_OBJECT_TYPE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_OBJECT_TYPE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_paren(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_PAREN.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_PAREN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_PAREN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_predicate(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_PREDICATE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_PREDICATE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_PREDICATE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_property_signature(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_PROPERTY_SIGNATURE.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_PROPERTY_SIGNATURE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_PROPERTY_SIGNATURE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_qualified_path(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(
				TS_QUALIFIED_PATH.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_QUALIFIED_PATH.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_QUALIFIED_PATH, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_string(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_STRING.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_STRING.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_STRING, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_symbol(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_SYMBOL.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_SYMBOL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_SYMBOL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_template(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_TEMPLATE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_TEMPLATE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TEMPLATE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_template_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				TS_TEMPLATE_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_TEMPLATE_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TEMPLATE_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_this(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_THIS.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_THIS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_THIS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_tuple(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_TUPLE.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_TUPLE.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TUPLE, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_tuple_element(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 5usize {
			return RawSyntaxNode::new(
				TS_TUPLE_ELEMENT.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 5usize] = Default::default();
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
				TS_TUPLE_ELEMENT.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TUPLE_ELEMENT, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_alias_decl(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 4usize {
			return RawSyntaxNode::new(
				TS_TYPE_ALIAS_DECL.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 4usize] = Default::default();
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
				TS_TYPE_ALIAS_DECL.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_ALIAS_DECL, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_annotation(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(
				TS_TYPE_ANNOTATION.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_TYPE_ANNOTATION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_ANNOTATION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_args(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_TYPE_ARGS.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_TYPE_ARGS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_ARGS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_name(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_TYPE_NAME.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_TYPE_NAME.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_NAME, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_operator(
		children: ParsedChildren<JsSyntaxKind>,
	) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(
				TS_TYPE_OPERATOR.to_unknown(),
				children.into_iter().map(Some),
			);
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_TYPE_OPERATOR.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_OPERATOR, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_param(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_TYPE_PARAM.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_TYPE_PARAM.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_PARAM, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_params(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 3usize {
			return RawSyntaxNode::new(TS_TYPE_PARAMS.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 3usize] = Default::default();
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
				TS_TYPE_PARAMS.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_PARAMS, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_type_ref(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 2usize {
			return RawSyntaxNode::new(TS_TYPE_REF.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 2usize] = Default::default();
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
				TS_TYPE_REF.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_TYPE_REF, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_undefined(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_UNDEFINED.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_UNDEFINED.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_UNDEFINED, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_union(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_UNION.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_UNION.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_UNION, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_unknown(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_UNKNOWN.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_UNKNOWN.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_UNKNOWN, slots)
	}
	#[allow(unused_mut)]
	#[allow(unused_assignments)]
	#[allow(unused_variables)]
	fn make_ts_void(children: ParsedChildren<JsSyntaxKind>) -> RawSyntaxNode<JsSyntaxKind> {
		let actual_len = children.len();
		if actual_len > 1usize {
			return RawSyntaxNode::new(TS_VOID.to_unknown(), children.into_iter().map(Some));
		}
		let mut elements = children.into_iter();
		let mut current_slot_index = 0;
		let mut slots: [Option<RawSyntaxElement<JsSyntaxKind>>; 1usize] = Default::default();
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
				TS_VOID.to_unknown(),
				UnknownNodeChildrenIterator::new(slots, actual_len, element, elements),
			);
		}
		RawSyntaxNode::new(TS_VOID, slots)
	}
}
