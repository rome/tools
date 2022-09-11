use crate::prelude::*;
use rome_formatter::{format_args, write, CstFormatContext, FormatRuleWithOptions};
use std::iter::once;

use crate::parentheses::{
    is_binary_like_left_or_right, is_callee, is_conditional_test,
    update_or_lower_expression_needs_parentheses, NeedsParentheses,
};
use crate::utils::{
    resolve_left_most_expression, AssignmentLikeLayout, JsAnyBinaryLikeLeftExpression,
};
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyBindingPattern, JsAnyExpression, JsAnyFormalParameter,
    JsAnyFunctionBody, JsAnyParameter, JsAnyTemplateElement, JsArrowFunctionExpression,
    JsSyntaxKind, JsSyntaxNode, JsTemplate,
};
use rome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrowFunctionExpression {
    assignment_layout: Option<AssignmentLikeLayout>,
}

impl FormatRuleWithOptions<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    type Options = Option<AssignmentLikeLayout>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.assignment_layout = options;
        self
    }
}

impl FormatNodeRule<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let layout = ArrowFunctionLayout::for_arrow(
            node.clone(),
            f.context().comments(),
            self.assignment_layout,
        )?;

        match layout {
            ArrowFunctionLayout::Chain(chain) => {
                write!(f, [chain])
            }
            ArrowFunctionLayout::Single(arrow) => {
                use self::JsAnyExpression::*;
                use JsAnyFunctionBody::*;

                let body = arrow.body()?;

                let format_signature = format_with(|f| {
                    write!(
                        f,
                        [
                            format_signature(&arrow),
                            space(),
                            arrow.fat_arrow_token().format()
                        ]
                    )
                });

                // With arrays, arrow selfs and objects, they have a natural line breaking strategy:
                // Arrays and objects become blocks:
                //
                //    [
                //      100000,
                //      200000,
                //      300000
                //    ]
                //
                // Arrow selfs get line broken after the `=>`:
                //
                //  (foo) => (bar) =>
                //     (foo + bar) * (foo + bar)
                //
                // Therefore if our body is an arrow self, array, or object, we
                // do not have a soft line break after the arrow because the body is
                // going to get broken anyways.
                let body_has_soft_line_break = match &body {
                    JsFunctionBody(_) => true,
                    JsAnyExpression(expr) => match expr {
                        JsArrowFunctionExpression(_)
                        | JsArrayExpression(_)
                        | JsObjectExpression(_)
                        | JsxTagExpression(_) => true,
                        JsTemplate(template) => {
                            is_multiline_template_starting_on_same_line(template)
                        }
                        JsSequenceExpression(_) => {
                            return write!(
                                f,
                                [group(&format_args![
                                    format_signature,
                                    group(&format_args![
                                        space(),
                                        text("("),
                                        soft_block_indent(&body.format()),
                                        text(")")
                                    ])
                                ])]
                            );
                        }
                        _ => false,
                    },
                };
                let body_has_leading_line_comment = f
                    .context()
                    .comments()
                    .has_leading_own_line_comment(body.syntax());

                // Add parentheses to avoid confusion between `a => b ? c : d` and `a <= b ? c : d`
                // but only if the body isn't an object/function or class expression because parentheses are always required in that
                // case and added by the object expression itself
                let should_add_parens = match &body {
                    JsAnyExpression(expression) => {
                        let is_conditional = matches!(expression, JsConditionalExpression(_));
                        let are_parentheses_mandatory = matches!(
                            resolve_left_most_expression(expression),
                            JsAnyBinaryLikeLeftExpression::JsAnyExpression(
                                JsObjectExpression(_)
                                    | JsFunctionExpression(_)
                                    | JsClassExpression(_)
                            )
                        );

                        is_conditional && !are_parentheses_mandatory
                    }
                    _ => false,
                };

                if body_has_soft_line_break && !should_add_parens && !body_has_leading_line_comment
                {
                    write![f, [format_signature, space(), body.format()]]
                } else {
                    write!(
                        f,
                        [
                            format_signature,
                            group(&soft_line_indent_or_space(&format_with(|f| {
                                if should_add_parens {
                                    write!(f, [if_group_fits_on_line(&text("("))])?;
                                }

                                write!(f, [body.format()])?;

                                if should_add_parens {
                                    write!(f, [if_group_fits_on_line(&text(")"))])?;
                                }

                                Ok(())
                            })))
                        ]
                    )
                }
            }
        }
    }

    fn needs_parentheses(&self, item: &JsArrowFunctionExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsArrowFunctionExpression,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// writes the arrow function type parameters, parameters, and return type annotation
fn format_signature(arrow: &JsArrowFunctionExpression) -> impl Format<JsFormatContext> + '_ {
    format_with(|f| {
        if let Some(async_token) = arrow.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        let format_parameters = format_with(|f| {
            write!(f, [arrow.type_parameters().format()])?;

            match arrow.parameters()? {
                JsAnyArrowFunctionParameters::JsAnyBinding(binding) => write!(
                    f,
                    [
                        text("("),
                        &soft_block_indent(&format_args![
                            binding.format(),
                            if_group_breaks(&text(","))
                        ]),
                        text(")")
                    ]
                )?,
                JsAnyArrowFunctionParameters::JsParameters(params) => {
                    write!(f, [params.format()])?;
                }
            };

            write!(f, [arrow.return_type_annotation().format()])
        });

        write!(f, [group(&format_parameters)])?;

        if f.comments().has_dangling_comments(arrow.syntax()) {
            write!(f, [space(), format_dangling_comments(arrow.syntax())])?;
        }

        Ok(())
    })
}

