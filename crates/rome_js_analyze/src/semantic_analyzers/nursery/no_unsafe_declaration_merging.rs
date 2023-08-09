use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, TsDeclareStatement, TsExportDeclareClause,
    TsInterfaceDeclaration,
};
use rome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Disallow unsafe declaration merging between interfaces and classes.
    ///
    /// _TypeScript_'s [declaration merging](https://www.typescriptlang.org/docs/handbook/declaration-merging.html) supports merging separate declarations with the same name.
    ///
    /// _Declaration merging_ between classes and interfaces is unsafe.
    /// The _TypeScript Compiler_ doesn't check whether properties defined in the interface are initialized in the class.
    /// This can cause lead to _TypeScript_ not detecting code that will cause runtime errors.
    ///
    /// Source: https://typescript-eslint.io/rules/no-unsafe-declaration-merging/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface Foo {
    ///     f(): void
    /// }
    ///
    /// class Foo {}
    ///
    /// const foo = new Foo();
    /// foo.f(); // Runtime Error: Cannot read properties of undefined.
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// interface Foo {}
    /// class Bar implements Foo {}
    /// ```
    ///
    /// ```ts
    /// namespace Baz {}
    /// namespace Baz {}
    /// enum Baz {}
    /// ```
    pub(crate) NoUnsafeDeclarationMerging {
        version: "next",
        name: "noUnsafeDeclarationMerging",
        recommended: true,
    }
}

impl Rule for NoUnsafeDeclarationMerging {
    type Query = Semantic<TsInterfaceDeclaration>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ts_interface = ctx.query();
        let model = ctx.model();
        let interface_binding = ts_interface.id().ok()?;
        let interface_name = interface_binding.name_token().ok()?;
        let scope = model.scope(ts_interface.syntax()).parent()?;
        for binding in scope.bindings() {
            if let Some(AnyJsBindingDeclaration::JsClassDeclaration(class)) =
                binding.tree().declaration()
            {
                // This is not unsafe of merging an interface and an ambient class.
                if class.parent::<TsDeclareStatement>().is_none()
                    && class.parent::<TsExportDeclareClause>().is_none()
                {
                    if let Ok(id) = class.id() {
                        if let Some(id) = id.as_js_identifier_binding() {
                            if let Ok(name) = id.name_token() {
                                if name.text() == interface_name.text() {
                                    return Some(name.text_trimmed_range());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, class_range: &Self::State) -> Option<RuleDiagnostic> {
        let ts_interface = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                class_range,
                markup! {
                    "This "<Emphasis>"class"</Emphasis>" is unsafely merged with an "<Emphasis>"interface"</Emphasis>"."
                },
            )
            .detail(ts_interface.id().ok()?.range(), markup! {
                "The "<Emphasis>"interface"</Emphasis>" is declared here."
            })
            .note(markup! {
                "The TypeScript compiler doesn't check whether properties defined in the interface are initialized in the class."
            }),
        )
    }
}
