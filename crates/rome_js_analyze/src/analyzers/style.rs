//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_negation_else;
mod use_self_closing_elements;
mod use_shorthand_array_type;
declare_group! { pub (crate) Style { name : "style" , rules : [self :: no_negation_else :: NoNegationElse , self :: use_self_closing_elements :: UseSelfClosingElements , self :: use_shorthand_array_type :: UseShorthandArrayType ,] } }
