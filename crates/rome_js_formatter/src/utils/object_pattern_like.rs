use crate::builders::format_delimited;
use crate::prelude::*;
use crate::JsFormatContext;
use rome_formatter::formatter::Formatter;
use rome_formatter::write;
use rome_formatter::{Format, FormatResult};
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyBindingPattern, JsAnyObjectAssignmentPatternMember,
    JsAnyObjectBindingPatternMember, JsObjectAssignmentPattern, JsObjectBindingPattern,
    JsSyntaxKind, JsSyntaxToken,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub (crate) JsObjectPatternLike = JsObjectAssignmentPattern | JsObjectBindingPattern
}

impl JsObjectPatternLike {
    fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectPatternLike::JsObjectAssignmentPattern(node) => node.l_curly_token(),
            JsObjectPatternLike::JsObjectBindingPattern(node) => node.l_curly_token(),
        }
    }

    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectPatternLike::JsObjectAssignmentPattern(node) => node.r_curly_token(),
            JsObjectPatternLike::JsObjectBindingPattern(node) => node.r_curly_token(),
        }
    }

    fn properties_len(&self) -> usize {
        match self {
            JsObjectPatternLike::JsObjectAssignmentPattern(node) => node.properties().len(),
            JsObjectPatternLike::JsObjectBindingPattern(node) => node.properties().len(),
        }
    }

    fn write_properties(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsObjectPatternLike::JsObjectAssignmentPattern(node) => {
                write!(f, [node.properties().format()])
            }
            JsObjectPatternLike::JsObjectBindingPattern(node) => {
                write!(f, [node.properties().format()])
            }
        }
    }

    fn should_break_properties(&self) -> SyntaxResult<bool> {
        let has_at_least_a_complex_property = match self {
            JsObjectPatternLike::JsObjectAssignmentPattern(node) => {
                node.properties().iter().any(|property| {
                    if let Ok(
                        JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node),
                    ) = property
                    {
                        let pattern = node.pattern();
                        matches!(
                            pattern,
                            Ok(JsAnyAssignmentPattern::JsObjectAssignmentPattern(_)
                                | JsAnyAssignmentPattern::JsArrayAssignmentPattern(_))
                        )
                    } else {
                        false
                    }
                })
            }
            JsObjectPatternLike::JsObjectBindingPattern(node) => {
                node.properties().iter().any(|property| {
                    if let Ok(JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(
                        node,
                    )) = property
                    {
                        let pattern = node.pattern();
                        matches!(
                            pattern,
                            Ok(JsAnyBindingPattern::JsObjectBindingPattern(_)
                                | JsAnyBindingPattern::JsArrayBindingPattern(_))
                        )
                    } else {
                        false
                    }
                })
            }
        };

        let parent_kind = self.syntax().parent().map(|p| p.kind());

        let parent_where_not_to_break = !matches!(
            parent_kind,
            Some(
                // These parents are the kinds where we want to prevent
                // to go to multiple lines.
                JsSyntaxKind::JS_FUNCTION_EXPRESSION
                    | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                    | JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
                    | JsSyntaxKind::JS_CATCH_DECLARATION
                    | JsSyntaxKind::JS_FUNCTION_DECLARATION
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY
                    | JsSyntaxKind::JS_FORMAL_PARAMETER
            )
        );

        Ok(parent_where_not_to_break && has_at_least_a_complex_property)
    }

    fn is_in_assignment_like(&self) -> bool {
        if let JsObjectPatternLike::JsObjectAssignmentPattern(pattern) = self {
            let parent_kind = pattern.syntax().parent().map(|p| p.kind());
            matches!(
                parent_kind,
                Some(JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION | JsSyntaxKind::JS_VARIABLE_DECLARATOR)
            )
        } else {
            false
        }
    }
}

impl Format<JsFormatContext> for JsObjectPatternLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let should_break_properties = self.should_break_properties()?;
        let right = &format_with(|f| self.write_properties(f));
        let is_in_assignment_like = self.is_in_assignment_like();
        let properties_len = self.properties_len();

        if properties_len == 0 {
            write!(
                f,
                [
                    self.l_curly_token().format(),
                    format_dangling_comments(self.syntax()).with_soft_block_indent(),
                    self.r_curly_token().format()
                ]
            )
        } else if should_break_properties {
            write!(
                f,
                [
                    self.l_curly_token().format(),
                    block_indent(right),
                    self.r_curly_token().format(),
                ]
            )
        } else if !should_break_properties && is_in_assignment_like {
            // no need to add a group if we know the parent already does that
            write!(f, [self.l_curly_token()?.format()])?;
            if properties_len > 0 {
                write!(f, [soft_line_break_or_space()])?;
                write!(f, [soft_block_indent(right)])?;
                write!(f, [soft_line_break_or_space()])?;
            } else {
                write!(f, [right])?;
            }
            write!(f, [self.r_curly_token().format()])
        } else {
            write!(
                f,
                [
                    format_delimited(&self.l_curly_token()?, right, &self.r_curly_token()?)
                        .soft_block_spaces()
                ]
            )
        }
    }
}
