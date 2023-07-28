use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{
    jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_ident, jsx_name,
    jsx_string, token,
};
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, JsxAttribute, JsxAttributeList, T,
};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};

declare_rule! {
    /// Disallow `target="_blank"` attribute without `rel="noreferrer"`
    ///
    /// When creating anchor `a` element, there are times when its link has to be opened in a new browser tab
    /// via `target="_blank"` attribute. This attribute has to paired with `rel="noreferrer"` or you're incur
    /// in a security issue.
    ///
    /// Refer to [the noreferrer documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noreferrer)
    /// and the [the noopener documentation](https://html.spec.whatwg.org/multipage/links.html#link-type-noopener)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a href='http://external.link' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href='http://external.link' target='_blank' rel="noopener">child</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a {...props} href='http://external.link' target='_blank' rel="noopener">child</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    /// ```
    ///
    /// ```jsx
    /// <a href='http://external.link' target='_blank' rel="noopener" {...props}>child</a>
    /// ```
    pub(crate) NoBlankTarget {
        version: "10.0.0",
        name: "noBlankTarget",
        recommended: true,
    }
}

impl Rule for NoBlankTarget {
    type Query = Ast<AnyJsxElement>;
    /// Two attributes:
    /// 1. The attribute `target=`
    /// 2. The attribute `rel=`, if present
    type State = (JsxAttribute, Option<JsxAttribute>);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.name_value_token()?.text_trimmed() != "a"
            || node.find_attribute_by_name("href").is_none()
        {
            return None;
        }

        let target_attribute = node.find_attribute_by_name("target")?;
        let rel_attribute = node.find_attribute_by_name("rel");

        if target_attribute.as_static_value()?.text() == "_blank" {
            match rel_attribute {
                None => {
                    if !node.has_trailing_spread_prop(target_attribute.clone()) {
                        return Some((target_attribute, None));
                    }
                }
                Some(rel_attribute) => {
                    if rel_attribute.initializer().is_none()
                        || (!rel_attribute
                            .as_static_value()?
                            .text()
                            .split_ascii_whitespace()
                            .any(|f| f == "noreferrer")
                            && !node.has_trailing_spread_prop(target_attribute.clone())
                            && !node.has_trailing_spread_prop(rel_attribute.clone()))
                    {
                        return Some((target_attribute, Some(rel_attribute)));
                    }
                }
            }
        }

        None
    }

    fn action(
        ctx: &RuleContext<Self>,
        (target_attribute, rel_attribute): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = if let Some(rel_attribute) = rel_attribute {
            let prev_jsx_attribute = rel_attribute.initializer()?.value().ok()?;
            let prev_jsx_string = prev_jsx_attribute.as_jsx_string()?;
            let new_text = format!(
                "\"noreferrer {}\"",
                prev_jsx_string.inner_string_text().ok()?.text()
            );
            mutation.replace_node(prev_jsx_string.clone(), jsx_string(jsx_ident(&new_text)));

            (markup! {
                "Add the "<Emphasis>"\"noreferrer\""</Emphasis>" to the existing attribute."
            })
            .to_owned()
        } else {
            let old_attribute_list = target_attribute
                .syntax()
                .ancestors()
                .find_map(JsxAttributeList::cast)?;
            let mut new_attribute_list: Vec<_> = old_attribute_list.iter().collect();
            let new_attribute = jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(
                jsx_ident("rel").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )))
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                AnyJsxAttributeValue::JsxString(jsx_string(jsx_ident("\"noreferrer\""))),
            ))
            .build();

            new_attribute_list.push(AnyJsxAttribute::JsxAttribute(new_attribute));

            mutation.replace_node(old_attribute_list, jsx_attribute_list(new_attribute_list));

            (markup! {
                "Add the "<Emphasis>"rel=\"noreferrer\""</Emphasis>" attribute."
            })
            .to_owned()
        };

        Some(JsRuleAction {
            mutation,
            message,
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (target_attribute, _): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            target_attribute.syntax().text_trimmed_range(),
            markup! {
                "Avoid using "<Emphasis>"target=\"_blank\""</Emphasis>" without "<Emphasis>"rel=\"noreferrer\""</Emphasis>"."
            },
        ).note(
            markup!{
                "Opening external links in new tabs without rel=\"noreferrer\" is a security risk. See \
                "<Hyperlink href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">"the explanation"</Hyperlink>" for more details."
            }
        ))
    }
}
