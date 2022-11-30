//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_restricted_globals;
mod no_var;
mod use_camel_case;
mod use_const;
mod use_exhaustive_dependencies;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_restricted_globals :: NoRestrictedGlobals , self :: no_var :: NoVar , self :: use_camel_case :: UseCamelCase , self :: use_const :: UseConst , self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,] } }
