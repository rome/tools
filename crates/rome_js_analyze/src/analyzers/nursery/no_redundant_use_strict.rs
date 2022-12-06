use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    AnyJsClass, JsClassDeclaration, JsClassExpression, JsDirective, JsDirectiveList,
    JsFunctionBody, JsLanguage, JsModule, JsScript,
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
declare_node_union! { pub(crate) AnyJsStrictModeNode = AnyJsClass| JsModule | JsDirective  }

impl Rule for NoRedundantUseStrict {
    type Query = Ast<JsDirective>;
    type State = AnyJsStrictModeNode;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut outer_most: Option<AnyJsStrictModeNode> = None;
        let root = ctx.root();
        match root {
            rome_js_syntax::AnyJsRoot::JsModule(js_module) => outer_most = Some(js_module.into()),
            _ => {
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
                    if let Some(module_or_class) = AnyJsClass::cast_ref(&n) {
                        outer_most = Some(module_or_class.into());
                    }
                }
            }
        }

        if let Some(outer_most) = outer_most {
            // skip itself
            if outer_most.syntax() == node.syntax() {
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

        match state {
            AnyJsStrictModeNode::AnyJsClass(js_class) =>  diag = diag.detail(
                js_class.range(),
                markup! {"All parts of a class's body are already in strict mode."},
            ) ,
            AnyJsStrictModeNode::JsModule(_js_module) => diag= diag.note(
                markup! {"The entire contents of "<Emphasis>{"JavaScript modules are automatically"}</Emphasis>" in strict mode, with no statement needed to initiate it."},
            ),
            AnyJsStrictModeNode::JsDirective(js_directive) => diag= diag.detail(
                js_directive.range(),
                markup! {"This outer "<Emphasis>{"use strict"}</Emphasis>" directive already enables strict mode."},
            ),
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