fn should_break_chain(arrow: &JsArrowFunctionExpression) -> SyntaxResult<bool> {
    if arrow.type_parameters().is_some() {
        return Ok(true);
    }

    let parameters = arrow.parameters()?;

    let has_parameters = match &parameters {
        JsAnyArrowFunctionParameters::JsAnyBinding(_) => true,
        JsAnyArrowFunctionParameters::JsParameters(parameters) => !parameters.items().is_empty(),
    };

    if arrow.return_type_annotation().is_some() && has_parameters {
        return Ok(true);
    }

    // Break if the function has any rest, object, or array parameter
    let result = match parameters {
        JsAnyArrowFunctionParameters::JsAnyBinding(_) => false,
        JsAnyArrowFunctionParameters::JsParameters(parameters) => parameters
            .items()
            .iter()
            .flatten()
            .any(|parameter| match parameter {
                JsAnyParameter::JsAnyFormalParameter(JsAnyFormalParameter::JsFormalParameter(
                    parameter,
                )) => {
                    matches!(
                        parameter.binding(),
                        Ok(JsAnyBindingPattern::JsArrayBindingPattern(_)
                            | JsAnyBindingPattern::JsObjectBindingPattern(_))
                    )
                }
                JsAnyParameter::JsAnyFormalParameter(JsAnyFormalParameter::JsUnknownParameter(
                    _,
                )) => false,
                JsAnyParameter::TsThisParameter(_) => false,
                JsAnyParameter::JsRestParameter(_) => true,
            }),
    };

    Ok(result)
}

#[derive(Clone, Debug)]
enum ArrowFunctionLayout {
    /// Arrow function with a non-arrow function body
    Single(JsArrowFunctionExpression),

    /// A chain of at least two arrow functions.
    ///
    /// An arrow function is part of the chain when it is the body of the parent arrow function.
    ///
    /// The idea of arrow chains is that they break after the `=>` token
    ///
    /// ```javascript
    /// const x =
    ///   (a): string =>
    ///   (b) =>
    ///   (c) =>
    ///   (d) =>
    ///   (e) =>
    ///     f;
    /// ```
    Chain(ArrowChain),
}

