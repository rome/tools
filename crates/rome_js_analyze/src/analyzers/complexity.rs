//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_extra_boolean_cast;
mod no_extra_semicolon;
mod no_multiple_spaces_in_regular_expression_literals;
mod no_useless_catch;
mod no_useless_constructor;
mod no_useless_label;
mod no_useless_rename;
mod no_useless_switch_case;
mod no_with;
mod use_flat_map;
mod use_optional_chain;
mod use_simplified_logic_expression;
declare_group! { pub (crate) Complexity { name : "complexity" , rules : [self :: no_extra_boolean_cast :: NoExtraBooleanCast , self :: no_extra_semicolon :: NoExtraSemicolon , self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals , self :: no_useless_catch :: NoUselessCatch , self :: no_useless_constructor :: NoUselessConstructor , self :: no_useless_label :: NoUselessLabel , self :: no_useless_rename :: NoUselessRename , self :: no_useless_switch_case :: NoUselessSwitchCase , self :: no_with :: NoWith , self :: use_flat_map :: UseFlatMap , self :: use_optional_chain :: UseOptionalChain , self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression ,] } }
