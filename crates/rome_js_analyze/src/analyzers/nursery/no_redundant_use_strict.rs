use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{AnyJsRoot, JsDirective, JsDirectiveList, JsFunctionBody};

use rome_rowan::{AstNode, BatchMutationExt};

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
    redundant: Vec<JsDirective>,
}

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut directives = vec![(*node).clone()];

        let check_use_strict_directive =
            |node: &JsDirective| node.value_token().map(|v| v.text_trimmed().to_owned());

        // We have to trace the check from great-grandfather's ancestors to skip itself.
        let ancestors = node.syntax().parent()?.parent()?.parent()?.ancestors();
        for possible_directive_list_parent in ancestors {
            if JsFunctionBody::can_cast(possible_directive_list_parent.kind())
                || AnyJsRoot::can_cast(possible_directive_list_parent.kind())
            {
                for n in possible_directive_list_parent.children() {
                    if JsDirectiveList::can_cast(n.kind()) {
                        for n in n.children() {
                            if let Some(js_directive) = JsDirective::cast(n) {
                                if let Ok("\"use strict\"" | "'use strict'") =
                                    check_use_strict_directive(&js_directive).as_deref()
                                {
                                    directives.push(js_directive);
                                }
                            }
                        }
                    }
                }
            }
        }

        if directives.len() > 1 {
            let first = directives.pop()?;
            directives.reverse();
            let redundant = directives;

            return Some(State { redundant, first });
        }

        None
    }
    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            state.first.range(),
            markup! {
                "Cannot have redundant directive of \"use strict\"."
            },
        );

        for js_directive in &state.redundant {
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
        for js_directive in state.redundant.iter() {
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
