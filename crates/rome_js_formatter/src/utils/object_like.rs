use crate::prelude::*;
use crate::utils::node_has_leading_newline;
use crate::JsFormatContext;
use rome_formatter::write;
use rome_formatter::{Format, FormatResult};
use rome_js_syntax::{JsObjectExpression, JsSyntaxToken, TsObjectType};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, SyntaxResult};

declare_node_union! {
    pub (crate) JsObjectLike = JsObjectExpression | TsObjectType
}
impl JsObjectLike {
    fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.l_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.l_curly_token(),
        }
    }
    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.r_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.r_curly_token(),
        }
    }

    fn members_have_leading_newline(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => node_has_leading_newline(oe.members().syntax()),
            JsObjectLike::TsObjectType(ot) => node_has_leading_newline(ot.members().syntax()),
        }
    }

    fn members_are_empty(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.members().is_empty(),
            JsObjectLike::TsObjectType(ot) => ot.members().is_empty(),
        }
    }

    fn write_members(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => {
                write!(f, [oe.members().format()])
            }
            JsObjectLike::TsObjectType(ot) => {
                write!(f, [ot.members().format()])
            }
        }
    }
}

impl Format<JsFormatContext> for JsObjectLike {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let members = format_with(|f| self.write_members(f));

        write!(f, [self.l_curly_token().format(),])?;

        if self.members_are_empty() {
            write!(
                f,
                [format_dangling_comments(self.syntax()).with_block_indent(),]
            )?;
        } else if self.members_have_leading_newline() {
            write!(f, [block_indent(&members)])?;
        } else {
            write!(f, [group(&soft_line_indent_or_spaced(&members))])?;
        }

        write!(f, [self.r_curly_token().format()])
    }
}
