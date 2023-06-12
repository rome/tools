use std::str::FromStr;

use crate::{
    control_flow::AnyJsControlFlowRoot,
    semantic_services::Semantic,
    utils::case::{Case, Decomposed},
    utils::rename::{AnyJsRenamableDeclaration, RenameSymbolExtensions},
    JsRuleAction,
};
use bpaf::Bpaf;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_deserialize::{
    json::{has_only_known_keys, with_only_known_variants, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use rome_diagnostics::Applicability;
use rome_js_semantic::CanBeImportedExported;
use rome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsClassMember, AnyJsObjectMember,
    AnyJsVariableDeclaration, AnyTsTypeMember, JsIdentifierBinding, JsLiteralExportName,
    JsLiteralMemberName, JsPrivateClassMemberName, JsSyntaxKind, JsSyntaxToken,
    JsVariableDeclarator, JsVariableKind, TsEnumMember, TsIdentifierBinding, TsTypeParameterName,
};
use rome_json_syntax::JsonLanguage;
use rome_rowan::{
    declare_node_union, AstNode, AstNodeList, BatchMutationExt, SyntaxNode, SyntaxResult,
};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce naming conventions for everything across a codebase.
    ///
    /// Enforcing naming conventions helps to keep the codebase consistent,
    /// and reduces overhead when thinking about how to name a variable.
    ///
    /// This rule enforces the wide-spread naming conventions of _JavaScript_ and _TypeScript_.
    ///
    /// ## Naming conventions
    ///
    /// All names can be prefixed and suffixed by one or two underscores `_`, or a dollar sign `$`.
    ///
    /// ### Variables
    ///
    /// All variables, including function parameters and catch parameters, should be in `camelCase`.
    ///
    /// ```js
    /// function f(param, _unusedParam) {
    ///     let localValue = 0;
    ///     try {
    ///         /* ... */
    ///     } catch (error) {
    ///         /* ... */
    ///     }
    /// }
    /// ```
    ///
    /// Additionally, top-level variables declared as `const` or `var` may be in `CONSTANT_CASE` or `PascalCase`.
    /// Top-level variables are declared at the module or script level.
    /// Variables declared in a TypeScript `module` or `namespace` are also considered as top-level.
    ///
    /// ```ts
    /// export const A_CONSTANT = 5;
    ///
    /// export const Person = class {}
    ///
    /// let aVariable = 0;
    ///
    /// export namespace ns {
    ///     export const ANOTHER_CONSTANT = "";
    /// }
    /// ```
    ///
    /// Examples of incorrect names:
    ///
    /// ```js,expect_diagnostic
    /// let a_value = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(Param) {}
    /// ```
    ///
    /// ### `function`
    ///
    /// A `function` name should be in `camelCase`.
    ///
    /// ```js
    /// function trim(s) { /*...*/ }
    /// ```
    ///
    /// `function` names in `PascalCase` are also supported.
    ///
    /// ### `import` and `export`
    ///
    /// ### `class`, `interface`, and literal object `type`
    ///
    /// A `class` and a _TypeScript_ `interface` names should be in `PascalCase`.
    ///
    /// Properties and methods should be in `camelCase`.
    ///
    /// ```js
    /// class Person {
    ///     property = 0;
    ///
    ///     method() {}
    /// }
    /// ```
    ///
    /// ```ts
    /// interface Named {
    ///     readonly property: string;
    ///
    ///     method(): void;
    /// }
    /// ```
    ///
    /// ```ts
    /// type Named = {
    ///     readonly property: string;
    ///
    ///     method(): void;
    /// };
    /// ```
    ///
    /// Static properties and getters can be in `CONSTANT_CASE`:
    ///
    /// ```js
    /// class Person {
    ///     static CONSTANT = 0;
    ///
    ///     static get INSTANCE() { /*...*/ }
    /// }
    /// ```
    ///
    /// An interface and a literal object type allow `readonly` properties and getter in `CONSTANT_CASE`.
    /// This enables to model a `class` constructor:
    ///
    /// ```ts
    /// interface PersonConstructor {
    ///     readonly CONSTANT: number;
    ///
    ///     get INSTANCE(): Person;
    ///
    ///     new(): Person;
    /// }
    /// ```
    ///
    /// ### `enum`
    ///
    /// A _TypeScript_ `enum` name should be in `PascalCase`.
    /// `enum` members are by default in `PascalCase`.
    /// However, you can configure the case of `enum` members.
    /// See [options](#options) for more details.
    ///
    /// ```ts
    /// enum Status {
    ///     Open,
    ///     Close,
    /// }
    /// ```
    ///
    /// ### `namespace`
    ///
    /// A _TypeScript_ `namespace` should be either in `camelCase` or in `PascalCase`.
    ///
    /// ```ts
    /// namespace str {
    ///     /*...*/
    /// }
    /// ```
    ///
    /// ### Literal objects
    ///
    /// Properties and methods of literal objects should be in `camelCase`.
    ///
    /// ```js
    /// const book = {
    ///     authorName: "Isaac Asimov",
    ///     name: "Foundation"
    /// };
    /// ```
    ///
    /// ### `type` alias
    ///
    /// A _TypeScript_ `type` alias should be in `PascalCase`:
    ///
    /// ```ts
    /// type Person = { name: string };
    /// ```
    ///
    /// A `type` that aliases a primitive type can also be in `camelCase`:
    ///
    /// ```ts
    /// type integer = number;
    /// ```
    ///
    /// Examples of incorrect names:
    ///
    /// ```ts,expect_diagnostic
    /// type PERSON = { name: string };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type bool = Boolean;
    /// ```
    /// ### `import` and `export`
    ///
    /// Namespaced `import * as` and `export * as` should be in `camelCase`.
    ///
    /// ```js
    /// import * as myLib from "myLib";
    ///
    /// export * as myLib from "myLib";
    /// ```
    ///
    /// `import` and `export` aliases should be in `camelCase`, `PascalCase`, or `CONSTANT_CASE`:
    ///
    /// ```js
    /// import assert, {
    ///     deepStrictEqual as deepEqual,
    ///     AssertionError as AssertError
    /// } from "node:assert";
    /// ```
    ///
    /// ### Type parameters `<T>`
    ///
    /// _Type parameters_, also named _generics_, may appear in _TypeScript_ source files.
    /// They are declared between enclosing angle brackets.
    ///
    /// A type parameter name must consist of a uppercase character and may be followed by a number.
    /// Thus, `T`, `T1`, and `T42` are valid names.
    ///
    /// ```ts
    /// function id<T>(val: T): T { return val; }
    ///
    /// type Pair<E1, E2> = [E1, E2];
    /// ```
    ///
    /// Example of incorrect name:
    ///
    /// ```ts,expect_diagnostic
    /// function id<Val>(val: Val): T { return val; }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides two options that are detailed in the following subsections.
    /// The default configuration is as follows:
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "strictCase": true,
    ///         "enumMemberCase": "PascalCase"
    ///     }
    /// }
    /// ```
    ///
    /// ### strictCase
    ///
    /// When this option is set to `true`, it forbids consecutive uppercase characters in `camelCase` and `PascalCase`.
    /// For instance, `HTTPServer` and `aHTTPServer` are not considered in `PascalCase` and in `camelCase` when the option is set to `true`.
    /// These names should be renamed to `HttpServer` and `aHttpServer`
    ///
    /// When the option is set to `false`, consecutive uppercase characters are allowed.
    /// `HTTPServer` and `aHTTPServer` are so valid.
    ///
    /// Default: _true_
    ///
    /// ### enumMemberCase
    ///
    /// By default, the rule enforces the naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
    /// an `enum` member should be in `PascalCase`.
    ///
    /// You can enforce another convention by setting `enumMemberCase` option.
    /// The supported cases are: `PascalCase`, `CONSTANT_CASE`, and `camelCase`.
    pub(crate)  UseNamingConvention {
        version: "next",
        name: "useNamingConvention",
        recommended: false,
    }
}

