//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing the [crate::FormatNodeRule] trait.

pub(crate) use crate::{
    builders::format_or_verbatim, comments::JsComments, AsFormat as _, FormatNodeRule,
    FormattedIterExt, JsFormatContext, JsFormatter,
};
pub use rome_formatter::prelude::*;
pub use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub use crate::separated::{
    FormatAstSeparatedListExtension, FormatSeparatedOptions, TrailingSeparator,
};
