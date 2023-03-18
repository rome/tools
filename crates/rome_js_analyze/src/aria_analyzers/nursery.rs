//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_noninteractive_element_to_interactive_role;
<<<<<<< HEAD
mod no_redundant_roles;
=======
mod no_noninteractive_tabindex;
>>>>>>> e910953dca (feat(rome_js_analyze): noNoninteractiveTabindex)
mod use_aria_prop_types;
mod use_aria_props_for_role;
mod use_valid_aria_props;
mod use_valid_lang;
<<<<<<< HEAD
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: no_redundant_roles :: NoRedundantRoles , self :: use_aria_prop_types :: UseAriaPropTypes , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_valid_aria_props :: UseValidAriaProps , self :: use_valid_lang :: UseValidLang ,] } }
=======
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: no_noninteractive_tabindex :: NoNoninteractiveTabindex , self :: use_aria_prop_types :: UseAriaPropTypes , self :: use_aria_props_for_role :: UseAriaPropsForRole , self :: use_valid_aria_props :: UseValidAriaProps , self :: use_valid_lang :: UseValidLang ,] } }
>>>>>>> e910953dca (feat(rome_js_analyze): noNoninteractiveTabindex)
