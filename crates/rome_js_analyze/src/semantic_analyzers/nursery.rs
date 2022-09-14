//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_danger;
mod no_unused_variables;
mod use_camel_case;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_danger :: NoDanger , self :: no_unused_variables :: NoUnusedVariables , self :: use_camel_case :: UseCamelCase ,] } }
