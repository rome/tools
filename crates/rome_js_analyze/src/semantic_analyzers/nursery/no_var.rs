use crate::{control_flow::AnyJsControlFlowRoot, semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsModule, JsScript, JsSyntaxKind};

use rome_rowan::{AstNode, BatchMutationExt};

use super::use_const::{ConstBindings, VariableDeclaration};

declare_rule! {
    /// Disallow the use of `var`
    ///
    /// ECMAScript 6 allows programmers to create variables with block scope instead of function scope using the let and const keywords. Block scope is common in many other programming languages and helps programmers avoid mistakes.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-var
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 1;
    /// let bar = 1;
    ///```
    pub(crate) NoVar {
        version: "11.0.0",
        name: "noVar",
        recommended: true,
    }
}

impl Rule for NoVar {
    type Query = Semantic<VariableDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        declaration.is_var().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();
        let var_scope = declaration
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))?;
        let contextual_note = if JsScript::can_cast(var_scope.kind()) {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" in the global scope pollutes the global object."
            }
        } else if JsModule::can_cast(var_scope.kind()) {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" is accessible in the whole module. Thus, the variable can be accessed before its initialization and outside the block where it is declared."
            }
        } else {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" is accessible in the whole body of the function. Thus, the variable can be accessed before its initialization and outside the block where it is declared."
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            declaration.range(),
            markup! {
                "Use "<Emphasis>"let"</Emphasis>" or "<Emphasis>"const"</Emphasis>" instead of "<Emphasis>"var"</Emphasis>"."
            },
        ).note(contextual_note).note(
            markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var">"MDN web docs"</Hyperlink>" for more details."
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let declaration = ctx.query();
        let model = ctx.model();
        let maybe_const = ConstBindings::new(declaration, model)?;
        let replacing_token_kind = if maybe_const.can_fix {
            JsSyntaxKind::CONST_KW
        } else {
            JsSyntaxKind::LET_KW
        };
        let mut mutation = ctx.root().begin();
        mutation.replace_token(declaration.kind_token()?, make::token(replacing_token_kind));
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use '"<Emphasis>{replacing_token_kind.to_string()?}</Emphasis>"' instead." }.to_owned(),
            mutation,
        })
    }
}
