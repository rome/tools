//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_implicit_boolean;
mod no_negation_else;
mod no_unused_template_literal;
mod prefer_numeric_literals;
mod use_block_statements;
mod use_optional_chain;
mod use_self_closing_elements;
mod use_shorthand_array_type;
mod use_single_var_declarator;
mod use_template;
declare_group! { pub (crate) Style { name : "style" , rules : [self :: no_implicit_boolean :: NoImplicitBoolean , self :: no_negation_else :: NoNegationElse , self :: no_unused_template_literal :: NoUnusedTemplateLiteral , self :: prefer_numeric_literals :: PreferNumericLiterals , self :: use_block_statements :: UseBlockStatements , self :: use_optional_chain :: UseOptionalChain , self :: use_self_closing_elements :: UseSelfClosingElements , self :: use_shorthand_array_type :: UseShorthandArrayType , self :: use_single_var_declarator :: UseSingleVarDeclarator , self :: use_template :: UseTemplate ,] } }
