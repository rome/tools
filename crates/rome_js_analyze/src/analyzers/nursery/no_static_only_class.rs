use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsClass, AnyJsClassMember};
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://eslint.org/docs/latest/rules/rule-name
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class X {
    ///   static foo = false;
    ///   static bar() {};
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// class StaticConstants {
    ///   static readonly version = 42;
    ///
    ///   static isProduction() {
    ///     return process.env.NODE_ENV === 'production';
    ///   }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// const X = {
    ///   foo: false,
    ///   bar() {}
    /// };
    /// ```
    /// ```js
    /// export const version = 42;
    ///
    /// export function isProduction() {
    ///   return process.env.NODE_ENV === 'production';
    /// }
    ///
    /// function logHelloWorld() {
    ///   console.log('Hello, world!');
    /// }
    /// ```
    /// ```js
    /// class Empty {}
    /// ```
    ///
    pub(crate) NoStaticOnlyClass {
        version: "next",
        name: "noStaticOnlyClass",
        recommended: true,
    }
}

impl Rule for NoStaticOnlyClass {
    type Query = Ast<AnyJsClass>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_declaration = ctx.query();

        // If the class is empty, we can't be sure that the user can use a module-based approach instead
        // Also, the user might be in the process of writing a class, so we won't warn them yet
        if class_declaration.members().len() == 0 {
            return None;
        }

        // If the class has decorators, we can't be sure that the user can use a module-based approach instead
        if class_declaration.decorators().len() > 0 {
            return None;
        }

        let all_members_static =
            class_declaration
                .members()
                .into_iter()
                .all(|member| match member {
                    AnyJsClassMember::JsBogusMember(_) => false,
                    AnyJsClassMember::JsEmptyClassMember(_) => true, // treat this as static, since it doesn't do anything
                    AnyJsClassMember::JsConstructorClassMember(_) => false, // See GH#4482: Constructors are not regarded as static
                    AnyJsClassMember::TsConstructorSignatureClassMember(_) => false, // See GH#4482: Constructors are not regarded as static
                    AnyJsClassMember::JsGetterClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::JsMethodClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::JsPropertyClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::JsSetterClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true, // See GH#4482: Static initialization blocks are regarded as static
                    AnyJsClassMember::TsGetterSignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::TsIndexSignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::TsInitializedPropertySignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::TsMethodSignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::TsPropertySignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    AnyJsClassMember::TsSetterSignatureClassMember(m) => m
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some()),
                });
        if all_members_static {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let class_declaration = ctx.query();
        let class_keyword = class_declaration.class_token().ok()?;

        // As this rule matches against classes and anonymous classes (class expressions),
        // we highlight the class keyword if it has no identifier
        let range = class_declaration
            .id()
            .ok()?
            .map_or(class_keyword.text_trimmed_range(), |id| {
                id.syntax().text_trimmed_range()
            });

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "A class that includes only "<Emphasis>"static members"</Emphasis>" is confusing."
                },
            )
            .note(markup! {
                "Consider using a module or a plain object instead."
            }),
        )
    }
}
