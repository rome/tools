use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsEmptyClassMember, JsEmptyStatement, JsSyntaxKind};

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
    type State = AnyJsExtraSemicolonOptionType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        match node {
            AnyJsExtraSemicolon::JsEmptyStatement(stmt) => {
                let parent = stmt.syntax().parent()?;
                let allowed_parent_kinds = vec![
                    JsSyntaxKind::JS_FOR_STATEMENT,
                    JsSyntaxKind::JS_FOR_IN_STATEMENT,
                    JsSyntaxKind::JS_FOR_OF_STATEMENT,
                    JsSyntaxKind::JS_WHILE_STATEMENT,
                    JsSyntaxKind::JS_DO_WHILE_STATEMENT,
                    JsSyntaxKind::JS_IF_STATEMENT,
                    JsSyntaxKind::JS_LABELED_STATEMENT,
                    JsSyntaxKind::JS_WITH_STATEMENT,
                ];
                let has_allowed_parent = allowed_parent_kinds.contains(&parent.kind());
                if !has_allowed_parent {
                    Some(AnyJsExtraSemicolonOptionType::JsEmptyStatement(
                        stmt.clone(),
                    ))
                } else {
                    None
                }
            }
            AnyJsExtraSemicolon::JsEmptyClassMember(stmt) => Some(
                AnyJsExtraSemicolonOptionType::JsEmptyClassMember(stmt.clone()),
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
            AnyJsExtraSemicolonOptionType::JsEmptyStatement(stmt) => {
                mutation.remove_node(stmt.clone());
            }
            AnyJsExtraSemicolonOptionType::JsEmptyClassMember(stmt) => {
                mutation.remove_node(stmt.clone());
            }
        }
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove unnecessary semicolon." }.to_owned(),
            mutation,
        })
    }
}

pub enum AnyJsExtraSemicolonOptionType {
    JsEmptyStatement(JsEmptyStatement),
    JsEmptyClassMember(JsEmptyClassMember),
}