impl Rule for UseNamingConvention {
    type Query = Semantic<AnyName>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NamingConventionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let element = Named::from_name(node)?;
        let allowed_cases = element.allowed_cases(options);
        if allowed_cases.is_empty() {
            // No naming convention to verify.
            return None;
        }
        let name_token = node.name_token().ok()?;
        let name = name_token.text_trimmed();
        let Decomposed {
            prefix,
            main,
            suffix,
        } = Decomposed::from(name);
        let actual_case = Case::identify(main, options.strict_case);
        let issue = if !matches!(prefix, "" | "_" | "__" | "$") {
            Invalid::Prefix
        } else if !matches!(suffix, "" | "_" | "__" | "$") {
            Invalid::Suffix
        } else {
            if main.is_empty()
                || allowed_cases
                    .iter()
                    .any(|&expected_style| actual_case.is_compatible_with(expected_style))
            {
                // Valid case
                return None;
            }
            Invalid::Case(actual_case)
        };
        Some(State {
            element,
            invalid: issue,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State {
            element,
            invalid: issue,
        } = state;
        let node = ctx.query();
        let name_token = node.name_token().ok()?;
        let name = name_token.text_trimmed();
        let node_range = node.syntax().text_trimmed_range();
        let diagnostic = match issue {
            Invalid::Case(actual_case) => {
                let allowed_cases = element.allowed_cases(ctx.options());
                let allowed_case_names = allowed_cases
                    .iter()
                    .map(|style| style.to_string())
                    .collect::<SmallVec<[_; 3]>>()
                    .join(" or ");
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "This "<Emphasis>{element.to_string()}</Emphasis>" name should be in "<Emphasis>{allowed_case_names}</Emphasis>"."
                    },
                );
                diagnostic = diagnostic.note(markup! {
                    "The name is currently in "<Emphasis>{actual_case.to_string()}</Emphasis>"."
                });
                diagnostic
            }
            Invalid::Prefix => {
                let Decomposed { prefix, .. } = Decomposed::from(name);
                RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "This "<Emphasis>{element.to_string()}</Emphasis>" name might only prefixed by "<Emphasis>"_"</Emphasis>" or "<Emphasis>"$"</Emphasis>"."
                    },
                ).note(markup! {
                    "The current prefix is "<Emphasis>{prefix}</Emphasis>"."
                })
            }
            Invalid::Suffix => {
                let Decomposed { suffix, .. } = Decomposed::from(name);
                RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "This "<Emphasis>{element.to_string()}</Emphasis>" name might only be suffixed by "<Emphasis>"_"</Emphasis>" or "<Emphasis>"$"</Emphasis>"."
                    },
                ).note(markup! {
                    "The current suffix is "<Emphasis>{suffix}</Emphasis>"."
                })
            }
        };
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let model = ctx.model();
        let mut mutation = ctx.root().begin();
        let State { element, invalid } = state;
        let renamable = match node {
            AnyName::JsIdentifierBinding(binding) => {
                if binding.is_exported(model) {
                    return None;
                }
                if let Some(AnyJsBindingDeclaration::TsPropertyParameter(_)) = binding.declaration()
                {
                    // Property parameters are also class properties.
                    return None;
                }
                Some(AnyJsRenamableDeclaration::JsIdentifierBinding(
                    binding.clone(),
                ))
            }
            AnyName::TsIdentifierBinding(binding) => {
                if binding.is_exported(model) {
                    return None;
                }
                Some(AnyJsRenamableDeclaration::TsIdentifierBinding(
                    binding.clone(),
                ))
            }
            _ => None,
        };
        if let Some(renamable) = renamable {
            let name_token = node.name_token().ok()?;
            let Decomposed {
                mut prefix,
                main,
                mut suffix,
            } = Decomposed::from(name_token.text_trimmed());
            prefix = if prefix.contains("__") {
                "__"
            } else if prefix.contains('_') {
                "_"
            } else if prefix.contains('$') {
                "$"
            } else {
                ""
            };
            suffix = if suffix.contains("__") {
                "__"
            } else if suffix.contains('_') {
                "_"
            } else if suffix.contains('$') {
                "$"
            } else {
                ""
            };
            let message;
            let mut new_name;
            match invalid {
                Invalid::Case(_) => {
                    let preferred_case = element.allowed_cases(ctx.options())[0];
                    new_name = preferred_case.convert(main);
                    message = markup! { "Rename this symbol in "<Emphasis>{preferred_case.to_string()}</Emphasis>"." }.to_owned();
                }
                Invalid::Prefix | Invalid::Suffix => {
                    new_name = main.to_string();
                    message = markup! { "Rename with a recommended prefix and suffix." }.to_owned();
                }
            }
            new_name.insert_str(0, prefix);
            new_name.insert_str(new_name.len(), suffix);
            let renamed = mutation.rename_any_renamable_node(model, renamable, &new_name[..]);
            if renamed {
                return Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message,
                    mutation,
                });
            }
        }
        None
    }
}

