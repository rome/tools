use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
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
    ///
    pub(crate) NoStaticOnlyClass {
        version: "next",
        name: "noStaticOnlyClass",
        recommended: false,
    }
}

impl Rule for NoStaticOnlyClass {
    type Query = Semantic<AnyJsClass>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_declaration = ctx.query();

        // If the class has decorators, we can't be sure that the suer can use a module-based approach instead
        if class_declaration.decorators().len() > 0 {
            return None;
        }

        let all_members_static =
            class_declaration
                .members()
                .into_iter()
                .all(|member| match member {
                    // TODO: Clean up this mess
                    AnyJsClassMember::JsBogusMember(_) => true, // TODO: What is this?
                    AnyJsClassMember::JsConstructorClassMember(_) => false, // See GH#4482: Constructors are not regarded as static
                    AnyJsClassMember::JsEmptyClassMember(_) => true, // treat this as static, since it doesn't do anything
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
                    AnyJsClassMember::TsConstructorSignatureClassMember(_) => false, // TODO: What to do with TS types?
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

        let diagnostic_node = class_declaration
            .id()
            .ok()?
            .map_or(class_keyword.text_trimmed_range(), |id| {
                id.syntax().text_trimmed_range()
            });

        // TODO: Wording
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                diagnostic_node,
                markup! {
                    "Don't use static classes as namespaces."
                },
            )
            .note(markup! {
                "Consider using a module or a plain object instead."
            }),
        )
    }
}
