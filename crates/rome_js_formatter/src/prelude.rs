//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing the [crate::FormatNodeRule] trait.

pub(crate) use crate::{
    AsFormat as _, FormatNodeRule, FormattedIterExt, JsFormatContext, JsFormatter,
};
pub use rome_formatter::prelude::*;
pub use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub use crate::builders::{
    format_or_verbatim, format_suppressed_node, format_unknown_node, format_verbatim_node,
};

pub use crate::separated::{
    FormatAstSeparatedListExtension, FormatSeparatedOptions, TrailingSeparator,
};
