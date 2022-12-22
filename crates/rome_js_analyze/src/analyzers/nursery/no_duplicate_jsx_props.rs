use std::collections::HashMap;

use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{AnyJsxAttribute, JsxAttribute, SyntaxNodeText};
use rome_rowan::AstNode;

declare_rule! {
    /// Prevents duplicate properties in JSX elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Hello name="John" name="John" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <label xml:lang="en-US" xml:lang="en-US"></label>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Hello firstname="John" lastname="Doe" />
    /// ```
    ///
    /// ```js
    /// <label xml:lang="en-US" lang="en-US"></label>
    /// ```
 pub(crate) NoDuplicateJsxProps {
     version: "13.0.0",
     name: "noDuplicateJsxProps",
     recommended: true,
    }
}

fn get_name_text(attr: &JsxAttribute) -> Option<SyntaxNodeText> {
    Some(attr.name().ok()?.syntax().text_trimmed())
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
                    if let Some(name) = get_name_text(&attribute) {
                        defined_properties
                            .entry(name.to_string().to_lowercase())
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
        _: &rome_analyze::context::RuleContext<Self>,
        state: &Self::State,
    ) -> Option<rome_analyze::RuleDiagnostic> {
        let mut attributes = state.1.iter();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attributes.next()?.syntax().text_trimmed_range(),
            markup!("Elements can not have attributes with the same name."),
        );

        for attr in attributes {
            diagnostic = diagnostic.detail(
                attr.syntax().text_trimmed_range(),
                "attribute is duplicated!",
            )
        }

        Some(diagnostic)
    }
}
