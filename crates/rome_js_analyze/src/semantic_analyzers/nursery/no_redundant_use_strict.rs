use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsDirective, JsModule};
use rome_rowan::{AstNode, BatchMutationExt};

use crate::{semantic_services::Semantic, JsRuleAction};

declare_rule! {
 /// Prevents from having redundant \"use strict\"
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// "use strict";
 /// function foo() {
 ///  	"use strict";
 /// }
 ///
 /// ### valid
 /// "use strict";
 /// function foo() {
 ///
 /// }
 ///

 pub(crate) NoRedundantUseStrict {
     version: "11.0.0",
     name: "NoRedundantUseStrict",
     recommended: false,
    }
}

impl Rule for NoRedundantUseStrict {
    type Query = Semantic<JsModule>;
    type State = Vec<JsDirective>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let check_use_strict_directive =
            |node: &JsDirective| node.value_token().map(|v| v.text_trimmed().to_owned());

        let mut use_strict_vec = vec![];
        for n in node.syntax().descendants() {
            if let Some(js_directive) = JsDirective::cast(n) {
                if let Ok("\"use strict\"" | "'use strict'") =
                    check_use_strict_directive(&js_directive).as_deref()
                {
                    use_strict_vec.push(js_directive);
                }
            }
        }

        if use_strict_vec.len() > 1 {
            return Some(use_strict_vec);
        }

        None
    }
    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            state[0].range(),
            markup! {
                "Cannot have redundant directive of \"use strict\"."
            },
        );

        for js_directive in state.iter().skip(1) {
            diag = diag.detail(
                js_directive.range(),
                markup! {"This is where the redundant \"use strict\" is declared."},
            );
        }
        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let mut batch = root.begin();
        for js_directive in state.iter().skip(1) {
            if let Ok(token) = js_directive.value_token() {
                batch.remove_token(token);
            }
            if let Some(token) = js_directive.semicolon_token() {
                batch.remove_token(token);
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove redundant use strict" }.to_owned(),
            mutation: batch,
        })
    }
}
