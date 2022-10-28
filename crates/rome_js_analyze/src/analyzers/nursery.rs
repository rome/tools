//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_auto_focus;
mod no_banned_types;
mod no_explicit_any;
mod no_invalid_constructor_super;
mod no_new_symbol;
mod no_unreachable;
mod use_alt_text;
mod use_anchor_content;
mod use_blank_target;
mod use_flat_map;
mod use_key_with_click_events;
mod use_key_with_mouse_events;
mod use_optional_chain;
mod use_valid_anchor;
mod use_valid_for_direction;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_auto_focus :: NoAutoFocus , self :: no_banned_types :: NoBannedTypes , self :: no_explicit_any :: NoExplicitAny , self :: no_invalid_constructor_super :: NoInvalidConstructorSuper , self :: no_new_symbol :: NoNewSymbol , self :: no_unreachable :: NoUnreachable , self :: use_alt_text :: UseAltText , self :: use_anchor_content :: UseAnchorContent , self :: use_blank_target :: UseBlankTarget , self :: use_flat_map :: UseFlatMap , self :: use_key_with_click_events :: UseKeyWithClickEvents , self :: use_key_with_mouse_events :: UseKeyWithMouseEvents , self :: use_optional_chain :: UseOptionalChain , self :: use_valid_anchor :: UseValidAnchor , self :: use_valid_for_direction :: UseValidForDirection ,] } }
