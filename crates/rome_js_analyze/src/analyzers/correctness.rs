//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_empty_pattern;
mod no_new_symbol;
mod no_unnecessary_continue;
mod no_unreachable;
mod use_valid_for_direction;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: no_empty_pattern :: NoEmptyPattern , self :: no_new_symbol :: NoNewSymbol , self :: no_unnecessary_continue :: NoUnnecessaryContinue , self :: no_unreachable :: NoUnreachable , self :: use_valid_for_direction :: UseValidForDirection ,] } }
