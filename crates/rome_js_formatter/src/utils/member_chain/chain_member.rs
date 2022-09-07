use crate::context::TabWidth;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsCallExpressionFields, JsComputedMemberExpression,
    JsComputedMemberExpressionFields, JsIdentifierExpression, JsImportCallExpression,
    JsNewExpression, JsStaticMemberExpression, JsStaticMemberExpressionFields, JsSyntaxNode,
    JsThisExpression,
};
use rome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;

/// Data structure that holds the node with its formatted version
#[derive(Clone, Debug)]
pub(crate) enum ChainMember {
    /// Holds onto a [rome_js_syntax::JsStaticMemberExpression]
    StaticMember {
        expression: JsStaticMemberExpression,
        root: bool,
    },

    /// Holds onto a [rome_js_syntax::JsCallExpression]
    CallExpression {
        expression: JsCallExpression,
        root: bool,
    },

    /// Holds onto a [rome_js_syntax::JsComputedMemberExpression]
    ComputedMember {
        expression: JsComputedMemberExpression,
        root: bool,
    },

    /// Any other node that are not  [rome_js_syntax::JsCallExpression] or [rome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(JsSyntaxNode),
}

impl ChainMember {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            ChainMember::CallExpression { .. } => true,
            ChainMember::Node(node) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsNewExpression::can_cast(node.kind())
            }
            _ => false,
        }
    }

    pub(crate) fn syntax(&self) -> &JsSyntaxNode {
        match self {
            ChainMember::StaticMember { expression, .. } => expression.syntax(),
            ChainMember::CallExpression { expression, .. } => expression.syntax(),
            ChainMember::ComputedMember { expression, .. } => expression.syntax(),
            ChainMember::Node(node) => node,
        }
    }

    pub const fn is_computed_expression(&self) -> bool {
        matches!(self, ChainMember::ComputedMember { .. })
    }

    pub(crate) fn is_this_expression(&self) -> bool {
        match self {
            ChainMember::Node(node) => JsThisExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    pub(crate) fn is_identifier_expression(&self) -> bool {
        match self {
            ChainMember::Node(node) => JsIdentifierExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    /// There are cases like Object.keys(), Observable.of(), _.values() where
    /// they are the subject of all the chained calls and therefore should
    /// be kept on the same line:
    ///
    /// ```js
    ///   Object.keys(items)
    ///     .filter(x => x)
    ///     .map(x => x)
    /// ```
    /// In order to detect those cases, we use an heuristic: if the first
    /// node is an identifier with the name starting with a capital
    /// letter or just a sequence of _$. The rationale is that they are
    /// likely to be factories.
    ///
    /// Comment from [Prettier]
    ///
    /// [Prettier]: https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js#L252-L266
    pub(crate) fn is_factory(&self, check_left_hand_side: bool) -> SyntaxResult<bool> {
        fn check_str(text: &str) -> bool {
            text.chars().next().map_or(false, |c| c.is_uppercase())
                || text.starts_with('_')
                || text.starts_with('$')
        }

        if let ChainMember::StaticMember { expression, .. } = self {
            if check_left_hand_side {
                if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                    expression.object()?
                {
                    let value_token = identifier_expression.name()?.value_token()?;
                    let text = value_token.text_trimmed();
                    Ok(check_str(text))
                } else {
                    Ok(false)
                }
            } else {
                Ok(check_str(expression.member()?.text().as_str()))
            }
        } else if let ChainMember::Node(node, ..) = self {
            if let Some(identifier_expression) = JsIdentifierExpression::cast(node.clone()) {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(check_str(text))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub(crate) fn has_short_name(&self, tab_width: TabWidth) -> SyntaxResult<bool> {
        if let ChainMember::StaticMember { expression, .. } = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                expression.object()?
            {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(text.len() <= u8::from(tab_width) as usize)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Format<JsFormatContext> for ChainMember {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            ChainMember::StaticMember { expression, root } => {
                let JsStaticMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    operator_token,
                    member,
                } = expression.as_fields();
                write![
                    f,
                    [
                        (!root).then_some(format_leading_comments(expression.syntax())),
                        operator_token.format(),
                        member.format(),
                        (!root).then_some(format_trailing_comments(expression.syntax()))
                    ]
                ]
            }

            ChainMember::CallExpression { expression, root } => {
                let JsCallExpressionFields {
                    // Formatted as part of the previous item
                    callee: _,
                    optional_chain_token,
                    type_arguments,
                    arguments,
                } = expression.as_fields();

                write!(
                    f,
                    [
                        (!root).then_some(format_leading_comments(expression.syntax())),
                        optional_chain_token.format(),
                        type_arguments.format(),
                        arguments.format(),
                        (!root).then_some(format_trailing_comments(expression.syntax()))
                    ]
                )
            }
            ChainMember::ComputedMember { expression, root } => {
                let JsComputedMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    optional_chain_token,
                    l_brack_token,
                    member,
                    r_brack_token,
                } = expression.as_fields();
                write!(
                    f,
                    [
                        (!root).then_some(format_leading_comments(expression.syntax())),
                        optional_chain_token.format(),
                        l_brack_token.format(),
                        member.format(),
                        r_brack_token.format(),
                        (!root).then_some(format_trailing_comments(expression.syntax()))
                    ]
                )
            }
            ChainMember::Node(node) => {
                write!(f, [node.format()])
            }
        }
    }
}
