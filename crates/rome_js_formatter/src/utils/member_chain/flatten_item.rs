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

#[derive(Clone, Debug)]
/// Data structure that holds the node with its formatted version
pub(crate) enum FlattenItem {
    /// Holds onto a [rome_js_syntax::JsStaticMemberExpression]
    StaticMember(JsStaticMemberExpression),
    /// Holds onto a [rome_js_syntax::JsCallExpression]
    CallExpression(JsCallExpression),
    /// Holds onto a [rome_js_syntax::JsComputedMemberExpression]
    ComputedMember(JsComputedMemberExpression),
    /// Any other node that are not  [rome_js_syntax::JsCallExpression] or [rome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(JsSyntaxNode),
}

impl FlattenItem {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            FlattenItem::CallExpression(_) => true,
            FlattenItem::Node(node) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsNewExpression::can_cast(node.kind())
            }
            _ => false,
        }
    }

    pub(crate) fn as_syntax(&self) -> &JsSyntaxNode {
        match self {
            FlattenItem::StaticMember(node) => node.syntax(),
            FlattenItem::CallExpression(node) => node.syntax(),
            FlattenItem::ComputedMember(node) => node.syntax(),
            FlattenItem::Node(node) => node,
        }
    }

    pub(crate) fn has_trailing_comments(&self) -> bool {
        match self {
            FlattenItem::StaticMember(node) => node.syntax().has_trailing_comments(),
            FlattenItem::CallExpression(node) => node.syntax().has_trailing_comments(),
            FlattenItem::ComputedMember(node) => node.syntax().has_trailing_comments(),
            FlattenItem::Node(node) => node.has_trailing_comments(),
        }
    }

    pub fn is_computed_expression(&self) -> bool {
        matches!(self, FlattenItem::ComputedMember(..))
    }

    pub(crate) fn is_this_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node) => JsThisExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    pub(crate) fn is_identifier_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node) => JsIdentifierExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    pub(crate) fn has_leading_comments(&self) -> SyntaxResult<bool> {
        Ok(match self {
            FlattenItem::StaticMember(node) => {
                node.syntax().has_comments_direct() || node.operator_token()?.has_leading_comments()
            }
            FlattenItem::CallExpression(node) => node.syntax().has_leading_comments(),
            FlattenItem::ComputedMember(node) => node.syntax().has_leading_comments(),
            FlattenItem::Node(node) => node.has_leading_comments(),
        })
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

        if let FlattenItem::StaticMember(static_member, ..) = self {
            if check_left_hand_side {
                if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                    static_member.object()?
                {
                    let value_token = identifier_expression.name()?.value_token()?;
                    let text = value_token.text_trimmed();
                    Ok(check_str(text))
                } else {
                    Ok(false)
                }
            } else {
                Ok(check_str(static_member.member()?.text().as_str()))
            }
        } else if let FlattenItem::Node(node, ..) = self {
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
        if let FlattenItem::StaticMember(static_member, ..) = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                static_member.object()?
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

impl Format<JsFormatContext> for FlattenItem {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            FlattenItem::StaticMember(static_member) => {
                let JsStaticMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    operator_token,
                    member,
                } = static_member.as_fields();
                write![f, [operator_token.format(), member.format(),]]
            }
            FlattenItem::CallExpression(call_expression) => {
                let JsCallExpressionFields {
                    // Formatted as part of the previous item
                    callee: _,
                    optional_chain_token,
                    type_arguments,
                    arguments,
                } = call_expression.as_fields();

                write!(
                    f,
                    [
                        optional_chain_token.format(),
                        type_arguments.format(),
                        arguments.format()
                    ]
                )
            }
            FlattenItem::ComputedMember(computed_member) => {
                let JsComputedMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    optional_chain_token,
                    l_brack_token,
                    member,
                    r_brack_token,
                } = computed_member.as_fields();
                write!(
                    f,
                    [
                        optional_chain_token.format(),
                        l_brack_token.format(),
                        member.format(),
                        r_brack_token.format(),
                    ]
                )
            }
            FlattenItem::Node(node) => {
                write!(f, [node.format()])
            }
        }
    }
}
