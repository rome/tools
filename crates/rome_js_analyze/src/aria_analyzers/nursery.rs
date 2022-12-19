//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod use_aria_prop_types;
mod use_aria_props_for_role;
mod use_valid_lang;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: use_aria_prop_types :: UseAriaPropTypes , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_valid_lang :: UseValidLang ,] } }
