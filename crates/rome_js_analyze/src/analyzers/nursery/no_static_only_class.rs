use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsClass, AnyJsClassMember};
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
    ///
    /// Users who come from a [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming) paradigm may wrap their utility functions in an extra class,
    /// instead of putting them at the top level of an ECMAScript module. Doing so is generally unnecessary in JavaScript and TypeScript projects.
    ///
    /// - Wrapper classes add extra cognitive complexity to code without adding any structural improvements
    /// 	- Whatever would be put on them, such as utility functions, are already organized by virtue of being in a module.
    /// 	- As an alternative, you can import * as ... the module to get all of them in a single object.
    /// - IDEs can't provide as good suggestions for static class or namespace imported properties when you start typing property names
    /// - It's more difficult to statically analyze code for unused variables, etc. when they're all on the class (see: Finding dead code (and dead types) in TypeScript).
    ///
    /// Source: https://typescript-eslint.io/rules/no-extraneous-class
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
    /// ## Notes on Mutating Variables
    /// One case you need to be careful of is exporting mutable variables. While class properties can be mutated externally, exported variables are always constant. This means that importers can only ever read the first value they are assigned and cannot write to the variables.
    ///
    /// Needing to write to an exported variable is very rare and is generally considered a code smell. If you do need it you can accomplish it using getter and setter functions:
    /// ```js,expect_diagnostic
    /// export class Utilities {
    ///   static mutableCount = 1;
    ///   static incrementCount() {
    ///     Utilities.mutableCount += 1;
    ///   }
    /// }
    /// ```
    ///
    /// Do this instead:
    /// ```js
    /// let mutableCount = 1;
    ///
    /// export function getMutableCount() {
    ///   return mutableField;
    /// }
    ///
    /// export function incrementCount() {
    ///   mutableField += 1;
    /// }
    /// ```
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
        if class_declaration.members().is_empty() {
            return None;
        }

        // If the class has decorators, we can't be sure that the user can use a module-based approach instead
        if class_declaration.decorators().len() > 0 {
            return None;
        }

        let all_members_static = class_declaration
            .members()
            .iter()
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

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                class_declaration.syntax().text_trimmed_range(),
                markup! {
                    "Avoid classes that contain only static fields."
                },
            )
            .note(markup! {
                "Prefer using simple functions instead of classes with only static fields."
            }),
        )
    }
}