#[derive(Clone, Debug)]
struct ArrowChain {
    /// The top most arrow function in the chain
    head: JsArrowFunctionExpression,

    /// The arrow functions in the chain that are neither the first nor the last.
    /// Empty for chains consisting only of two arrow functions.
    middle: Vec<JsArrowFunctionExpression>,

    /// The last arrow function in the chain
    tail: JsArrowFunctionExpression,

    /// The layout of the assignment this arrow function is the right hand side of or `None`
    assignment_layout: Option<AssignmentLikeLayout>,

    /// Whether the group wrapping the signatures should be expanded or not.
    expand_signatures: bool,
}

impl ArrowChain {
    /// Returns an iterator over all arrow functions in this chain
    fn arrows(&self) -> impl Iterator<Item = &JsArrowFunctionExpression> {
        once(&self.head)
            .chain(self.middle.iter())
            .chain(once(&self.tail))
    }
}

impl Format<JsFormatContext> for ArrowChain {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let ArrowChain {
            head,
            tail,
            expand_signatures,
            assignment_layout,
            ..
        } = self;

        let head_parent = head.syntax().parent();
        let tail_body = tail.body()?;

        let is_assignment_rhs = assignment_layout.is_some();

        let is_callee = head_parent
            .as_ref()
            .map_or(false, |parent| is_callee(head.syntax(), parent));

        let body_on_separate_line = !matches!(
            tail_body,
            JsAnyFunctionBody::JsFunctionBody(_)
                | JsAnyFunctionBody::JsAnyExpression(
                    JsAnyExpression::JsObjectExpression(_)
                        | JsAnyExpression::JsSequenceExpression(_)
                )
        );

        let break_before_chain = (is_callee && body_on_separate_line)
            || matches!(
                assignment_layout,
                Some(AssignmentLikeLayout::ChainTailArrowFunction)
            );

        let format_arrow_signatures = format_with(|f| {
            if is_callee || is_assignment_rhs {
                write!(f, [soft_line_break()])?;
            }

            let join_signatures = format_with(|f| {
                for arrow in self.arrows() {
                    write!(f, [format_signature(arrow)])?;

                    // The arrow of the tail is formatted outside of the group to ensure it never
                    // breaks from the body
                    if arrow != tail {
                        write!(
                            f,
                            [
                                space(),
                                arrow.fat_arrow_token().format(),
                                soft_line_break_or_space()
                            ]
                        )?;
                    }
                }

                Ok(())
            });

            write!(
                f,
                [group(&join_signatures).should_expand(*expand_signatures)]
            )
        });

        let format_tail_body_inner = format_with(|f| {
            // Ensure that the parens of sequence expressions end up on their own line if the
            // body breaks
            if matches!(
                tail_body,
                JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsSequenceExpression(_))
            ) {
                write!(
                    f,
                    [group(&format_args![
                        text("("),
                        soft_block_indent(&tail_body.format()),
                        text(")")
                    ])]
                )
            } else {
                write!(f, [tail_body.format()])
            }
        });

        let format_tail_body = format_with(|f| {
            if body_on_separate_line {
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break_or_space(),
                        format_tail_body_inner
                    ])]
                )
            } else {
                write!(f, [space(), format_tail_body_inner])
            }
        });

        let group_id = f.group_id("arrow-chain");

        let format_inner = format_once(|f| {
            write!(
                f,
                [
                    group(&indent(&format_arrow_signatures))
                        .with_group_id(Some(group_id))
                        .should_expand(break_before_chain),
                    space(),
                    tail.fat_arrow_token().format(),
                    indent_if_group_breaks(&format_tail_body, group_id)
                ]
            )?;

            if is_callee {
                write!(
                    f,
                    [if_group_breaks(&soft_line_break()).with_group_id(Some(group_id))]
                )?;
            }

            Ok(())
        });

        write!(f, [group(&format_inner)])
    }
}

