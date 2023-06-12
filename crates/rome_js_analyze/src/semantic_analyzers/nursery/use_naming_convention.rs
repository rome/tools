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
    binding_ext::AnyJsBindingDeclaration,
    member_name_ext::{AnyJsMember, AnyJsNamedExport},
    AnyJsClassMember, AnyJsObjectMember, AnyTsTypeMember, JsIdentifierBinding, JsLiteralExportName,
    JsLiteralMemberName, JsPrivateClassMemberName, JsSyntaxKind, JsSyntaxToken,
    JsVariableDeclarator, JsVariableKind, TsIdentifierBinding, TsTypeParameterName,
};
use rome_json_syntax::JsonLanguage;
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxNode, SyntaxResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Enforce naming conventions for everything across a codebase.
    ///
    /// Enforcing naming conventions helps keep the codebase consistent,
    /// and reduces overhead when thinking about how to name a variable.
    ///
    /// This rule enforces the wide-spread naming conventions of JavaScript and TypeScript codebase.
    ///
    /// In contrast to the _ESLint_ `naming-convention` rule, this rule is not fine-tunable.
    /// By default, an enum member has to be in _StrictUpperPascalCase_,
    /// while the _ESLint_ requires an enum member to be in _strictCamelCase_.
    /// This rule also supports to require an enum member to be in _CONSTANT_CASE_.
    ///
    /// Source: https://typescript-eslint.io/rules/naming-convention/#allowed-selectors-modifiers-and-types
    ///
    /// ## Options
    ///
    /// ### strictCase
    ///
    /// Default: _true_
    ///
    /// By default, `HTTPServer` and `aHTTPServer` are not considered in `PascalCase` and in `camelCase`.
    /// They have to be written as `HttpServer` and `aHttpServer`.
    /// By setting `strictCase` to `false`, consecutive uppercase characters are allowed.
    ///
    /// ### enumMemberCase
    ///
    /// Default: `PascalCase`
    ///
    /// By default, the rule enforces tha naming convention followed by the [TypeScript Compiler team](https://www.typescriptlang.org/docs/handbook/enums.html):
    /// an `enum` member has to be in `PascalCase`.
    /// You can enforce another convention by setting `enumMemberCase`.
    /// The supported cases are: `camelCase`, `CONSTANT_CASE`, and `PascalCase`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let snake_case;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class camelCase {}
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let camelCase;
    /// ```
    ///
    /// ```js
    /// const CONSTANT_CASE = 0;
    /// const camelCase = {};
    /// ```
    ///
    /// ```js
    /// class PascalCase {}
    /// ```
    pub(crate)  UseNamingConvention {
        version: "next",
        name: "useNamingConvention",
        recommended: true,
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
        let allowed_cases = element.naming_convention(options);
        if allowed_cases.is_empty() {
            // No naming convention to verify
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
            if let Some(actual_case) = actual_case {
                if allowed_cases
                    .iter()
                    .any(|&expected_style| actual_case.is_compatible(expected_style))
                {
                    // Valid case
                    return None;
                }
            } else if main.is_empty() {
                return None;
            }
            Invalid::Case(actual_case)
        };
        Some(State { element, issue })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State { element, issue } = state;
        let node = ctx.query();
        let name_token = node.name_token().ok()?;
        let name = name_token.text_trimmed();
        let node_range = node.syntax().text_trimmed_range();
        let diagnostic = match issue {
            Invalid::Case(actual_case) => {
                let allowed_cases = element.naming_convention(ctx.options());
                let allowed_case_names = allowed_cases
                    .iter()
                    .map(|style| style.to_str())
                    .collect::<SmallVec<[_; 3]>>()
                    .join(" or ");
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "This "<Emphasis>{element.to_str()}</Emphasis>" name should be in "<Emphasis>{allowed_case_names}</Emphasis>"."
                    },
                );
                if let Some(actual_case) = actual_case {
                    diagnostic = diagnostic.note(markup! {
                        "The name is currently in "<Emphasis>{actual_case.to_str()}</Emphasis>"."
                    })
                }
                diagnostic
            }
            Invalid::Prefix => {
                let Decomposed { prefix, .. } = Decomposed::from(name);
                RuleDiagnostic::new(
                    rule_category!(),
                    node_range,
                    markup! {
                        "This "<Emphasis>{element.to_str()}</Emphasis>" name might only prefixed by "<Emphasis>"_"</Emphasis>" or "<Emphasis>"$"</Emphasis>"."
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
                        "This "<Emphasis>{element.to_str()}</Emphasis>" name might only be suffixed by "<Emphasis>"_"</Emphasis>" or "<Emphasis>"$"</Emphasis>"."
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
        let State { element, issue } = state;
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
            match issue {
                Invalid::Case(_) => {
                    let preferred_case = element.naming_convention(ctx.options())[0];
                    new_name = preferred_case.convert(main);
                    message = markup! { "Rename this symbol in "<Emphasis>{preferred_case.to_str()}</Emphasis>"." }.to_owned();
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
    pub(crate) AnyName = JsIdentifierBinding | JsLiteralMemberName | JsPrivateClassMemberName |
    JsLiteralExportName | TsIdentifierBinding | TsTypeParameterName
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
    issue: Invalid,
}

#[derive(Debug)]
enum Invalid {
    Case(Option<Case>),
    Prefix,
    Suffix,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamingConventionOptions {
    #[bpaf(hide)]
    #[serde(default = "default_strict_case", skip_serializing_if = "is_default_strict_case")]
    pub strict_case: bool,
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
            enum_member_case: Default::default(),
        }
    }
}

impl FromStr for NamingConventionOptions {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
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
            _ => return None,
        }
        Some(())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum EnumMemberCase {
    #[serde(rename = "PascalCase")]
    #[default]
    Pascal,
    #[serde(rename = "CONSTANT_CASE")]
    Constant,
    #[serde(rename = "camelCase")]
    Camel,
}

impl EnumMemberCase {
    pub const KNOWN_VALUES: &'static [&'static str] = &["camelCase", "CONSTANT_CASE", "PascalCase"];
}

impl FromStr for EnumMemberCase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camelCase" | "CamelCase" => Ok(Self::Camel),
            "CONSTANT_CASE" | "ConstantCase" => Ok(Self::Constant),
            "PascalCase" => Ok(Self::Pascal),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for EnumMemberCase"),
        }
    }
}

