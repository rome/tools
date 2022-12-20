//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing a syntax formatter.

pub(crate) use crate::{
    comments::JsComments, AsFormat as _, FormatNodeRule, FormattedIterExt, JsFormatContext,
    JsFormatter,
};
pub use rome_formatter::prelude::*;
pub use rome_formatter::separated::TrailingSeparator;
pub use rome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub(crate) use crate::separated::FormatAstSeparatedListExtension;
