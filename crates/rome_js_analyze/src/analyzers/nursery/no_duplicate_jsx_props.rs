use std::collections::HashMap;

use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::diagnostic;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, JsxAttribute, JsxAttributeList, JsxOpeningElement,
    JsxSelfClosingElement,
};
use rome_rowan::{AstNode, AstNodeList, SyntaxTokenText};

declare_rule! {
 /// Promotes the use of awesome tricks
 ///
 /// ## Examples
 ///
 /// ### Invalid
 ///
 pub(crate) NoDuplicateJsxProps {
     version: "13.0.0",
     name: "noDuplicateJsxProps",
     recommended: true,
    }
}

fn get_name(attr: &JsxAttribute) -> Option<SyntaxTokenText> {
    match attr.name().ok()? {
        AnyJsxAttributeName::JsxName(name) => Some(name.value_token().ok()?.token_text_trimmed()),
        AnyJsxAttributeName::JsxNamespaceName(_) => None,
    }
}

impl Rule for NoDuplicateJsxProps {
    type Query = Ast<AnyJsxElement>;
    type State = (String, Vec<JsxAttribute>);
    type Signals = HashMap<String, Vec<JsxAttribute>>;
    type Options = ();

    fn run(ctx: &rome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_properties: HashMap<String, Vec<JsxAttribute>> = HashMap::new();
        for attribute in node.attributes() {
            match attribute {
                AnyJsxAttribute::JsxAttribute(attribute) => {
                    if let Some(name) = get_name(&attribute) {
                        defined_properties
                            .entry(name.to_lowercase())
                            .or_default()
                            .push(attribute);
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_) => continue,
            }
        }

        defined_properties.retain(|_, val| val.len() > 1);

        defined_properties
    }

    fn diagnostic(
        _ctx: &rome_analyze::context::RuleContext<Self>,
        _state: &Self::State,
    ) -> Option<rome_analyze::RuleDiagnostic> {
        let mut diagnostic: Option<RuleDiagnostic> = None;

        for attr in _state.1.iter() {
            match diagnostic {
                Some(diag) => {
                    diagnostic = Some(diag.detail(
                        attr.syntax().text_trimmed_range(),
                        "attribute is duplicated!",
                    ));
                }
                None => {
                    diagnostic = Some(RuleDiagnostic::new(
                        rule_category!(),
                        attr.syntax().text_trimmed_range(),
                        markup!("Elements can not have attributes with the same name."),
                    ));
                }
            }
        }

        // for (_, attributes) in _state {
        //     for attr in attributes {
        //         match diagnostic {
        //             Some(diag) => {
        //                 diag.detail(
        //                     attr.syntax().text_trimmed_range(),
        //                     "attribute is duplicated!",
        //                 );
        //             }
        //             None => {
        //                 diagnostic = Some(RuleDiagnostic::new(
        //                     rule_category!(),
        //                     attr.syntax().text_trimmed_range(),
        //                     markup!("Elements can not have attributes with the same name."),
        //                 ));
        //             }
        //         }
        //     }
        // }
        diagnostic
    }
}
