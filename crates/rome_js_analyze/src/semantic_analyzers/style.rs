//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_shouty_constants;
mod use_camel_case;
mod use_fragment_syntax;
declare_group! { pub (crate) Style { name : "style" , rules : [self :: no_shouty_constants :: NoShoutyConstants , self :: use_camel_case :: UseCamelCase , self :: use_fragment_syntax :: UseFragmentSyntax ,] } }
