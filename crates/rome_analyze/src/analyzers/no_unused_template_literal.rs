use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::JsSyntaxKind::*;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsAnyTemplateElement, JsTemplate,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, SyntaxToken};

use crate::registry::{Rule, RuleAction, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};

pub(crate) enum NoUnusedTemplateLiteral {}

impl Rule for NoUnusedTemplateLiteral {
    const NAME: &'static str = "noUnusedTemplateLiteral";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsTemplate;
    type State = ();

    fn run(node: &Self::Query) -> Option<Self::State> {
        if node.tag().is_none() && can_convert_to_string_lit(node) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(node: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "Do not use template literals if interpolation and special-character handling are not needed."
            }
            .to_owned(),
            range: node.range(),
        })
    }

    fn action(root: JsAnyRoot, node: &Self::Query, _: &Self::State) -> Option<RuleAction> {
        // join all template content
        let inner_content = node
            .elements()
            .iter()
            .fold(String::from("\""), |mut acc, cur| {
                match cur {
                    JsAnyTemplateElement::JsTemplateChunkElement(ele) => {
                        // Safety: if `ele.template_chunk_token()` is `Err` variant, [can_convert_to_string_lit] should return false,
                        // thus `run` will return None
                        acc += ele.template_chunk_token().unwrap().text();
                        acc
                    }
                    JsAnyTemplateElement::JsTemplateElement(_) => {
                        // Because we know if TemplateLit has any `JsTemplateElement` will return `None` in `run` function
                        unreachable!()
                    }
                }
            });
        let root = root.replace_node(
            JsAnyExpression::JsTemplate(node.clone()),
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(SyntaxToken::new_detached(
                        JS_STRING_LITERAL,
                        &(inner_content + "\""),
                        [],
                        [],
                    )),
                ),
            ),
        )?;
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with string literal" }.to_owned(),
            root,
        })
    }
}

fn can_convert_to_string_lit(node: &JsTemplate) -> bool {
    !node.elements().iter().any(|ele| {
        // We want to test if any templateElement has violated rule that can convert to string literal
        match ele {
            JsAnyTemplateElement::JsTemplateElement(_) => true,
            JsAnyTemplateElement::JsTemplateChunkElement(chunk) => {
                match chunk.template_chunk_token() {
                    Ok(token) => {
                        // if token text has any SpecialCharacters
                        token
                            .text()
                            .chars()
                            .any(|ch| matches!(ch, '\n' | '\'' | '"'))
                    }
                    Err(_) => {
                        // if we found a error token, then just return true, which means this template literal can't convert to
                        // string literal
                        true
                    }
                }
            }
        }
    })
}
