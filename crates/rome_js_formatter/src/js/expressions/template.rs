use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{
    JsAnyExpression, JsSyntaxNode, JsSyntaxToken, JsTemplate, TsTemplateLiteralType,
    TsTypeArguments,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplate;

impl FormatNodeRule<JsTemplate> for FormatJsTemplate {
    fn fmt_fields(&self, node: &JsTemplate, f: &mut JsFormatter) -> FormatResult<()> {
        JsAnyTemplate::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsTemplate) -> bool {
        item.needs_parentheses()
    }
}

declare_node_union! {
    JsAnyTemplate = JsTemplate | TsTemplateLiteralType
}

impl Format<JsFormatContext> for JsAnyTemplate {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [
                self.tag().format(),
                self.type_arguments().format(),
                line_suffix_boundary(),
                self.l_tick_token().format(),
            ]
        )?;

        self.write_elements(f)?;

        write!(f, [self.r_tick_token().format()])
    }
}

impl JsAnyTemplate {
    fn tag(&self) -> Option<JsAnyExpression> {
        match self {
            JsAnyTemplate::JsTemplate(template) => template.tag(),
            JsAnyTemplate::TsTemplateLiteralType(_) => None,
        }
    }

    fn type_arguments(&self) -> Option<TsTypeArguments> {
        match self {
            JsAnyTemplate::JsTemplate(template) => template.type_arguments(),
            JsAnyTemplate::TsTemplateLiteralType(_) => None,
        }
    }

    fn l_tick_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyTemplate::JsTemplate(template) => template.l_tick_token(),
            JsAnyTemplate::TsTemplateLiteralType(template) => template.l_tick_token(),
        }
    }

    fn write_elements(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsAnyTemplate::JsTemplate(template) => {
                write!(f, [template.elements().format()])
            }
            JsAnyTemplate::TsTemplateLiteralType(template) => {
                write!(f, [template.elements().format()])
            }
        }
    }

    fn r_tick_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyTemplate::JsTemplate(template) => template.r_tick_token(),
            JsAnyTemplate::TsTemplateLiteralType(template) => template.r_tick_token(),
        }
    }
}

/// `TemplateLiteral`'s are `PrimaryExpression's that never need parentheses.
impl NeedsParentheses for JsTemplate {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
