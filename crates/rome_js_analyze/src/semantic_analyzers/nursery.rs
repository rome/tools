//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_class_assign;
mod no_restricted_globals;
mod no_var;
mod use_camel_case;
mod use_const;
mod use_exhaustive_dependencies;
mod use_hook_at_top_level;
mod use_iframe_title;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_class_assign :: NoClassAssign , self :: no_restricted_globals :: NoRestrictedGlobals , self :: no_var :: NoVar , self :: use_camel_case :: UseCamelCase , self :: use_const :: UseConst , self :: use_exhaustive_dependencies :: UseExhaustiveDependencies , self :: use_hook_at_top_level :: UseHookAtTopLevel , self :: use_iframe_title :: UseIframeTitle ,] } }
