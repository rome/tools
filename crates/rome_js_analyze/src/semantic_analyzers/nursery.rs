//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_accumulating_spread;
mod no_console_log;
mod no_constant_condition;
mod use_camel_case;
pub(crate) mod use_exhaustive_dependencies;
mod use_hook_at_top_level;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_accumulating_spread :: NoAccumulatingSpread , self :: no_console_log :: NoConsoleLog , self :: no_constant_condition :: NoConstantCondition , self :: use_camel_case :: UseCamelCase , self :: use_exhaustive_dependencies :: UseExhaustiveDependencies , self :: use_hook_at_top_level :: UseHookAtTopLevel ,] } }
