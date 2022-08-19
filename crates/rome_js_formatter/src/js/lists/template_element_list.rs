use crate::js::expressions::template_chunk_element::AnyTemplateChunkElement;
use crate::js::expressions::template_element::{AnyTemplateElement, TemplateElementOptions};

use crate::context::TabWidth;
use crate::prelude::*;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyTemplateElement, JsLanguage,
    JsTemplateElementList, TsAnyTemplateElement, TsTemplateElementList,
};
use rome_rowan::{declare_node_union, AstNodeListIterator, SyntaxResult};
use std::iter::FusedIterator;

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplateElementList;

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        AnyTemplateElementList::JsTemplateElementList(node.clone()).fmt(f)
    }
}

pub(crate) enum AnyTemplateElementList {
    JsTemplateElementList(JsTemplateElementList),
    TsTemplateElementList(TsTemplateElementList),
}

impl Format<JsFormatContext> for AnyTemplateElementList {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let layout = if self.is_simple() {
            TemplateElementLayout::SingleLine
        } else {
            TemplateElementLayout::Fit
        };

        let mut indention = TemplateElementIndention::default();
        let mut after_new_line = false;

        for element in self.elements() {
            match element {
                AnyTemplateElementOrChunk::AnyTemplateElement(element) => {
                    let options = TemplateElementOptions {
                        after_new_line,
                        indention,
                        layout,
                    };

                    match &element {
                        AnyTemplateElement::JsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                        AnyTemplateElement::TsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                    }
                }
                AnyTemplateElementOrChunk::AnyTemplateChunkElement(chunk) => {
                    match &chunk {
                        AnyTemplateChunkElement::JsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                        AnyTemplateChunkElement::TsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                    }

                    let chunk_token = chunk.template_chunk_token()?;
                    let chunk_text = chunk_token.text();

                    let tab_width = f.context().tab_width();

                    indention =
                        TemplateElementIndention::after_last_new_line(chunk_text, tab_width);
                    after_new_line = chunk_text.ends_with('\n');
                }
            }
        }

        Ok(())
    }
}

impl AnyTemplateElementList {
    /// Returns `true` for `JsTemplate` if all elements are simple expressions that should be printed on a single line.
    ///
    /// Simple expressions are:
    /// * Identifiers: `this`, `a`
    /// * Members: `a.b`, `a[b]`, `a.b[c].d`, `a.b[5]`, `a.b["test"]`
    fn is_simple(&self) -> bool {
        match self {
            AnyTemplateElementList::JsTemplateElementList(list) => {
                if list.is_empty() {
                    return false;
                }

                let mut expression_elements = list.iter().filter_map(|element| match element {
                    JsAnyTemplateElement::JsTemplateElement(element) => Some(element),
                    _ => None,
                });

                expression_elements.all(|expression_element| {
                    match expression_element.expression() {
                        Ok(expression) => is_simple_member_expression(expression).unwrap_or(false),
                        Err(_) => false,
                    }
                })
            }
            AnyTemplateElementList::TsTemplateElementList(_) => false,
        }
    }

    fn elements(&self) -> TemplateElementIterator {
        match self {
            AnyTemplateElementList::JsTemplateElementList(list) => {
                TemplateElementIterator::JsTemplateElementList(list.iter())
            }
            AnyTemplateElementList::TsTemplateElementList(list) => {
                TemplateElementIterator::TsTemplateElementList(list.iter())
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TemplateElementLayout {
    /// Applied when all expressions are identifiers, `this`, static member expressions, or computed member expressions with number or string literals.
    /// Formats the expressions on a single line, even if their width otherwise would exceed the print width.
    SingleLine,

    /// Tries to format the expression on a single line but may break the expression if the line otherwise exceeds the print width.
    Fit,
}

impl Default for TemplateElementLayout {
    fn default() -> Self {
        TemplateElementLayout::Fit
    }
}

declare_node_union! {
    AnyTemplateElementOrChunk = AnyTemplateElement | AnyTemplateChunkElement
}

fn is_simple_member_expression(expression: JsAnyExpression) -> SyntaxResult<bool> {
    let mut current = expression;

    loop {
        if current.syntax().has_comments_direct() {
            return Ok(false);
        }

        current = match current {
            JsAnyExpression::JsStaticMemberExpression(expression) => expression.object()?,
            JsAnyExpression::JsComputedMemberExpression(expression) => {
                if matches!(
                    expression.member()?,
                    JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsStringLiteralExpression(_)
                            | JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                    ) | JsAnyExpression::JsIdentifierExpression(_)
                ) {
                    expression.object()?
                } else {
                    break;
                }
            }
            JsAnyExpression::JsIdentifierExpression(_) | JsAnyExpression::JsThisExpression(_) => {
                return Ok(true);
            }
            _ => {
                break;
            }
        }
    }

    Ok(false)
}

enum TemplateElementIterator {
    JsTemplateElementList(AstNodeListIterator<JsLanguage, JsAnyTemplateElement>),
    TsTemplateElementList(AstNodeListIterator<JsLanguage, TsAnyTemplateElement>),
}

impl Iterator for TemplateElementIterator {
    type Item = AnyTemplateElementOrChunk;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    JsAnyTemplateElement::JsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    JsAnyTemplateElement::JsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };
                Some(result)
            }
            TemplateElementIterator::TsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    TsAnyTemplateElement::TsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    TsAnyTemplateElement::TsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };

                Some(result)
            }
        }
    }
}

impl ExactSizeIterator for TemplateElementIterator {
    fn len(&self) -> usize {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => inner.len(),
            TemplateElementIterator::TsTemplateElementList(inner) => inner.len(),
        }
    }
}

impl FusedIterator for TemplateElementIterator {}

/// The indention derived from a position in the source document. Consists of indention level and spaces
#[derive(Debug, Copy, Clone, Default)]
pub struct TemplateElementIndention(u32);

impl TemplateElementIndention {
    /// Returns the indention level
    pub(crate) fn level(&self, tab_width: TabWidth) -> u32 {
        self.0 / (u8::from(tab_width) as u32)
    }

    /// Returns the number of space indents on top of the indent level
    pub(crate) fn align(&self, tab_width: TabWidth) -> u8 {
        (self.0 % u8::from(tab_width) as u32) as u8
    }

    /// Computes the indention after the last new line character.
    fn after_last_new_line(text: &str, tab_width: TabWidth) -> Self {
        let by_new_line = text.rsplit_once('\n');

        let size = match by_new_line {
            None => 0,
            Some((_, after_new_line)) => {
                let tab_width: u32 = u8::from(tab_width).into();
                let mut size: u32 = 0;

                for c in after_new_line.chars() {
                    match c {
                        '\t' => {
                            // Tabs behave in a way that they are aligned to the nearest
                            // multiple of tab_width:
                            // number of spaces -> added size
                            // 0 -> 4, 1 -> 4, 2 -> 4, 3 -> 4
                            // 4 -> 8, 5 -> 8, 6 -> 8, 7 -> 8 ..
                            // Or in other words, it clips the size to the next multiple of tab width.
                            size = size + tab_width - (size % tab_width);
                        }
                        ' ' => {
                            size += 1;
                        }
                        _ => break,
                    };
                }

                size
            }
        };

        TemplateElementIndention(size)
    }
}
