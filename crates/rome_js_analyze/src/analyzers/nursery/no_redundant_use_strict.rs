use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{AnyJsRoot, JsDirective, JsFunctionBody, JsLanguage};

use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxNode};

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
 ///
 /// function foo() {
 ///
 /// }
 ///
 /// ### valid
 ///
 ///  function foo() {
 ///     "use strict";
 /// }
 /// function bar() {
 ///     "use strict";
 /// }
 ///
 ///

 pub(crate) NoRedundantUseStrict {
     version: "11.0.0",
     name: "NoRedundantUseStrict",
     recommended: false,
    }
}

pub struct State {
    first: JsDirective,
    redundant: JsDirective,
}
declare_node_union! { AnyNodeWithDirectives = JsFunctionBody | AnyJsRoot }

impl AnyNodeWithDirectives {
    fn directives(&self) -> Option<Vec<JsDirective>> {
        match self {
            AnyNodeWithDirectives::JsFunctionBody(node) => {
                get_directives_from_directive_list(node.syntax())
            }
            AnyNodeWithDirectives::AnyJsRoot(node) => {
                get_directives_from_directive_list(node.syntax())
            }
        }
    }
}

fn get_directives_from_directive_list(node: &SyntaxNode<JsLanguage>) -> Option<Vec<JsDirective>> {
    let mut directives = vec![];
    for n in node.first_child()?.children() {
        if let Some(js_directive) = JsDirective::cast(n) {
            directives.push(js_directive)
        }
    }
    Some(directives)
}

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = State;
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
            for directive in parent.directives()? {
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
            return Some(State {
                first: outer_most,
                redundant: (*node).clone(),
            });
        }
        None
    }
    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.first.range(),
            markup! {
                "Cannot have redundant directive of \"use strict\"."
            },
        )
        .detail(
            state.redundant.range(),
            markup! {"This is where the redundant \"use strict\" is declared."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let mut batch = root.begin();

        batch.remove_node(state.redundant.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove redundant use strict" }.to_owned(),
            mutation: batch,
        })
    }
}
