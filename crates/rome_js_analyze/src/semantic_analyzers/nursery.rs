//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_const_assign;
mod no_explicit_any;
mod use_exhaustive_dependencies;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_const_assign :: NoConstAssign , self :: no_explicit_any :: NoExplicitAny , self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,] } }
