use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsEmptyClassMember, JsEmptyStatement, JsSyntaxKind, T};

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
    type State = AnyJsExtraSemicolonOptionType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        match node {
            AnyJsExtraSemicolon::JsEmptyStatement(stmt) => {
                let parent = stmt.syntax().parent()?;
                let has_last_entity_in_parent = parent.kind() == JsSyntaxKind::JS_MODULE_ITEM_LIST;
                let has_entity_in_parent = parent.kind() == JsSyntaxKind::JS_FOR_STATEMENT;
                let empty_statements_in_list = parent
                    .children_with_tokens()
                    .into_iter()
                    .filter(|child| child.kind() == JsSyntaxKind::JS_EMPTY_STATEMENT)
                    .count();
                let has_empty_statements_in_list = has_last_entity_in_parent
                    && !has_entity_in_parent
                    && empty_statements_in_list > 0;
                let has_empty_statements_in_for_statement = !has_last_entity_in_parent
                    && has_entity_in_parent
                    && empty_statements_in_list > 1;
                let has_first_semicolon_in_node =
                    stmt.syntax().kind() == JsSyntaxKind::JS_EMPTY_STATEMENT;
                let has_empty_statements_in_module_list =
                    has_empty_statements_in_list && has_first_semicolon_in_node;
                let has_empty_statements_not_in_module_list =
                    !has_empty_statements_in_list && has_empty_statements_in_for_statement;
                let has_empty_statement_not_in_short_cases = !has_last_entity_in_parent
                    && !has_entity_in_parent
                    && has_first_semicolon_in_node;

                if has_empty_statements_in_module_list
                    || has_empty_statements_not_in_module_list
                    || has_empty_statement_not_in_short_cases
                {
                    Some(AnyJsExtraSemicolonOptionType::JsEmptyStatement(
                        stmt.clone(),
                    ))
                } else {
                    None
                }
            }
            AnyJsExtraSemicolon::JsEmptyClassMember(stmt) => {
                let has_first_semicolon_in_node = stmt.syntax().first_token()?.kind() == T![;];

                if has_first_semicolon_in_node {
                    Some(AnyJsExtraSemicolonOptionType::JsEmptyClassMember(
                        stmt.clone(),
                    ))
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
