use crate::prelude::*;
use rome_formatter::printer::{PrintWidth, Printer};
use rome_formatter::{
    format_args, write, CstFormatContext, FormatOptions, FormatRuleWithOptions, VecBuffer,
};

use crate::context::TabWidth;
use crate::js::lists::template_element_list::{TemplateElementIndention, TemplateElementLayout};
use rome_js_syntax::{
    JsAnyExpression, JsSyntaxNode, JsSyntaxToken, JsTemplateElement, TsTemplateElement,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplateElement {
    options: TemplateElementOptions,
}

impl FormatRuleWithOptions<JsTemplateElement> for FormatJsTemplateElement {
    type Options = TemplateElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<JsTemplateElement> for FormatJsTemplateElement {
    fn fmt_fields(
        &self,
        node: &JsTemplateElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let element = AnyTemplateElement::from(node.clone());

        FormatTemplateElement::new(element, self.options).fmt(formatter)
    }
}

declare_node_union! {
    pub(crate) AnyTemplateElement = JsTemplateElement | TsTemplateElement
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TemplateElementOptions {
    pub(crate) layout: TemplateElementLayout,

    /// The indention to use for this element
    pub(crate) indention: TemplateElementIndention,

    /// Does the last template chunk (text element) end with a new line?
    pub(crate) after_new_line: bool,
}

pub(crate) struct FormatTemplateElement {
    element: AnyTemplateElement,
    options: TemplateElementOptions,
}

impl FormatTemplateElement {
    pub(crate) fn new(element: AnyTemplateElement, options: TemplateElementOptions) -> Self {
        Self { element, options }
    }
}

impl Format<JsFormatContext> for FormatTemplateElement {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let format_expression = format_with(|f| match &self.element {
            AnyTemplateElement::JsTemplateElement(template) => {
                write!(f, [template.expression().format()])
            }
            AnyTemplateElement::TsTemplateElement(template) => {
                write!(f, [template.ty().format()])
            }
        });

        let format_inner = format_with(|f: &mut JsFormatter| match self.options.layout {
            TemplateElementLayout::SingleLine => {
                // The goal is to print the expression on a single line, even if it exceeds the configured print width.
                //
                // Ideally, it would be possible to use a custom buffer that drops all soft line breaks
                // (or converts them to spaces). However, this isn't straightforward with our
                // nested IR (but would be with a flat ir).
                //
                // That's why we write the expression into a temporary buffer and print it
                // with a printer that uses a print width so large, that the expression never exceeds
                // the print width.
                let mut buffer = VecBuffer::new(f.state_mut());
                write!(buffer, [format_expression])?;
                let root = buffer.into_element();

                let print_options = f
                    .options()
                    .as_print_options()
                    .with_print_width(PrintWidth::infinite());
                let printed = Printer::new(print_options).print(&root);

                write!(
                    f,
                    [dynamic_text(
                        printed.as_code(),
                        self.element.inner_syntax()?.text_trimmed_range().start()
                    )]
                )
            }
            TemplateElementLayout::Fit => {
                use JsAnyExpression::*;

                let expression = self.element.expression();

                // It's preferred to break after/before `${` and `}` rather than breaking in the
                // middle of some expressions.
                let indent = f
                    .context()
                    .comments()
                    .has_comments(&self.element.inner_syntax()?)
                    || matches!(
                        expression,
                        Some(
                            JsStaticMemberExpression(_)
                                | JsComputedMemberExpression(_)
                                | JsConditionalExpression(_)
                                | JsSequenceExpression(_)
                                | TsAsExpression(_)
                                | JsBinaryExpression(_)
                                | JsLogicalExpression(_)
                                | JsInstanceofExpression(_)
                                | JsInExpression(_)
                        )
                    );

                if indent {
                    write!(f, [soft_block_indent(&format_expression)])
                } else {
                    write!(f, [format_expression])
                }
            }
        });

        let format_indented = format_with(|f: &mut JsFormatter| {
            if self.options.after_new_line {
                write!(f, [dedent_to_root(&format_inner)])
            } else {
                write_with_indention(
                    &format_inner,
                    self.options.indention,
                    f.options().tab_width(),
                    f,
                )
            }
        });

        write!(
            f,
            [group(&format_args![
                self.element.dollar_curly_token().format(),
                format_indented,
                line_suffix_boundary(),
                self.element.r_curly_token().format()
            ])]
        )
    }
}

impl AnyTemplateElement {
    fn dollar_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.dollar_curly_token(),
            AnyTemplateElement::TsTemplateElement(template) => template.dollar_curly_token(),
        }
    }

    fn inner_syntax(&self) -> SyntaxResult<JsSyntaxNode> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => {
                template.expression().map(AstNode::into_syntax)
            }
            AnyTemplateElement::TsTemplateElement(template) => {
                template.ty().map(AstNode::into_syntax)
            }
        }
    }

    fn expression(&self) -> Option<JsAnyExpression> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.expression().ok(),
            AnyTemplateElement::TsTemplateElement(_) => None,
        }
    }

    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.r_curly_token(),
            AnyTemplateElement::TsTemplateElement(template) => template.r_curly_token(),
        }
    }
}

/// Writes `content` with the specified `indention`.
fn write_with_indention<Content>(
    content: &Content,
    indention: TemplateElementIndention,
    tab_width: TabWidth,
    f: &mut JsFormatter,
) -> FormatResult<()>
where
    Content: Format<JsFormatContext>,
{
    let level = indention.level(tab_width);
    let spaces = indention.align(tab_width);

    if level == 0 && spaces == 0 {
        return write!(f, [content]);
    }

    // Adds as many nested `indent` elements until it reaches the desired indention level.
    let format_indented = format_with(|f| {
        if level == 0 {
            write!(f, [content])
        } else {
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(buffer, [content])?;

            let mut indented = buffer.into_element();

            for _ in 0..level {
                indented = FormatElement::Indent(vec![indented].into_boxed_slice());
            }

            f.write_element(indented)
        }
    });

    // Adds any necessary `align` for spaces not covered by indent level.
    let format_aligned = format_with(|f| {
        if spaces == 0 {
            write!(f, [format_indented])
        } else {
            write!(f, [align(spaces, &format_indented)])
        }
    });

    write!(f, [dedent_to_root(&format_aligned)])
}
