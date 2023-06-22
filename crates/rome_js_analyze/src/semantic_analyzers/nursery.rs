//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
pub(crate) mod no_accumulating_spread;
pub(crate) mod no_banned_types;
pub(crate) mod no_console_log;
pub(crate) mod no_constant_condition;
pub(crate) mod use_camel_case;
pub(crate) mod use_exhaustive_dependencies;
pub(crate) mod use_hook_at_top_level;
pub(crate) mod use_naming_convention;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_accumulating_spread :: NoAccumulatingSpread , self :: no_banned_types :: NoBannedTypes , self :: no_console_log :: NoConsoleLog , self :: no_constant_condition :: NoConstantCondition , self :: use_camel_case :: UseCamelCase , self :: use_exhaustive_dependencies :: UseExhaustiveDependencies , self :: use_hook_at_top_level :: UseHookAtTopLevel , self :: use_naming_convention :: UseNamingConvention ,] } }
