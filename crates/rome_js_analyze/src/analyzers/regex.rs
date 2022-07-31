//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_multiple_spaces_in_regular_expression_literals;
declare_group! { pub (crate) Regex { name : "regex" , rules : [self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals ,] } }
