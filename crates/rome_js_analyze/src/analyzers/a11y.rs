//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod use_anchor_content;
mod use_blank_target;
mod use_key_with_click_events;
mod use_key_with_mouse_events;
mod use_valid_anchor;
declare_group! { pub (crate) A11y { name : "a11y" , rules : [self :: use_anchor_content :: UseAnchorContent , self :: use_blank_target :: UseBlankTarget , self :: use_key_with_click_events :: UseKeyWithClickEvents , self :: use_key_with_mouse_events :: UseKeyWithMouseEvents , self :: use_valid_anchor :: UseValidAnchor ,] } }
