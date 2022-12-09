//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_extra_boolean_cast;
mod no_multiple_spaces_in_regular_expression_literals;
mod use_flat_map;
mod use_optional_chain;
mod use_simplified_logic_expression;
declare_group! { pub (crate) Complexity { name : "complexity" , rules : [self :: no_extra_boolean_cast :: NoExtraBooleanCast , self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals , self :: use_flat_map :: UseFlatMap , self :: use_optional_chain :: UseOptionalChain , self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression ,] } }
