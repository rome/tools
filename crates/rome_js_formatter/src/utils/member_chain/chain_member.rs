use crate::context::TabWidth;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsCallExpressionFields, JsComputedMemberExpression,
    JsComputedMemberExpressionFields, JsIdentifierExpression, JsImportCallExpression,
    JsNewExpression, JsParenthesizedExpression, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsSyntaxNode, JsThisExpression,
};
use rome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;

/// One entry in a member chain.
#[derive(Clone, Debug)]
pub(crate) enum ChainEntry {
    /// A member that is parenthesized in the source document
    Parenthesized {
        /// The chain member
        member: ChainMember,
        /// The top most ancestor of the chain member that is a parenthesized expression.
        ///
        /// ```text
        /// (a.b).c()
        ///  ^^^ -> member
        /// ^----^ -> top_most_parentheses
        ///
        /// ((a.b)).c()
        ///   ^^^ -> member
        /// ^-----^ -> top most parentheses (skips inner parentheses node)
        /// ```
        top_most_parentheses: JsParenthesizedExpression,
    },
    Member(ChainMember),
}

impl ChainEntry {
    /// Returns the inner member
    pub fn member(&self) -> &ChainMember {
        match self {
            ChainEntry::Parenthesized { member, .. } => member,
            ChainEntry::Member(member) => member,
        }
    }

    /// Returns the top most parentheses node if any
    pub fn top_most_parentheses(&self) -> Option<&JsParenthesizedExpression> {
        match self {
            ChainEntry::Parenthesized {
                top_most_parentheses,
                ..
            } => Some(top_most_parentheses),
            ChainEntry::Member(_) => None,
        }
    }

    pub fn into_member(self) -> ChainMember {
        match self {
            ChainEntry::Parenthesized { member, .. } => member,
            ChainEntry::Member(member) => member,
        }
    }

    pub(crate) fn has_trailing_comments(&self) -> bool {
        self.nodes().any(|node| node.has_trailing_comments())
    }

    /// Returns true if the member any of it's ancestor parentheses nodes has any leading comments.
    pub(crate) fn has_leading_comments(&self) -> SyntaxResult<bool> {
        let has_operator_comment = match self.member() {
            ChainMember::StaticMember(node) => node.operator_token()?.has_leading_comments(),
            _ => false,
        };

        Ok(self.nodes().any(|node| node.has_leading_comments()) || has_operator_comment)
    }

    fn nodes(&self) -> impl Iterator<Item = JsSyntaxNode> {
        let first = match self {
            ChainEntry::Parenthesized {
                top_most_parentheses,
                ..
            } => top_most_parentheses.syntax().clone(),
            ChainEntry::Member(member) => member.syntax().clone(),
        };

        let is_parenthesized = matches!(self, ChainEntry::Parenthesized { .. });

        std::iter::successors(Some(first), move |previous| {
            if is_parenthesized {
                JsParenthesizedExpression::cast(previous.clone()).and_then(|parenthesized| {
                    parenthesized.expression().map(AstNode::into_syntax).ok()
                })
            } else {
                None
            }
        })
    }
}

impl Format<JsFormatContext> for ChainEntry {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let parentheses = self.top_most_parentheses();

        if let Some(parentheses) = parentheses {
            let mut current = parentheses.clone();

            loop {
                write!(f, [format_removed(&current.l_paren_token()?)])?;

                match current.expression()? {
                    JsAnyExpression::JsParenthesizedExpression(inner) => {
                        current = inner;
                    }
                    _ => break,
                }
            }
        }

        write!(f, [self.member()])?;

        if let Some(parentheses) = parentheses {
            let mut current = parentheses.clone();

            loop {
                write!(f, [format_removed(&current.r_paren_token()?)])?;

                match current.expression()? {
                    JsAnyExpression::JsParenthesizedExpression(inner) => {
                        current = inner;
                    }
                    _ => break,
                }
            }
        }

        Ok(())
    }
}

/// Data structure that holds the node with its formatted version
#[derive(Clone, Debug)]
pub(crate) enum ChainMember {
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

impl ChainMember {
    /// checks if the current node is a [rome_js_syntax::JsCallExpression],  [rome_js_syntax::JsImportExpression] or a [rome_js_syntax::JsNewExpression]
    pub fn is_loose_call_expression(&self) -> bool {
        match self {
            ChainMember::CallExpression(_) => true,
            ChainMember::Node(node) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsNewExpression::can_cast(node.kind())
            }
            _ => false,
        }
    }

    pub(crate) fn syntax(&self) -> &JsSyntaxNode {
        match self {
            ChainMember::StaticMember(node) => node.syntax(),
            ChainMember::CallExpression(node) => node.syntax(),
            ChainMember::ComputedMember(node) => node.syntax(),
            ChainMember::Node(node) => node,
        }
    }

    pub const fn is_computed_expression(&self) -> bool {
        matches!(self, ChainMember::ComputedMember(..))
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

        if let ChainMember::StaticMember(static_member, ..) = self {
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
        if let ChainMember::StaticMember(static_member, ..) = self {
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

impl Format<JsFormatContext> for ChainMember {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            ChainMember::StaticMember(static_member) => {
                let JsStaticMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    operator_token,
                    member,
                } = static_member.as_fields();
                write![f, [operator_token.format(), member.format()]]
            }
            ChainMember::CallExpression(call_expression) => {
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
            ChainMember::ComputedMember(computed_member) => {
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
            ChainMember::Node(node) => {
                write!(f, [node.format()])
            }
        }
    }
}
