//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_noninteractive_element_to_interactive_role;
mod use_aria_props_for_role;
mod use_valid_aria_props;
mod use_valid_lang;
declare_group! { pub (crate) A11y { name : "a11y" , rules : [self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_valid_aria_props :: UseValidAriaProps , self :: use_valid_lang :: UseValidLang ,] } }
