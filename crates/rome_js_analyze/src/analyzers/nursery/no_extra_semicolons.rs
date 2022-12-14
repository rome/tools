use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsEmptyStatement, T,
};

use rome_rowan::{AstNode};

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
    ///```js,expect_diagnostic
    ///  function foo() {
    ///    // code
    ///  };
    /// ```
    ///```js,expect_diagnostic
    ///    class C {
    ///      field;;
    ///
    ///      method() {
    ///          // code
    ///      };
    ///
    ///      static {
    ///          // code
    ///      };
    ///    };
    /// ```
    pub(crate) NoExtraSemicolons {
        version: "12.0.0",
        name: "noExtraSemicolons",
        recommended: true,
    }
}

pub type JsExtraSemicolon = JsEmptyStatement;

impl Rule for NoExtraSemicolons {
    type Query = Ast<JsExtraSemicolon>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if let Some(semicolon) = node.syntax().first_token() {
          if semicolon.kind() == T![;] {
            Some(())
          } else {
            None
          }
        } else {
          None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unnecessary semicolons."
            },
        ))
    }
}
