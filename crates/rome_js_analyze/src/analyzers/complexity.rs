//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_extra_boolean_cast;
mod use_simplified_logic_expression;
declare_group! { pub (crate) Complexity { name : "complexity" , rules : [self :: no_extra_boolean_cast :: NoExtraBooleanCast , self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression ,] } }
