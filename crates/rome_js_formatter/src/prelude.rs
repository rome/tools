//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing the [crate::FormatNode] trait.

pub(crate) use crate::{
    AsFormat as _, FormatNodeRule, FormattedIterExt, JsFormatContext, JsFormatter,
};
pub use rome_formatter::prelude::*;
pub use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub use crate::builders::{
    format_delimited, format_or_verbatim, format_removed, format_replaced, format_suppressed_node,
    format_trimmed_token, format_unknown_node, format_verbatim_node,
};

pub(crate) use crate::trivia::{format_leading_trivia, format_trailing_trivia};

pub use crate::separated::{
    FormatAstSeparatedListExtension, FormatSeparatedOptions, TrailingSeparator,
};