declare_node_union! {
    /// Ast nodes that carries a name.
    pub(crate) AnyName =
        JsIdentifierBinding |
        JsLiteralMemberName |
        JsPrivateClassMemberName |
        JsLiteralExportName |
        TsIdentifierBinding |
        TsTypeParameterName
}

impl AnyName {
    fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyName::JsIdentifierBinding(binding) => binding.name_token(),
            AnyName::JsLiteralMemberName(member_name) => member_name.value(),
            AnyName::JsPrivateClassMemberName(member_name) => member_name.id_token(),
            AnyName::JsLiteralExportName(export_name) => export_name.value(),
            AnyName::TsIdentifierBinding(binding) => binding.name_token(),
            AnyName::TsTypeParameterName(type_parameter) => type_parameter.ident_token(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct State {
    element: Named,
    invalid: Invalid,
}

#[derive(Debug)]
enum Invalid {
    Case(Case),
    Prefix,
    Suffix,
}

/// Rule's options.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in [Case::camelCase] and [Case::PascalCase].
    /// This does not affect other [Case].
    #[bpaf(hide)]
    #[serde(
        default = "default_strict_case",
        skip_serializing_if = "is_default_strict_case"
    )]
    pub strict_case: bool,

    /// Allowed [Case] for _TypeScript_ `enum` member names.
    #[bpaf(hide)]
    #[serde(default, skip_serializing_if = "is_default")]
    pub enum_member_case: EnumMemberCase,
}