impl ArrowFunctionLayout {
    /// Determines the layout for the passed arrow function. See [ArrowFunctionLayout] for a description
    /// of the different layouts.
    fn for_arrow(
        arrow: JsArrowFunctionExpression,
        comments: &JsComments,
        assignment_layout: Option<AssignmentLikeLayout>,
    ) -> SyntaxResult<ArrowFunctionLayout> {
        let mut head = None;
        let mut middle = Vec::new();
        let mut current = arrow;
        let mut should_break = false;

        let result = loop {
            match current.body()? {
                JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrowFunctionExpression(
                    next,
                )) if !comments.is_suppressed(next.syntax()) => {
                    should_break = should_break || should_break_chain(&current)?;

                    if head.is_none() {
                        head = Some(current);
                    } else {
                        middle.push(current);
                    }

                    current = next;
                }
                _ => {
                    break match head {
                        None => ArrowFunctionLayout::Single(current),
                        Some(head) => ArrowFunctionLayout::Chain(ArrowChain {
                            head,
                            middle,
                            tail: current,
                            expand_signatures: should_break,
                            assignment_layout,
                        }),
                    }
                }
            }
        };

        Ok(result)
    }
}

impl NeedsParentheses for JsArrowFunctionExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => true,

            _ => {
                is_conditional_test(self.syntax(), parent)
                    || update_or_lower_expression_needs_parentheses(self.syntax(), parent)
                    || is_binary_like_left_or_right(self.syntax(), parent)
            }
        }
    }
}

/// Returns `true` if the template contains any new lines inside of its text chunks.
fn template_literal_contains_new_line(template: &JsTemplate) -> bool {
    template.elements().iter().any(|element| match element {
        JsAnyTemplateElement::JsTemplateChunkElement(chunk) => chunk
            .template_chunk_token()
            .map_or(false, |chunk| chunk.text().contains('\n')),
        JsAnyTemplateElement::JsTemplateElement(_) => false,
    })
}

/// Returns `true` for a template that starts on the same line as the previous token and contains a line break.
///
///
/// # Examples
//
/// ```javascript
/// "test" + `
///   some content
/// `;
/// ```
///
/// Returns `true` because the template starts on the same line as the `+` token and its text contains a line break.
///
/// ```javascript
/// "test" + `no line break`
/// ```
///
/// Returns `false` because the template text contains no line break.
///
/// ```javascript
/// "test" +
///     `template
///     with line break`;
/// ```
///
/// Returns `false` because the template isn't on the same line as the '+' token.
fn is_multiline_template_starting_on_same_line(template: &JsTemplate) -> bool {
    let contains_new_line = template_literal_contains_new_line(template);

    let starts_on_same_line = template.syntax().first_token().map_or(false, |token| {
        for piece in token.leading_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                if comment.has_newline() {
                    return false;
                }
            } else if piece.is_newline() {
                return false;
            }
        }

        true
    });

    contains_new_line && starts_on_same_line
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{JsArrowFunctionExpression, SourceType};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (a => test)()`", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test)()", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test).member", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => test)[member]", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("object[a => a]", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) as Function", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a)!", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a)`template`", JsArrowFunctionExpression);
        assert_needs_parentheses!("+(a => a)", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) && b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) instanceof b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) in b", JsArrowFunctionExpression);
        assert_needs_parentheses!("(a => a) + b", JsArrowFunctionExpression);
        assert_needs_parentheses!("await (a => a)", JsArrowFunctionExpression);
        assert_needs_parentheses!(
            "<Function>(a => a)",
            JsArrowFunctionExpression,
            SourceType::ts()
        );
        assert_needs_parentheses!("(a => a) ? b : c", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("a ? b => b : c", JsArrowFunctionExpression);
        assert_not_needs_parentheses!("a ? b : c => c", JsArrowFunctionExpression);
        assert_needs_parentheses!("class Test extends (a => a) {}", JsArrowFunctionExpression);
    }
}
