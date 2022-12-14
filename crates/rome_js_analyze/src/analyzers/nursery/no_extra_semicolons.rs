use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsEmptyStatement, JsEmptyClassMember, JsSyntaxKind, T};

use rome_rowan::{declare_node_union, AstNode};

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
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        match node {
            AnyJsExtraSemicolon::JsEmptyStatement(stmt) => {
                let parent = stmt.syntax().parent()?;
                let has_last_entity_in_parent =
                    parent.kind() == JsSyntaxKind::JS_MODULE_ITEM_LIST;
                let has_empty_statements_in_list = parent
                    .children_with_tokens()
                    .into_iter()
                    .filter(|child| child.kind() == JsSyntaxKind::JS_EMPTY_STATEMENT)
                    .map(|child| {
                        let child = child.into_node().unwrap();
                        let child = JsEmptyStatement::cast(child).unwrap();
                        child.syntax().first_token().unwrap().kind() == T![;]
                    })
                    .filter(|value| *value)
                    .count()
                    > 0;
                let has_first_semicolon_in_node = stmt.syntax().first_token()?.kind() == T![;];
                let has_empty_statements = has_last_entity_in_parent
                    && has_empty_statements_in_list
                    && has_first_semicolon_in_node;

                if has_empty_statements {
                    Some(())
                } else {
                    None
                }
            },
            AnyJsExtraSemicolon::JsEmptyClassMember(stmt) => {
                let has_first_semicolon_in_node = stmt.syntax().first_token()?.kind() == T![;];

                if has_first_semicolon_in_node {
                    Some(())
                } else {
                    None
                }
            }
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
}
