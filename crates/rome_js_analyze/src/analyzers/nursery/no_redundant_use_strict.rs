use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsClassDeclaration, JsClassExpression, JsDirective, JsDirectiveList, JsFunctionBody,
    JsLanguage, JsModule, JsScript,
};

use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxNode};

declare_rule! {
 /// Prevents from having redundant `"use strict"`.
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// ```cjs,expect_diagnostic
 /// "use strict";
 /// function foo() {
 ///  	"use strict";
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// "use strict";
 /// "use strict";
 ///
 /// function foo() {
 ///
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// function foo() {
 /// "use strict";
 /// "use strict";
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// class C1 {
 /// 	test() {
 /// 		"use strict";
 /// 	}
 /// }
 /// ```
 /// ```cjs,expect_diagnostic
 /// const C2 = class {
 /// 	test() {
 /// 		"use strict";
 /// 	}
 /// };
 ///
 /// ```
 /// ### Valid
 /// ```cjs
 /// function foo() {
 ///
 /// }
 ///```
 /// ```cjs
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

declare_node_union! { AnyNodeWithDirectives = JsFunctionBody | JsScript }
impl AnyNodeWithDirectives {
    fn directives(&self) -> JsDirectiveList {
        match self {
            AnyNodeWithDirectives::JsFunctionBody(node) => node.directives(),
            AnyNodeWithDirectives::JsScript(script) => script.directives(),
        }
    }
}
declare_node_union! { JsModuleOrClass = JsClassDeclaration | JsClassExpression| JsModule  }

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = SyntaxNode<JsLanguage>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut outer_most: Option<SyntaxNode<JsLanguage>> = None;
        for n in node.syntax().ancestors() {
            if let Some(parent) = AnyNodeWithDirectives::cast_ref(&n) {
                for directive in parent.directives() {
                    if directive.value_token().map_or(false, |t| {
                        matches!(t.text_trimmed(), "'use strict'" | "\"use strict\"")
                    }) {
                        outer_most = Some(directive.into());
                        break; // continue with next parent
                    }
                }
            }
            if let Some(module_or_class) = JsModuleOrClass::cast_ref(&n) {
                outer_most = Some(module_or_class.into());
            }
        }

        if let Some(outer_most) = outer_most {
            // skip itself
            if &outer_most == node.syntax() {
                return None;
            }
            return Some(outer_most);
        }

        None
    }
    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Redundant "<Emphasis>{"use strict"}</Emphasis>" directive."
            },
        );

        if JsModule::can_cast(state.kind()) {
            diag= diag.note(
                markup! {"The entire contents of "<Emphasis>{"JavaScript modules are automatically"}</Emphasis>" in strict mode, with no statement needed to initiate it."},
            );
        } else if JsClassDeclaration::can_cast(state.kind())
            || JsClassExpression::can_cast(state.kind())
        {
            diag = diag.detail(
                state.text_range(),
                markup! {"All parts of a class's body are already in strict mode."},
            );
        } else {
            // for redundant directive
            diag= diag.detail(
                state.text_range(),
                markup! {"This outer "<Emphasis>{"use strict"}</Emphasis>" directive already enables strict mode."},
            );
        }

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
