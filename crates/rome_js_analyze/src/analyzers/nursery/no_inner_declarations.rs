use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsDeclaration, JsExport, JsFunctionBody, JsModuleItemList, JsScript, JsStatementList,
    JsStaticInitializationBlockClassMember,
};
use rome_rowan::AstNode;

use crate::control_flow::AnyJsControlFlowRoot;

declare_rule! {
    /// Disallow `function` and `var` declarations in nested blocks.
    ///
    /// A `function` and a `var` are accessible in the whole body of the
    /// nearest root (function, module, script, static block).
    /// To avoid confusion, they should be declared to the nearest root.
    /// Note that `const` and `let` declarations are block-scoped, and therefore
    /// they are not affected by this rule.
    ///
    /// Moreover, prior to ES2015 a function declaration is only allowed in
    /// the nearest root, though parsers sometimes erroneously accept them elsewhere.
    /// This only applies to function declarations; named or anonymous function
    /// expressions can occur anywhere an expression is permitted.
    ///
    /// Source: https://eslint.org/docs/rules/no-inner-declarations
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (test) {
    ///     function f() {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (test) {
    ///     var x = 1;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     if (test) {
    ///         function g() {}
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     if (test) {
    ///         var x = 1;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f() { }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     function g() {}
    /// }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     var x = 1;
    /// }
    /// ```
    ///
    /// ```js
    /// function f() {
    ///     if (test) {
    ///         const g = function() {};
    ///     }
    /// }
    /// ```
    ///
    pub(crate) NoInnerDeclarations {
        version: "next",
        name: "noInnerDeclarations",
        recommended: true,
    }
}

impl Rule for NoInnerDeclarations {
    type Query = Ast<AnyJsDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let decl = ctx.query();
        let parent = match decl {
            AnyJsDeclaration::JsFunctionDeclaration(x) => x.syntax().parent(),
            AnyJsDeclaration::JsVariableDeclaration(x) => {
                if x.is_var() {
                    // ignore parent (JsVariableStatement or JsVariableDeclarationClause)
                    x.syntax().parent()?.parent()
                } else {
                    None
                }
            }
            _ => None,
        }?;
        if JsExport::can_cast(parent.kind()) || JsModuleItemList::can_cast(parent.kind()) {
            return None;
        }
        if let Some(stmt_list) = JsStatementList::cast(parent) {
            let parent_kind = stmt_list.syntax().parent()?.kind();
            if JsFunctionBody::can_cast(parent_kind)
                || JsScript::can_cast(parent_kind)
                || JsStaticInitializationBlockClassMember::can_cast(parent_kind)
            {
                return None;
            }
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let decl = ctx.query();
        let decl_type = match decl {
            AnyJsDeclaration::JsFunctionDeclaration(_) => "function",
            _ => "var",
        };
        let nearest_root = decl
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyJsControlFlowRoot::cast)?;
        let nearest_root_type = match nearest_root {
            AnyJsControlFlowRoot::JsModule(_) => "module",
            AnyJsControlFlowRoot::JsScript(_) => "script",
            AnyJsControlFlowRoot::JsStaticInitializationBlockClassMember(_) => "static block",
            _ => "enclosing function",
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            decl.range(),
            markup! {
                "This "<Emphasis>{decl_type}</Emphasis>" should be declared at the root of the "<Emphasis>{nearest_root_type}</Emphasis>"."
            },
        ).note(markup! {
            "The "<Emphasis>{decl_type}</Emphasis>" is accessible in the whole body of the "<Emphasis>{nearest_root_type}</Emphasis>".\nTo avoid confusion, it should be declared at the root of the "<Emphasis>{nearest_root_type}</Emphasis>"."
        }))
    }
}
