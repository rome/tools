//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_positive_tabindex;
mod use_button_type;
mod use_iframe_title;
declare_group! { pub (crate) A11y { name : "a11y" , rules : [self :: no_positive_tabindex :: NoPositiveTabindex , self :: use_button_type :: UseButtonType , self :: use_iframe_title :: UseIframeTitle ,] } }
