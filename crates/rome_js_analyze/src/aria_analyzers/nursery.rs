//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_aria_unsupported_elements;
mod no_noninteractive_element_to_interactive_role;
mod no_noninteractive_tabindex;
mod no_redundant_roles;
mod use_aria_prop_types;
mod use_aria_props_for_role;
mod use_valid_aria_props;
mod use_valid_lang;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_aria_unsupported_elements :: NoAriaUnsupportedElements , self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: no_noninteractive_tabindex :: NoNoninteractiveTabindex , self :: no_redundant_roles :: NoRedundantRoles , self :: use_aria_prop_types :: UseAriaPropTypes , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_valid_aria_props :: UseValidAriaProps , self :: use_valid_lang :: UseValidLang ,] } }
