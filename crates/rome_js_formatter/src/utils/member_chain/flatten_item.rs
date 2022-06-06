use crate::prelude::*;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsComputedMemberExpression, JsIdentifierExpression,
    JsImportCallExpression, JsNewExpression, JsStaticMemberExpression, JsSyntaxNode,
    JsThisExpression,
};
use rome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;

#[derive(Clone)]
/// Data structure that holds the node with its formatted version
pub(crate) enum FlattenItem {
    /// Holds onto a [rome_js_syntax::JsStaticMemberExpression]
    StaticMember(JsStaticMemberExpression, Vec<FormatElement>),
    /// Holds onto a [rome_js_syntax::JsCallExpression]
    CallExpression(JsCallExpression, Vec<FormatElement>),
    /// Holds onto a [rome_js_syntax::JsComputedMemberExpression]
    ComputedExpression(JsComputedMemberExpression, Vec<FormatElement>),
    /// Any other node that are not  [rome_js_syntax::JsCallExpression] or [rome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(JsSyntaxNode, FormatElement),
}

impl FlattenItem {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            FlattenItem::CallExpression(_, _) => true,
            FlattenItem::Node(node, _) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsNewExpression::can_cast(node.kind())
            }
            _ => false,
        }
    }

    pub(crate) fn as_syntax(&self) -> &JsSyntaxNode {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax(),
            FlattenItem::CallExpression(node, _) => node.syntax(),
            FlattenItem::ComputedExpression(node, _) => node.syntax(),
            FlattenItem::Node(node, _) => node,
        }
    }

    pub(crate) fn has_trailing_comments(&self) -> bool {
        match self {
            FlattenItem::StaticMember(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::CallExpression(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::ComputedExpression(node, _) => node.syntax().has_trailing_comments(),
            FlattenItem::Node(node, _) => node.has_trailing_comments(),
        }
    }

    pub fn is_computed_expression(&self) -> bool {
        matches!(self, FlattenItem::ComputedExpression(..))
    }

    pub(crate) fn is_this_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node, _) => JsThisExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    pub(crate) fn is_identifier_expression(&self) -> bool {
        match self {
            FlattenItem::Node(node, _) => JsIdentifierExpression::can_cast(node.kind()),
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

    pub(crate) fn has_short_name(&self, tab_width: u8) -> SyntaxResult<bool> {
        if let FlattenItem::StaticMember(static_member, ..) = self {
            if let JsAnyExpression::JsIdentifierExpression(identifier_expression) =
                static_member.object()?
            {
                let value_token = identifier_expression.name()?.value_token()?;
                let text = value_token.text_trimmed();
                Ok(text.len() <= tab_width as usize)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Debug for FlattenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlattenItem::StaticMember(_, formatted) => {
                std::write!(f, "StaticMember: {:?}", formatted)
            }
            FlattenItem::CallExpression(_, formatted) => {
                std::write!(f, "CallExpression: {:?}", formatted)
            }
            FlattenItem::ComputedExpression(_, formatted) => {
                std::write!(f, "ComputedExpression: {:?}", formatted)
            }
            FlattenItem::Node(node, formatted) => {
                std::write!(f, "{:?} {:?}", node.kind(), formatted)
            }
        }
    }
}

impl From<FlattenItem> for FormatElement {
    fn from(flatten_item: FlattenItem) -> Self {
        match flatten_item {
            FlattenItem::StaticMember(_, formatted) => FormatElement::from_iter(formatted),
            FlattenItem::CallExpression(_, formatted) => FormatElement::from_iter(formatted),
            FlattenItem::ComputedExpression(_, formatted) => FormatElement::from_iter(formatted),
            FlattenItem::Node(_, formatted) => formatted,
        }
    }
}