const fn default_strict_case() -> bool {
    true
}

const fn is_default_strict_case(strict_case: &bool) -> bool {
    *strict_case == default_strict_case()
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

impl NamingConventionOptions {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["strictCase", "enumMemberCase"];
}

impl Default for NamingConventionOptions {
    fn default() -> Self {
        Self {
            strict_case: default_strict_case(),
            enum_member_case: EnumMemberCase::default(),
        }
    }
}

// Required by [Bpaf].
impl FromStr for NamingConventionOptions {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // WARNING: should not be used.
        Ok(Self::default())
    }
}

impl VisitJsonNode for NamingConventionOptions {}
impl VisitNode<JsonLanguage> for NamingConventionOptions {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Self::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "strictCase" => {
                self.strict_case = self.map_to_boolean(&value, name_text, diagnostics)?
            }
            "enumMemberCase" => {
                let mut enum_member_case = EnumMemberCase::default();
                self.map_to_known_string(&value, name_text, &mut enum_member_case, diagnostics)?;
                self.enum_member_case = enum_member_case;
            }
            _ => (),
        }
        Some(())
    }
}

/// Supported cases for TypeScript `enum` member names.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum EnumMemberCase {
    /// PascalCase
    #[serde(rename = "PascalCase")]
    #[default]
    Pascal,

    /// CONSTANT_CASE
    #[serde(rename = "CONSTANT_CASE")]
    Constant,

    /// camelCase
    #[serde(rename = "camelCase")]
    Camel,
}

impl EnumMemberCase {
    pub const KNOWN_VALUES: &'static [&'static str] = &["camelCase", "CONSTANT_CASE", "PascalCase"];
}

/// Required by [Bpaf].
impl FromStr for EnumMemberCase {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // WARNING: should not be used.
        Ok(EnumMemberCase::default())
    }
}

