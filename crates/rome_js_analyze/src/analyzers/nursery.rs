//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_auto_focus;
mod no_new_symbol;
mod no_unreachable;
mod use_blank_target;
mod use_optional_chain;
mod use_valid_anchor;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_auto_focus :: NoAutoFocus , self :: no_new_symbol :: NoNewSymbol , self :: no_unreachable :: NoUnreachable , self :: use_blank_target :: UseBlankTarget , self :: use_optional_chain :: UseOptionalChain , self :: use_valid_anchor :: UseValidAnchor ,] } }
