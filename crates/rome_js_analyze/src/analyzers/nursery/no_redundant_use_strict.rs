use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsDirective, JsDirectiveList, JsFunctionBody, JsModule, JsScript};

use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
 /// Prevents from having redundant `"use strict"`.
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// ```js,expect_diagnostic
 /// "use strict";
 /// function foo() {
 ///  	"use strict";
 /// }
 /// ```
 /// ```js,expect_diagnostic
 /// "use strict";
 /// "use strict";
 ///
 /// function foo() {
 ///
 /// }
 /// ```
 /// ```js,expect_diagnostic
 /// function foo() {
 /// "use strict";
 /// "use strict";
 /// }
 /// ```
 /// ### valid
 /// ```js
 /// function foo() {
 ///
 /// }
 ///```
 /// ```js
 ///  function foo() {
 ///     "use strict";
 /// }
 /// function bar() {
 ///     "use strict";
 /// }
 ///```
 ///

 pub(crate) NoRedundantUseStrict {
     version: "11.0.0",
     name: "noRedundantUseStrict",
     recommended: false,
    }
}

declare_node_union! { AnyNodeWithDirectives = JsFunctionBody | JsModule | JsScript }
impl AnyNodeWithDirectives {
    fn directives(&self) -> JsDirectiveList {
        match self {
            AnyNodeWithDirectives::JsFunctionBody(node) => node.directives(),
            AnyNodeWithDirectives::JsScript(script) => script.directives(),
            AnyNodeWithDirectives::JsModule(module) => module.directives(),
        }
    }
}

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = JsDirective;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut outer_most: Option<JsDirective> = None;
        for parent in node
            .syntax()
            .ancestors()
            .filter_map(AnyNodeWithDirectives::cast)
        {
            for directive in parent.directives() {
                if directive.value_token().map_or(false, |t| {
                    matches!(t.text_trimmed(), "'use strict'" | "\"use strict\"")
                }) {
                    outer_most = Some(directive);
                    break; // continue with next parent
                }
            }
        }

        if let Some(outer_most) = outer_most {
            // skip itself
            if &outer_most == node {
                return None;
            }
            return Some(outer_most);
        }

        None
    }
    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Redundant "<Emphasis>{"use strict"}</Emphasis>" directive."
            },
        )
        .detail(
            state.range(),
            markup! {"This outer "<Emphasis>{"use strict"}</Emphasis>" directive already enables strict mode."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let mut batch = root.begin();

        batch.remove_node(ctx.query().clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove the redundant \"use strict\" directive" }.to_owned(),
            mutation: batch,
        })
    }
}