impl VisitNode<JsonLanguage> for EnumMemberCase {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, Self::KNOWN_VALUES, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "PascalCase" => *self = Self::Pascal,
            "CONSTANT_CASE" => *self = Self::Constant,
            "camelCase" => *self = Self::Camel,
            _ => (),
        }
        Some(())
    }
}

impl From<EnumMemberCase> for Case {
    fn from(case: EnumMemberCase) -> Case {
        match case {
            EnumMemberCase::Pascal => Case::Pascal,
            EnumMemberCase::Constant => Case::Constant,
            EnumMemberCase::Camel => Case::Camel,
        }
    }
}

/// Named elements with an attached naming convention.
///
/// [Named::from_name] enables to get the element from an [AnyName].
/// [Named::allowed_cases] enables to get a list of allowed cases for a given element.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Named {
    CatchParameter,
    Class,
    ClassGetter,
    ClassMethod,
    ClassProperty,
    ClassSetter,
    ClassStaticGetter,
    ClassStaticMethod,
    ClassStaticProperty,
    ClassStaticSetter,
    Enum,
    EnumMember,
    ExportAlias,
    ExportSource,
    Function,
    FunctionParameter,
    ImportAlias,
    ImportNamespace,
    ImportSource,
    IndexParameter,
    Interface,
    LocalConst,
    LocalLet,
    LocalVar,
    Namespace,
    ObjectGetter,
    ObjectMethod,
    ObjectProperty,
    ObjectSetter,
    ParameterProperty,
    PrimitiveTypeAlias,
    TopLevelConst,
    TopLevelLet,
    TopLevelVar,
    TypeAlias,
    TypeGetter,
    TypeMethod,
    TypeProperty,
    TypeReadonlyProperty,
    TypeSetter,
    TypeParameter,
}

