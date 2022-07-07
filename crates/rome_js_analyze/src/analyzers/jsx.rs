//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_comment_text;
mod no_implicit_boolean;
mod use_self_closing_elements;
declare_group! { pub (crate) Jsx { name : "jsx" , rules : [no_comment_text :: NoCommentText , no_implicit_boolean :: NoImplicitBoolean , use_self_closing_elements :: UseSelfClosingElements ,] } }
