//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_unused_variables;
declare_group! { pub (crate) NoUnusedVariables { name : "no_unused_variables" , rules : [self :: no_unused_variables :: NoUnusedVariables ,] } }