impl Named {
    fn from_name(js_name: &AnyName) -> Option<Named> {
        match js_name {
            AnyName::JsIdentifierBinding(binding) => {
                Named::from_binding_declaration(&binding.declaration()?)
            }
            AnyName::TsIdentifierBinding(binding) => {
                Named::from_binding_declaration(&binding.declaration()?)
            }
            AnyName::JsLiteralMemberName(member_name) => {
                // ignore quoted names
                if member_name.value().ok()?.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                    return None;
                }
                if let Some(member) = member_name.parent::<AnyJsClassMember>() {
                    Named::from_class_member(&member)
                } else if let Some(member) = member_name.parent::<AnyTsTypeMember>() {
                    Named::from_type_member(&member)
                } else if let Some(member) = member_name.parent::<AnyJsObjectMember>() {
                    Named::from_object_member(&member)
                } else if member_name.parent::<TsEnumMember>().is_some() {
                    Some(Named::EnumMember)
                } else {
                    None
                }
            }
            AnyName::JsPrivateClassMemberName(member_name) => {
                Named::from_class_member(&member_name.parent::<AnyJsClassMember>()?)
            }
            AnyName::JsLiteralExportName(export_name) => {
                match export_name.syntax().parent()?.kind() {
                    JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => Some(Named::ImportSource),
                    JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => Some(Named::ExportSource),
                    JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER | JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                        Some(Named::ExportAlias)
                    }
                    _ => None,
                }
            }
            AnyName::TsTypeParameterName(_) => Some(Named::TypeParameter),
        }
    }

    fn from_class_member(member: &AnyJsClassMember) -> Option<Named> {
        match member {
            AnyJsClassMember::JsBogusMember(_)
            | AnyJsClassMember::JsConstructorClassMember(_)
            | AnyJsClassMember::TsConstructorSignatureClassMember(_)
            | AnyJsClassMember::JsEmptyClassMember(_)
            | AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => None,
            AnyJsClassMember::TsIndexSignatureClassMember(_) => Some(Named::IndexParameter),
            AnyJsClassMember::JsGetterClassMember(getter) => {
                let is_static = getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                let is_static = getter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::JsMethodClassMember(method) => {
                let is_static = method
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                let is_static = method
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::JsPropertyClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsPropertySignatureClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                let is_static = property
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::JsSetterClassMember(setter) => {
                let is_static = setter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticSetter
                } else {
                    Named::ClassSetter
                })
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                let is_static = setter
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Some(if is_static {
                    Named::ClassStaticSetter
                } else {
                    Named::ClassSetter
                })
            }
        }
    }

    fn from_binding_declaration(decl: &AnyJsBindingDeclaration) -> Option<Named> {
        match decl {
            AnyJsBindingDeclaration::JsVariableDeclarator(var) => {
                Named::from_variable_declarator(var)
            }
            AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_) => Some(Named::FunctionParameter),
            AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(Named::CatchParameter),
            AnyJsBindingDeclaration::TsPropertyParameter(_) => Some(Named::ParameterProperty),
            AnyJsBindingDeclaration::TsIndexSignatureParameter(_) => Some(Named::IndexParameter),
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::JsImportNamespaceClause(_) => Some(Named::ImportNamespace),
            AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
                Some(Named::Function)
            }
            AnyJsBindingDeclaration::JsImportDefaultClause(_)
            | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => Some(Named::ImportAlias),
            AnyJsBindingDeclaration::TsModuleDeclaration(_) => Some(Named::Namespace),
            AnyJsBindingDeclaration::TsTypeAliasDeclaration(type_alias) => {
                let ty = type_alias.ty().ok()?.omit_parentheses();
                if ty.is_literal_type() || ty.is_primitive_type() {
                    Some(Named::PrimitiveTypeAlias)
                } else {
                    Some(Named::TypeAlias)
                }
            }
            AnyJsBindingDeclaration::JsClassDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExpression(_)
            | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => Some(Named::Class),
            AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => Some(Named::Interface),
            AnyJsBindingDeclaration::TsEnumDeclaration(_) => Some(Named::Enum),
            AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_) => {
                Some(Named::ImportSource)
            }
            AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_) => None,
        }
    }

    fn from_variable_declarator(var: &JsVariableDeclarator) -> Option<Named> {
        let is_top_level_level = matches!(
            var.syntax()
                .ancestors()
                .find_map(AnyJsControlFlowRoot::cast),
            Some(AnyJsControlFlowRoot::JsModule(_)) | Some(AnyJsControlFlowRoot::JsScript(_))
        );
        let var_declaration = var
            .syntax()
            .ancestors()
            .find_map(AnyJsVariableDeclaration::cast)?;
        let var_kind = var_declaration.variable_kind().ok()?;
        Some(match (var_kind, is_top_level_level) {
            (JsVariableKind::Const, false) => Named::LocalConst,
            (JsVariableKind::Let, false) => Named::LocalLet,
            (JsVariableKind::Var, false) => Named::LocalVar,
            (JsVariableKind::Const, true) => Named::TopLevelConst,
            (JsVariableKind::Let, true) => Named::TopLevelLet,
            (JsVariableKind::Var, true) => Named::TopLevelVar,
        })
    }

    fn from_object_member(member: &AnyJsObjectMember) -> Option<Named> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) | AnyJsObjectMember::JsSpread(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => Some(Named::ObjectGetter),
            AnyJsObjectMember::JsMethodObjectMember(_) => Some(Named::ObjectMethod),
            AnyJsObjectMember::JsPropertyObjectMember(_)
            | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => Some(Named::ObjectProperty),
            AnyJsObjectMember::JsSetterObjectMember(_) => Some(Named::ObjectSetter),
        }
    }

    fn from_type_member(member: &AnyTsTypeMember) -> Option<Named> {
        match member {
            AnyTsTypeMember::JsBogusMember(_)
            | AnyTsTypeMember::TsCallSignatureTypeMember(_)
            | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => None,
            AnyTsTypeMember::TsIndexSignatureTypeMember(_) => Some(Named::IndexParameter),
            AnyTsTypeMember::TsGetterSignatureTypeMember(_) => Some(Named::TypeGetter),
            AnyTsTypeMember::TsMethodSignatureTypeMember(_) => Some(Named::TypeMethod),
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                Some(if property.readonly_token().is_some() {
                    Named::TypeReadonlyProperty
                } else {
                    Named::TypeProperty
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_) => Some(Named::TypeSetter),
        }
    }

    /// Returns the list of allowed [Case] for `self`.
    /// The preferred case comes first in the list.
    fn allowed_cases(self, options: &NamingConventionOptions) -> SmallVec<[Case; 3]> {
        match self {
            Named::CatchParameter
            | Named::ClassGetter
            | Named::ClassMethod
            | Named::ClassProperty
            | Named::ClassSetter
            | Named::ClassStaticMethod
            | Named::ClassStaticSetter
            | Named::FunctionParameter
            | Named::ImportNamespace
            | Named::IndexParameter
            | Named::LocalConst
            | Named::LocalLet
            | Named::LocalVar
            | Named::ObjectGetter
            | Named::ObjectMethod
            | Named::ObjectProperty
            | Named::ObjectSetter
            | Named::ParameterProperty
            | Named::TopLevelLet
            | Named::TypeMethod
            | Named::TypeProperty
            | Named::TypeSetter => SmallVec::from_slice(&[Case::Camel]),
            Named::Class | Named::Enum | Named::Interface | Named::TypeAlias => {
                SmallVec::from_slice(&[Case::Pascal])
            }
            Named::ClassStaticGetter
            | Named::ClassStaticProperty
            | Named::TypeReadonlyProperty
            | Named::TypeGetter => SmallVec::from_slice(&[Case::Camel, Case::Constant]),
            Named::EnumMember => SmallVec::from_slice(&[options.enum_member_case.into()]),
            Named::ExportAlias | Named::ImportAlias | Named::TopLevelConst | Named::TopLevelVar => {
                SmallVec::from_slice(&[Case::Camel, Case::Pascal, Case::Constant])
            }
            Named::ExportSource | Named::ImportSource => SmallVec::new(),
            Named::Function | Named::Namespace | Named::PrimitiveTypeAlias => {
                SmallVec::from_slice(&[Case::Camel, Case::Pascal])
            }
            Named::TypeParameter => SmallVec::from_slice(&[Case::NumberableCapital]),
        }
    }
}

