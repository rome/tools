use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_json_syntax::{JsonObjectValue, TextRange};
use rome_rowan::{AstNode, AstSeparatedList};
use std::collections::HashMap;

declare_rule! {
    /// Disallow two keys with the same name inside an object.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "title": "New title",
    ///   "title": "Second title"
    /// }
    /// ```
    pub(crate) NoDuplicateKeys {
        version: "next",
        name: "noDuplicateKeys",
        recommended: true,
    }
}

impl Rule for NoDuplicateKeys {
    type Query = Ast<JsonObjectValue>;
    type State = (String, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let mut names = HashMap::<String, Vec<TextRange>>::new();
        for member in query.json_member_list().iter().flatten() {
            if let Ok(name) = member.name() {
                if let Ok(text) = name.inner_string_text() {
                    if let Some(ranges) = names.get_mut(text.text()) {
                        ranges.push(name.range());
                    } else {
                        names.insert(text.text().to_string(), vec![]);
                    }
                }
            }
        }

        let names: Vec<_> = names
            .into_iter()
            .filter(|(_, ranges)| !ranges.is_empty())
            .flat_map(|(string, ranges)| {
                ranges.into_iter().map(move |range| (string.clone(), range))
            })
            .collect();

        names
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (name, range): &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The key "<Emphasis>{{name}}</Emphasis>" was already declared"
                },
            )
            .detail(
                range,
                markup! {
                    "Remove or rename the key"
                },
            ),
        )
    }
}
