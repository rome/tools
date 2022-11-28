use crate::prelude::*;

use crate::jsx::lists::child_list::{FormatChildrenResult, FormatJsxChildList, JsxChildListLayout};
use crate::utils::jsx::{is_jsx_suppressed, is_meaningful_jsx_text};
use rome_formatter::{format_args, write, CstFormatContext, FormatResult, FormatRuleWithOptions};
use rome_js_syntax::{
    JsAnyExpression, JsxAnyChild, JsxChildList, JsxElement, JsxExpressionChild, JsxFragment,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxElement;

impl FormatNodeRule<JsxElement> for FormatJsxElement {
    fn fmt_fields(&self, node: &JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        JsxAnyTagWithChildren::from(node.clone()).fmt(f)
    }

    fn is_suppressed(&self, node: &JsxElement, f: &JsFormatter) -> bool {
        is_jsx_suppressed(&node.clone().into(), f.comments())
    }

    fn fmt_leading_comments(&self, node: &JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_leading_comments(node.syntax()),
            "JsxElement can not have comments."
        );
        Ok(())
    }

    fn fmt_dangling_comments(&self, node: &JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(node.syntax()),
            "JsxElement can not have comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, node: &JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_trailing_comments(node.syntax()),
            "JsxElement can not have comments."
        );
        Ok(())
    }
}

declare_node_union! {
    pub(super) JsxAnyTagWithChildren = JsxElement | JsxFragment
}

impl Format<JsFormatContext> for JsxAnyTagWithChildren {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let format_opening = format_with(|f| self.fmt_opening(f));
        let format_closing = format_with(|f| self.fmt_closing(f));

        let layout = self.layout(f)?;

        match layout {
            ElementLayout::NoChildren => {
                write!(f, [format_opening, format_closing])
            }

            ElementLayout::Template(expression) => {
                write!(f, [format_opening, expression.format(), format_closing])
            }

            ElementLayout::Default => {
                let mut format_opening = format_opening.memoized();
                let opening_breaks = format_opening.inspect(f)?.will_break();

                let multiple_attributes = match self {
                    JsxAnyTagWithChildren::JsxElement(element) => {
                        element.opening_element()?.attributes().len() > 1
                    }
                    JsxAnyTagWithChildren::JsxFragment(_) => false,
                };

                let list_layout = if multiple_attributes || opening_breaks {
                    JsxChildListLayout::Multiline
                } else {
                    JsxChildListLayout::BestFitting
                };

                let children = self.children();
                let format_children = FormatJsxChildList::default()
                    .with_options(list_layout)
                    .fmt_children(&children, f)?;

                match format_children {
                    FormatChildrenResult::ForceMultiline(multiline) => {
                        write!(f, [format_opening, multiline, format_closing])
                    }
                    FormatChildrenResult::BestFitting {
                        flat_children,
                        expanded_children,
                    } => {
                        let format_closing = format_closing.memoized();
                        write!(
                            f,
                            [best_fitting![
                                format_args![format_opening, flat_children, format_closing],
                                format_args![format_opening, expanded_children, format_closing]
                            ]]
                        )
                    }
                }
            }
        }
    }
}

impl JsxAnyTagWithChildren {
    fn fmt_opening(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsxAnyTagWithChildren::JsxElement(element) => {
                write!(f, [element.opening_element().format()])
            }
            JsxAnyTagWithChildren::JsxFragment(fragment) => {
                write!(f, [fragment.opening_fragment().format()])
            }
        }
    }

    fn fmt_closing(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsxAnyTagWithChildren::JsxElement(element) => {
                write!(f, [element.closing_element().format()])
            }
            JsxAnyTagWithChildren::JsxFragment(fragment) => {
                write!(f, [fragment.closing_fragment().format()])
            }
        }
    }

    fn children(&self) -> JsxChildList {
        match self {
            JsxAnyTagWithChildren::JsxElement(element) => element.children(),
            JsxAnyTagWithChildren::JsxFragment(fragment) => fragment.children(),
        }
    }

    fn layout(&self, f: &mut JsFormatter) -> SyntaxResult<ElementLayout> {
        use JsAnyExpression::*;
        use JsxAnyChild::*;

        let children = self.children();

        let layout = match children.len() {
            0 => ElementLayout::NoChildren,
            1 => {
                // SAFETY: Safe because of length check above
                let child = children.first().unwrap();

                match child {
                    JsxText(text) => {
                        let value_token = text.value_token()?;
                        if !is_meaningful_jsx_text(value_token.text()) {
                            // Text nodes can't have suppressions
                            f.context_mut()
                                .comments()
                                .mark_suppression_checked(text.syntax());
                            // It's safe to ignore the tokens here because JSX text tokens can't have comments (nor whitespace) attached.
                            f.state_mut().track_token(&value_token);

                            ElementLayout::NoChildren
                        } else {
                            ElementLayout::Default
                        }
                    }
                    JsxExpressionChild(expression) => match expression.expression() {
                        Some(JsTemplateExpression(_)) => ElementLayout::Template(expression),
                        _ => ElementLayout::Default,
                    },
                    _ => ElementLayout::Default,
                }
            }
            _ => ElementLayout::Default,
        };

        Ok(layout)
    }
}

#[derive(Debug, Clone)]
enum ElementLayout {
    /// Empty Tag with no children or contains no meaningful text.
    NoChildren,

    /// Prefer breaking the template if it is the only child of the element
    /// ```javascript
    /// <div>{`A Long Tempalte String That uses ${
    ///   5 + 4
    /// } that will eventually break across multiple lines ${(40 / 3) * 45}`}</div>;
    /// ```
    ///
    /// instead of
    ///
    /// ```javascript
    /// <div>
    ///   {`A Long Template String That uses ${
    ///     5 + 4
    ///   } that will eventually break across multiple lines ${(40 / 3) * 45}`}
    /// </div>;
    /// ```
    Template(JsxExpressionChild),

    /// Default layout used for all elements that have children and [ElementLayout::Template] does not apply.
    ///
    /// ```javascript
    ///<Element2>
    ///   Some more content
    ///   <Sub />
    ///   <Sub />
    ///   <Sub />
    /// </Element2>;
    /// ```
    Default,
}