impl std::fmt::Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Named::CatchParameter => "catch parameter",
            Named::Class => "class",
            Named::ClassGetter => "class getter",
            Named::ClassMethod => "class method",
            Named::ClassProperty => "class property",
            Named::ClassSetter => "class setter",
            Named::ClassStaticGetter => "static getter",
            Named::ClassStaticMethod => "static method",
            Named::ClassStaticProperty => "static property",
            Named::ClassStaticSetter => "static setter",
            Named::Enum => "enum",
            Named::EnumMember => "enum member",
            Named::ExportAlias => "export alias",
            Named::ExportSource => "export source",
            Named::Function => "function",
            Named::FunctionParameter => "function parameter",
            Named::ImportAlias => "import alias",
            Named::ImportNamespace => "import namespace",
            Named::ImportSource => "import source",
            Named::IndexParameter => "index parameter",
            Named::Interface => "interface",
            Named::LocalConst => "local const",
            Named::LocalLet => "local let",
            Named::LocalVar => "local var",
            Named::Namespace => "namespace",
            Named::ObjectGetter => "object getter",
            Named::ObjectMethod => "object method",
            Named::ObjectProperty => "object property",
            Named::ObjectSetter => "object setter",
            Named::ParameterProperty => "parameter property",
            Named::PrimitiveTypeAlias => "primitive type alias",
            Named::TopLevelConst => "top-level const",
            Named::TopLevelLet => "top-level let",
            Named::TopLevelVar => "top-level var",
            Named::TypeAlias => "type alias",
            Named::TypeGetter => "getter",
            Named::TypeMethod => "method",
            Named::TypeProperty => "property",
            Named::TypeReadonlyProperty => "readonly property",
            Named::TypeSetter => "setter",
            Named::TypeParameter => "type parameter",
        };
        write!(f, "{}", repr)
    }
}
