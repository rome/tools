use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsEmptyClassMember, JsEmptyStatement};

use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Typing mistakes and misunderstandings about where semicolons are required can lead to semicolons that are unnecessary.
    /// While not technically an error, extra semicolons can cause confusion when reading code.
    ///
    /// This rule disallows unnecessary semicolons.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    ///```js,expect_diagnostic
    ///  const x = 5;;
    ///```
    ///
    ///```js,expect_diagnostic
    /// function buzz() {
    ///     const x = 10;;
    /// }
    ///```
    ///
    ///```js,expect_diagnostic
    ///  function foo() {
    ///    // code
    ///  };
    ///```
    ///
    ///```js,expect_diagnostic
    ///    class C {
    ///      field;;
    ///
    ///      method() {
    ///          // code
    ///      }
    ///
    ///      static {
    ///          // code
    ///      }
    ///    }
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    class C {
    ///      field;
    ///
    ///      method() {
    ///          // code
    ///      };
    ///
    ///      static {
    ///          // code
    ///      }
    ///    }
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    class C {
    ///      field;
    ///
    ///      method() {
    ///          // code
    ///      }
    ///
    ///      static {
    ///          // code
    ///      };
    ///    }
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    class C {
    ///      field;
    ///
    ///      method() {
    ///          // code
    ///      }
    ///
    ///      static {
    ///          // code
    ///      }
    ///    };
    /// ```
    pub(crate) NoExtraSemicolons {
        version: "12.0.0",
        name: "noExtraSemicolons",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyJsExtraSemicolon = JsEmptyStatement | JsEmptyClassMember
}

impl Rule for NoExtraSemicolons {
    type Query = Ast<AnyJsExtraSemicolon>;
    type State = AnyJsExtraSemicolon;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        match node {
            AnyJsExtraSemicolon::JsEmptyStatement(stmt) => {
                let parent = stmt.syntax().parent()?;
                if parent.kind().is_list() {
                    Some(AnyJsExtraSemicolon::JsEmptyStatement(
                        stmt.clone(),
                    ))
                } else {
                    None
                }
            }
            AnyJsExtraSemicolon::JsEmptyClassMember(stmt) => Some(
                AnyJsExtraSemicolon::JsEmptyClassMember(stmt.clone()),
            ),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unnecessary semicolon."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, node_replace: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match node_replace {
            AnyJsExtraSemicolon::JsEmptyStatement(stmt) => {
                mutation.remove_node(stmt.clone());
            }
            AnyJsExtraSemicolon::JsEmptyClassMember(stmt) => {
                mutation.remove_node(stmt.clone());
            }
        }
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove unnecessary semicolon." }.to_owned(),
            mutation,
        })
    }
}