impl VisitNode<JsonLanguage> for EnumMemberCase {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, Self::KNOWN_VALUES, diagnostics)?;
        *self = match node.inner_string_text().ok()?.text() {
            "camelCase" => Self::Camel,
            "CONSTANT_CASE" => Self::Constant,
            "PascalCase" => Self::Pascal,
            _ => return None,
        };
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
                // ignore quoted members
                if member_name.value().ok()?.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                    return None;
                }
                match member_name.member()? {
                    AnyJsMember::AnyJsClassMember(member) => Named::from_class_member(&member),
                    AnyJsMember::AnyTsTypeMember(member) => Named::from_type_member(&member),
                    AnyJsMember::AnyJsObjectMember(member) => Named::from_object_member(&member),
                    AnyJsMember::JsObjectAssignmentPatternProperty(_)
                    | AnyJsMember::AnyJsObjectBindingPatternMember(_) => None,
                    AnyJsMember::TsEnumMember(_) => Some(Named::EnumMember),
                }
            }
            AnyName::JsPrivateClassMemberName(member_name) => {
                Named::from_class_member(&member_name.member()?)
            }
            AnyName::JsLiteralExportName(export_name) => {
                Some(match export_name.named_export()? {
                    AnyJsNamedExport::JsNamedImportSpecifier(_) => Named::ImportSource,
                    AnyJsNamedExport::JsExportNamedFromSpecifier(_) => Named::ExportSource,
                    AnyJsNamedExport::JsExportNamedSpecifier(_)
                    | AnyJsNamedExport::JsExportAsClause(_) => Named::ExportAlias,
                })
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
                Some(if getter.modifiers().has_static_modifier() {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                Some(if getter.modifiers().has_static_modifier() {
                    Named::ClassStaticGetter
                } else {
                    Named::ClassGetter
                })
            }
            AnyJsClassMember::JsMethodClassMember(method) => {
                Some(if method.modifiers().has_static_modifier() {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                Some(if method.modifiers().has_static_modifier() {
                    Named::ClassStaticMethod
                } else {
                    Named::ClassMethod
                })
            }
            AnyJsClassMember::JsPropertyClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsPropertySignatureClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Named::ClassStaticProperty
                } else {
                    Named::ClassProperty
                })
            }
            AnyJsClassMember::JsSetterClassMember(setter) => {
                Some(if setter.modifiers().has_static_modifier() {
                    Named::ClassStaticSetter
                } else {
                    Named::ClassSetter
                })
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                Some(if setter.modifiers().has_static_modifier() {
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
            AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => Some(Named::TypeAlias),
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
        let var_kind = var.declaration()?.variable_kind().ok()?;
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

    const fn to_str(self) -> &'static str {
        match self {
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
        }
    }

    /// Naming convention of `self`. The preferred convention comes first.
    fn naming_convention(self, options: &NamingConventionOptions) -> SmallVec<[Case; 3]> {
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
            | Named::ObjectMethod
            | Named::ObjectSetter
            | Named::ParameterProperty
            | Named::TypeMethod
            | Named::TypeProperty
            | Named::TypeSetter
            | Named::LocalConst
            | Named::LocalLet
            | Named::LocalVar
            | Named::TopLevelLet => SmallVec::from_slice(&[Case::Camel]),
            Named::Class | Named::Interface | Named::Enum => SmallVec::from_slice(&[Case::Pascal]),
            Named::ClassStaticGetter
            | Named::ClassStaticProperty
            | Named::ObjectGetter
            | Named::ObjectProperty
            | Named::TypeGetter
            | Named::TypeReadonlyProperty => SmallVec::from_slice(&[Case::Camel, Case::Constant]),
            Named::EnumMember => SmallVec::from_slice(&[options.enum_member_case.into()]),
            Named::ExportAlias | Named::ImportAlias | Named::TopLevelConst | Named::TopLevelVar => {
                SmallVec::from_slice(&[Case::Camel, Case::Pascal, Case::Constant])
            }
            Named::ExportSource | Named::ImportSource => SmallVec::new(),
            Named::Function | Named::Namespace => {
                SmallVec::from_slice(&[Case::Camel, Case::Pascal])
            }
            Named::TypeAlias => SmallVec::from_slice(&[Case::Pascal, Case::Camel]),
            Named::TypeParameter => SmallVec::from_slice(&[Case::NumberableCapital]),
        }
    }
}

#[cfg(test)]
mod tests {
    use rome_analyze::options::RuleOptions;
    use rome_analyze::{AnalyzerOptions, Never, RuleFilter, RuleKey};
    use rome_console::fmt::{Formatter, Termcolor};
    use rome_console::{markup, Markup};
    use rome_diagnostics::termcolor::NoColor;
    use rome_diagnostics::{Diagnostic, DiagnosticExt, PrintDiagnostic, Severity};
    use rome_js_parser::parse;
    use rome_js_syntax::{JsFileSource, TextRange};
    use std::slice;

    use super::*;
    use crate::{analyze, AnalysisFilter, ControlFlow};

    #[ignore]
    #[test]
    fn quick_test() {
        fn markup_to_string(markup: Markup) -> String {
            let mut buffer = Vec::new();
            let mut write = Termcolor(NoColor::new(&mut buffer));
            let mut fmt = Formatter::new(&mut write);
            fmt.write_markup(markup).unwrap();

            String::from_utf8(buffer).unwrap()
        }

        const SOURCE: &str = r#"
        	enum Status {
                OPEN,
                CLOSE,
            }
        "#;

        let parsed = parse(SOURCE, JsFileSource::ts());

        let mut error_ranges: Vec<TextRange> = Vec::new();
        let mut options = AnalyzerOptions::default();
        let rule_filter = RuleFilter::Rule("nursery", "useNamingConvention");
        options.configuration.rules.push_rule(
            RuleKey::new("nursery", "useNamingConvention"),
            RuleOptions::new(NamingConventionOptions {
                strict_case: true,
                enum_member_case: EnumMemberCase::Constant,
            }),
        );

        analyze(
            &parsed.tree(),
            AnalysisFilter {
                enabled_rules: Some(slice::from_ref(&rule_filter)),
                ..AnalysisFilter::default()
            },
            &options,
            JsFileSource::tsx(),
            |signal| {
                if let Some(diag) = signal.diagnostic() {
                    error_ranges.push(diag.location().span.unwrap());
                    let error = diag
                        .with_severity(Severity::Warning)
                        .with_file_path("ahahah")
                        .with_file_source_code(SOURCE);
                    let text = markup_to_string(markup! {
                        {PrintDiagnostic::verbose(&error)}
                    });
                    eprintln!("{text}");
                }

                for action in signal.actions() {
                    let new_code = action.mutation.commit();
                    eprintln!("{new_code}");
                }

                ControlFlow::<Never>::Continue(())
            },
        );
    }
}
