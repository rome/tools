use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{
    jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_ident, jsx_name,
    jsx_string, token,
};
use rome_js_syntax::{
    JsxAnyAttribute, JsxAnyAttributeName, JsxAnyAttributeValue, JsxAttribute, JsxAttributeList,
    JsxElement, JsxSelfClosingElement, T,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};

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
    /// ```jsx,expect_diagnostic
    /// <a href='http://external.link' target='_blank' rel="noopener">child</a>
    /// ```
    /// ```jsx,expect_diagnostic
    /// // case-insensitive
    /// <a href='http://external.link' target='_BlaNk'>child</a>
    /// ```
    /// ### Valid
    ///
    /// ```jsx
    /// <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    /// ```
    pub(crate) UseBlankTarget {
        version: "10.0.0",
        name: "useBlankTarget",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) UseBlankTargetQuery = JsxElement | JsxSelfClosingElement
}

impl Rule for UseBlankTarget {
    type Query = Ast<UseBlankTargetQuery>;
    /// Two attributes:
    /// 1. The attribute `target=`
    /// 2. The attribute `rel=`, if present
    type State = (JsxAttribute, Option<JsxAttribute>);
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (target_attribute, rel_attribute) = match node {
            UseBlankTargetQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                if opening_element.name().ok()?.text() != "a"
                    || opening_element
                        .find_attribute_by_name("href")
                        .ok()
                        .is_none()
                {
                    return None;
                }

                (
                    opening_element.find_attribute_by_name("target").ok()?,
                    opening_element.find_attribute_by_name("rel").ok()?,
                )
            }
            UseBlankTargetQuery::JsxSelfClosingElement(element) => {
                if element.name().ok()?.text() != "a"
                    || element.find_attribute_by_name("href").ok().is_none()
                {
                    return None;
                }
                (
                    element.find_attribute_by_name("target").ok()?,
                    element.find_attribute_by_name("rel").ok()?,
                )
            }
        };

        let target_attribute = target_attribute?;
        let text = target_attribute
            .initializer()?
            .value()
            .ok()?
            .as_jsx_string()?
            .inner_string_text()
            .ok()?;

        if text.to_lowercase() == "_blank" {
            match rel_attribute {
                None => {
                    return Some((target_attribute, None));
                }
                Some(rel_attribute) => {
                    let rel_text = rel_attribute
                        .initializer()?
                        .value()
                        .ok()?
                        .as_jsx_string()?
                        .inner_string_text()
                        .ok()?;
                    if !rel_text.text().contains("noreferrer") {
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
            let old_jsx_string = rel_attribute.initializer()?.value().ok()?;
            let old_jsx_string = old_jsx_string.as_jsx_string()?;
            let rel_text = old_jsx_string.inner_string_text().ok()?;
            let new_text = format!("\"noreferrer {rel_text}\"");
            let new_jsx_string = jsx_string(jsx_ident(&new_text));
            mutation.replace_node(old_jsx_string.clone(), new_jsx_string);

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
            let new_attribute = jsx_attribute(JsxAnyAttributeName::JsxName(jsx_name(
                jsx_ident("rel").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )))
            .with_initializer(jsx_attribute_initializer_clause(
                token(T![=]),
                JsxAnyAttributeValue::JsxString(jsx_string(jsx_ident("\"noreferrer\""))),
            ))
            .build();

            new_attribute_list.push(JsxAnyAttribute::JsxAttribute(new_attribute));

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
        ).footer_note(
            markup!{
                "Opening external links in new tabs without rel=\"noreferrer\" is a security risk. See \
                "<Hyperlink href="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener">"the explanation"</Hyperlink>" for more details."
            }
        ))
    }
}
