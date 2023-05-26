use crate::{control_flow::AnyJsControlFlowRoot, semantic_services::Semantic};
use bitflags::bitflags;
use rome_analyze::{
    context::RuleContext, declare_rule, DeserializableRuleOptions, Rule, RuleDiagnostic,
};
use rome_console::markup;
use rome_deserialize::{
    json::{
        deserialize_from_json_str, has_only_known_keys, with_only_known_variants, JsonDeserialize,
        VisitJsonNode,
    },
    DeserializationDiagnostic, Deserialized, VisitNode,
};
use rome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, member_name_ext::AnyJsMember, AnyJsClassMember,
    AnyJsObjectMember, AnyTsTypeMember, JsIdentifierBinding, JsLiteralExportName,
    JsLiteralMemberName, JsPrivateClassMemberName, TsTypeParameterName,
};
use rome_json_syntax::JsonLanguage;
use rome_rowan::{declare_node_union, AstNode, SyntaxNode};
use smallvec::SmallVec;

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
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate)  UseNamingConvention {
        version: "next",
        name: "useNamingConvention",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyName = JsIdentifierBinding | JsLiteralMemberName | JsPrivateClassMemberName | JsLiteralExportName | TsTypeParameterName
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Named {
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
    Function,
    FunctionParameter,
    ImportAlias,
    ImportNamespace,
    IndexParameter,
    Interface,
    Namespace,
    ObjectGetter,
    ObjectMethod,
    ObjectProperty,
    ObjectSetter,
    TypeAlias,
    TypeGetter,
    TypeMethod,
    TypeProperty,
    TypeReadonlyProperty,
    TypeSetter,
    TypeParameter,
    ModuleLevelVariable,
    Variable,
}

impl Named {
    fn from_any_js_name(js_name: &AnyName) -> Option<Self> {
        match js_name {
            AnyName::JsIdentifierBinding(binding) => {
                let Some(decl) = binding.declaration() else {
                    return None;
                };
                match decl {
                    AnyJsBindingDeclaration::JsVariableDeclarator(var) => {
                        let is_module_level = matches!(
                            var.syntax()
                                .ancestors()
                                .find_map(AnyJsControlFlowRoot::cast),
                            Some(AnyJsControlFlowRoot::JsModule(_))
                                | Some(AnyJsControlFlowRoot::JsScript(_))
                        );
                        Some(if is_module_level {
                            Self::ModuleLevelVariable
                        } else {
                            Self::Variable
                        })
                    }
                    AnyJsBindingDeclaration::JsCatchDeclaration(_) => Some(Self::CatchParameter),
                    AnyJsBindingDeclaration::JsFormalParameter(_)
                    | AnyJsBindingDeclaration::JsRestParameter(_)
                    | AnyJsBindingDeclaration::TsPropertyParameter(_) => {
                        Some(Self::FunctionParameter)
                    }
                    AnyJsBindingDeclaration::TsIndexSignatureParameter(_) => {
                        Some(Self::IndexParameter)
                    }
                    AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
                    | AnyJsBindingDeclaration::JsImportNamespaceClause(_) => {
                        Some(Self::ImportNamespace)
                    }
                    AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                    | AnyJsBindingDeclaration::JsFunctionExpression(_)
                    | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
                    | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
                    | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_) => {
                        Some(Self::Function)
                    }
                    AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => Some(Self::ImportAlias),
                    AnyJsBindingDeclaration::TsModuleDeclaration(_) => Some(Self::Namespace),
                    AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => Some(Self::TypeAlias),
                    AnyJsBindingDeclaration::JsClassDeclaration(_)
                    | AnyJsBindingDeclaration::JsClassExpression(_)
                    | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => {
                        Some(Self::Class)
                    }
                    AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => Some(Self::Interface),
                    AnyJsBindingDeclaration::TsEnumDeclaration(_) => Some(Self::Enum),
                    AnyJsBindingDeclaration::JsBogusParameter(_)
                    | AnyJsBindingDeclaration::JsImportDefaultClause(_)
                    | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
                    | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
                    | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
                    | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => None,
                }
            }
            AnyName::JsLiteralMemberName(member_name) => {
                if let Some(member) = member_name.member() {
                    return match member {
                        AnyJsMember::AnyJsClassMember(member) => Self::from_class_member(&member),
                        AnyJsMember::AnyTsTypeMember(member) => Self::from_type_member(&member),
                        AnyJsMember::AnyJsObjectMember(member) => Self::from_object_member(&member),
                        AnyJsMember::JsObjectAssignmentPatternProperty(_)
                        | AnyJsMember::AnyJsObjectBindingPatternMember(_) => None,
                        AnyJsMember::TsEnumMember(_) => Some(Self::EnumMember),
                    };
                }
                None
            }
            AnyName::JsPrivateClassMemberName(member_name) => {
                if let Some(member) = member_name.member() {
                    return Self::from_class_member(&member);
                }
                None
            }
            AnyName::JsLiteralExportName(_) => Some(Self::ExportAlias),
            AnyName::TsTypeParameterName(_) => Some(Self::TypeParameter),
        }
    }

    fn from_class_member(member: &AnyJsClassMember) -> Option<Self> {
        match member {
            AnyJsClassMember::JsBogusMember(_)
            | AnyJsClassMember::JsConstructorClassMember(_)
            | AnyJsClassMember::TsConstructorSignatureClassMember(_)
            | AnyJsClassMember::JsEmptyClassMember(_)
            | AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => None,
            AnyJsClassMember::TsIndexSignatureClassMember(_) => Some(Self::IndexParameter),
            AnyJsClassMember::JsGetterClassMember(getter) => {
                Some(if getter.modifiers().has_static_modifier() {
                    Self::ClassStaticGetter
                } else {
                    Self::ClassGetter
                })
            }
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => {
                Some(if getter.modifiers().has_static_modifier() {
                    Self::ClassStaticGetter
                } else {
                    Self::ClassGetter
                })
            }
            AnyJsClassMember::JsMethodClassMember(method) => {
                Some(if method.modifiers().has_static_modifier() {
                    Self::ClassStaticMethod
                } else {
                    Self::ClassMethod
                })
            }
            AnyJsClassMember::TsMethodSignatureClassMember(method) => {
                Some(if method.modifiers().has_static_modifier() {
                    Self::ClassStaticMethod
                } else {
                    Self::ClassMethod
                })
            }
            AnyJsClassMember::JsPropertyClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Self::ClassStaticProperty
                } else {
                    Self::ClassProperty
                })
            }
            AnyJsClassMember::TsPropertySignatureClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Self::ClassStaticProperty
                } else {
                    Self::ClassProperty
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                Some(if property.modifiers().has_static_modifier() {
                    Self::ClassStaticProperty
                } else {
                    Self::ClassProperty
                })
            }
            AnyJsClassMember::JsSetterClassMember(setter) => {
                Some(if setter.modifiers().has_static_modifier() {
                    Self::ClassStaticSetter
                } else {
                    Self::ClassSetter
                })
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => {
                Some(if setter.modifiers().has_static_modifier() {
                    Self::ClassStaticSetter
                } else {
                    Self::ClassSetter
                })
            }
        }
    }

    fn from_object_member(member: &AnyJsObjectMember) -> Option<Self> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) | AnyJsObjectMember::JsSpread(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => Some(Self::ObjectGetter),
            AnyJsObjectMember::JsMethodObjectMember(_) => Some(Self::ObjectMethod),
            AnyJsObjectMember::JsPropertyObjectMember(_)
            | AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => Some(Self::ObjectProperty),
            AnyJsObjectMember::JsSetterObjectMember(_) => Some(Self::ObjectSetter),
        }
    }

    fn from_type_member(member: &AnyTsTypeMember) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_)
            | AnyTsTypeMember::TsCallSignatureTypeMember(_)
            | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => None,
            AnyTsTypeMember::TsIndexSignatureTypeMember(_) => Some(Self::IndexParameter),
            AnyTsTypeMember::TsGetterSignatureTypeMember(_) => Some(Self::TypeGetter),
            AnyTsTypeMember::TsMethodSignatureTypeMember(_) => Some(Self::TypeMethod),
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                Some(if property.readonly_token().is_some() {
                    Self::TypeReadonlyProperty
                } else {
                    Self::TypeProperty
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_) => Some(Self::TypeSetter),
        }
    }

    const fn to_str(self) -> &'static str {
        match self {
            Self::CatchParameter => "catch parameter",
            Self::Class => "class",
            Self::ClassGetter => "class getter",
            Self::ClassMethod => "class method",
            Self::ClassProperty => "class property",
            Self::ClassSetter => "class setter",
            Self::ClassStaticGetter => "static getter",
            Self::ClassStaticMethod => "static method",
            Self::ClassStaticProperty => "static property",
            Self::ClassStaticSetter => "static setter",
            Self::Enum => "enum",
            Self::EnumMember => "enum member",
            Self::ExportAlias => "export alias",
            Self::Function => "function",
            Self::FunctionParameter => "function parameter",
            Self::ImportAlias => "import alias",
            Self::ImportNamespace => "import namespace",
            Self::IndexParameter => "index parameter",
            Self::Interface => "interface",
            Self::Namespace => "namespace",
            Self::ObjectGetter => "object getter",
            Self::ObjectMethod => "object method",
            Self::ObjectProperty => "object property",
            Self::ObjectSetter => "object setter",
            Self::TypeAlias => "type alias",
            Self::TypeGetter => "getter",
            Self::TypeMethod => "method",
            Self::TypeProperty => "property",
            Self::TypeReadonlyProperty => "readonly property",
            Self::TypeSetter => "setter",
            Self::TypeParameter => "type parameter",
            Self::ModuleLevelVariable => "top-level variable",
            Self::Variable => "variable",
        }
    }

    fn naming_style(self, options: &Options) -> NamingStyle {
        let result = match self {
            Self::CatchParameter
            | Self::ClassGetter
            | Self::ClassMethod
            | Self::ClassProperty
            | Self::ClassSetter
            | Self::ClassStaticMethod
            | Self::ClassStaticSetter
            | Self::FunctionParameter
            | Self::ImportNamespace
            | Self::IndexParameter
            | Self::ObjectMethod
            | Self::ObjectSetter
            | Self::TypeMethod
            | Self::TypeProperty
            | Self::TypeSetter
            | Self::Variable => NamingStyle::LOWER_CAMEL_CASE,
            Self::Class | Self::Interface | Self::Enum | Self::TypeParameter => {
                NamingStyle::UPPER_CAMEL_CASE
            }
            Self::ClassStaticGetter
            | Self::ClassStaticProperty
            | Self::ModuleLevelVariable
            | Self::ObjectGetter
            | Self::ObjectProperty
            | Self::TypeGetter
            | Self::TypeReadonlyProperty => {
                NamingStyle::LOWER_CAMEL_CASE | NamingStyle::UPPER_UNDERSCORE_CASE
            }
            Self::EnumMember => options.enum_member_case.0,
            Self::ImportAlias | Self::ExportAlias => {
                NamingStyle::CAMEL_CASE | NamingStyle::UPPER_UNDERSCORE_CASE
            }
            Self::Function | Self::Namespace | Self::TypeAlias => NamingStyle::CAMEL_CASE,
        };
        if options.strict_camel_case {
            result
        } else {
            result.non_strict()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) struct EnumMemberNamingStyleOption(NamingStyle);

impl EnumMemberNamingStyleOption {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] =
        &["UpperCamelCase", "UPPER_UNDERSCORE_CASE"];
}

impl Default for EnumMemberNamingStyleOption {
    fn default() -> Self {
        Self(NamingStyle::UPPER_CAMEL_CASE)
    }
}

impl VisitNode<JsonLanguage> for EnumMemberNamingStyleOption {
    fn visit_member_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = with_only_known_variants(node, Self::KNOWN_VALUES, diagnostics)?;
        match node.inner_string_text().ok()?.text() {
            "UpperCamelCase" => {
                *self = Self(NamingStyle::UPPER_CAMEL_CASE);
            }
            "UPPER_UNDERSCORE_CASE" => {
                *self = Self(NamingStyle::UPPER_UNDERSCORE_CASE);
            }
            _ => {}
        }
        Some(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) struct Options {
    strict_camel_case: bool,
    enum_member_case: EnumMemberNamingStyleOption,
}

impl Options {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["strictCamelCase", "enumMemberCase"];
}

impl Default for Options {
    fn default() -> Self {
        Self {
            strict_camel_case: true,
            enum_member_case: Default::default(),
        }
    }
}

impl VisitJsonNode for Options {}
impl VisitNode<JsonLanguage> for Options {
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
        if name_text == "strictCamelCase" {
            value.as_json_boolean_value()?;
        }
        Some(())
    }
}

impl JsonDeserialize for Options {
    fn deserialize_from_ast(
        root: rome_json_syntax::JsonRoot,
        visitor: &mut impl VisitJsonNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let object = root.value().ok()?;
        let object = object.as_json_object_value()?;
        for element in object.json_member_list() {
            let element = element.ok()?;
            visitor.visit_map(
                element.name().ok()?.syntax(),
                element.value().ok()?.syntax(),
                diagnostics,
            )?;
        }
        Some(())
    }
}

impl DeserializableRuleOptions for Options {
    fn from(value: String) -> Deserialized<Self> {
        deserialize_from_json_str(&value)
    }
}

#[derive(Debug)]
pub(crate) struct State {
    actual_style: NamingStyle,
    element: Named,
}

impl Rule for UseNamingConvention {
    type Query = Semantic<AnyName>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = Options;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query();
        let element = Named::from_any_js_name(name)?;
        let expected_style = element.naming_style(ctx.options());
        let actual_style = match name {
            AnyName::JsIdentifierBinding(binding) => identify_naming_style(
                trim_leading_trailing_underscore_dollar(binding.name_token().ok()?.text_trimmed()),
            ),

            AnyName::JsLiteralMemberName(member_name) => identify_naming_style(
                trim_leading_trailing_underscore_dollar(&member_name.name().ok()?),
            ),

            AnyName::JsPrivateClassMemberName(member_name) => {
                identify_naming_style(trim_leading_trailing_underscore_dollar(
                    member_name.id_token().ok()?.text_trimmed(),
                ))
            }
            AnyName::JsLiteralExportName(export_name) => identify_naming_style(
                trim_leading_trailing_underscore_dollar(export_name.value().ok()?.text_trimmed()),
            ),
            AnyName::TsTypeParameterName(type_parameter) => {
                identify_naming_style(trim_leading_trailing_underscore_dollar(
                    type_parameter.ident_token().ok()?.text_trimmed(),
                ))
            }
        };
        if actual_style.intersects(expected_style) {
            return None;
        }
        Some(State {
            actual_style,
            element,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State {
            actual_style,
            element,
        } = state;
        let expected_style = element.naming_style(ctx.options());
        let name = ctx.query();
        let actual_style_name = actual_style.style_names().join(" and ");
        let expected_style_names = expected_style.style_names().join(" or ");
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            name.syntax().text_trimmed_range(),
            markup! {
                "This "{element.to_str()}" name should be in "{expected_style_names}"."
            },
        );
        if !actual_style_name.is_empty() {
            diagnostic = diagnostic.note(markup! {
                "The name is currently in "{actual_style_name}"."
            })
        }
        Some(diagnostic)
    }
}

bitflags! {
    pub(crate) struct NamingStyle: u8 {
        const LOWER_CAMEL_CASE = 1;
        const UPPER_CAMEL_CASE = 1 << 1;
        const NON_STRICT = 1 << 2;
        const LOWER_UNDERSCORE_CASE = 1 << 3;
        const UPPER_UNDERSCORE_CASE = 1 << 4;
        const LOWER_DASH_CASE = 1 << 5;
        const UPPER_DASH_CASE = 1 << 6;

        const CAMEL_CASE = Self::LOWER_CAMEL_CASE.bits() | Self::UPPER_CAMEL_CASE.bits();
        const UNDERSCORE_CASE = Self::LOWER_UNDERSCORE_CASE.bits() | Self::UPPER_UNDERSCORE_CASE.bits();
        const DASH_CASE = Self::LOWER_DASH_CASE.bits() | Self::UPPER_DASH_CASE.bits();
        const NON_STRICT_LOWER_CAMEL_CASE = Self::NON_STRICT.bits() | Self::LOWER_CAMEL_CASE.bits();
        const NON_STRICT_UPPER_CAMEL_CASE = Self::NON_STRICT.bits() | Self::UPPER_CAMEL_CASE.bits();
        const NON_STRICT_CAMEL_CASE = Self::NON_STRICT_LOWER_CAMEL_CASE.bits() | Self::NON_STRICT_UPPER_CAMEL_CASE.bits();
        const LOWER_CASE = Self::LOWER_CAMEL_CASE.bits() | Self::LOWER_UNDERSCORE_CASE.bits() | Self::LOWER_DASH_CASE.bits();
        // UPPER_CASE name is not always a UPPER_CAMEL_CASE name. e.g. "TEST" is in uppercase, but is not in strict camel case.
        const UPPER_CASE = Self::NON_STRICT_UPPER_CAMEL_CASE.bits() | Self::UPPER_UNDERSCORE_CASE.bits() | Self::UPPER_DASH_CASE.bits();
    }
}

impl NamingStyle {
    fn non_strict(self) -> Self {
        if self.intersects(NamingStyle::CAMEL_CASE) {
            return self | NamingStyle::NON_STRICT;
        }
        self
    }

    fn style_names(self) -> SmallVec<[&'static str; 6]> {
        let mut result = SmallVec::new();
        if self.contains(Self::LOWER_CAMEL_CASE) {
            if self.contains(Self::NON_STRICT) {
                result.push("lowerCamelCase");
            } else {
                result.push("strictLowerCamelCase");
            }
        }
        if self.contains(Self::UPPER_CAMEL_CASE) {
            if self.contains(Self::NON_STRICT) {
                result.push("UpperCamelCase");
            } else {
                result.push("StrictUpperCamelCase");
            }
        }
        if self.contains(Self::LOWER_UNDERSCORE_CASE) {
            result.push("lower_underscore_case")
        }
        if self.contains(Self::UPPER_UNDERSCORE_CASE) {
            result.push("UPPER_UNDERSCORE_CASE")
        }
        if self.contains(Self::LOWER_DASH_CASE) {
            result.push("lower-dash-case")
        }
        if self.contains(Self::UPPER_DASH_CASE) {
            result.push("UPPER-DASH-CASE")
        }
        result
    }
}

fn trim_leading_trailing_underscore_dollar(s: &str) -> &str {
    let mut start = 0;
    let mut end = s.len();
    if start < end && (&s[..1] == "_" || &s[..1] == "$") {
        start = 1
    }
    if start < end && (&s[end - 1..] == "_" || &s[end - 1..] == "$") {
        end -= 1
    }
    &s[start..end]
}

pub(crate) fn identify_naming_style(s: &str) -> NamingStyle {
    // An empty string respects all styles
    let mut result = NamingStyle::all();
    let mut previous_is_uppercase = false;
    for (i, c) in s.char_indices() {
        if !c.is_alphanumeric() && c != '_' && c != '-' {
            return NamingStyle::empty();
        }
        if i == 0 {
            if !c.is_alphanumeric() {
                return NamingStyle::empty();
            }
            if c.is_uppercase() {
                result = NamingStyle::UPPER_CASE - NamingStyle::NON_STRICT;
                previous_is_uppercase = true;
            } else {
                result = NamingStyle::LOWER_CASE;
            }
            continue;
        }
        if c == '_' {
            result &= NamingStyle::UNDERSCORE_CASE;
        } else if c == '-' {
            result &= NamingStyle::DASH_CASE;
        } else if c.is_uppercase() {
            if previous_is_uppercase {
                result = result.non_strict();
            } else {
                result &= NamingStyle::NON_STRICT_CAMEL_CASE;
                previous_is_uppercase = true;
            }
        } else if previous_is_uppercase {
            result &= NamingStyle::NON_STRICT_CAMEL_CASE;
            previous_is_uppercase = false;
        } else {
            result &= NamingStyle::NON_STRICT_CAMEL_CASE | NamingStyle::LOWER_CASE;
        }
        if result.is_empty() {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_naming_style() {
        assert!(identify_naming_style("UpperStrictCamelCase") == NamingStyle::UPPER_CAMEL_CASE);
        assert!(identify_naming_style("lowerStrictCamelCase") == NamingStyle::LOWER_CAMEL_CASE);
        assert!(
            identify_naming_style("UPPER_UNDERSCORE_CASE") == NamingStyle::UPPER_UNDERSCORE_CASE
        );
        assert!(
            identify_naming_style("lower_underscore_case") == NamingStyle::LOWER_UNDERSCORE_CASE
        );
        assert!(identify_naming_style("UPPER-DASH-CASE") == NamingStyle::UPPER_DASH_CASE);
        assert!(identify_naming_style("lower-dash-case") == NamingStyle::LOWER_DASH_CASE);
        assert!(identify_naming_style("UPPER") == NamingStyle::UPPER_CASE);
        assert!(identify_naming_style("lower") == NamingStyle::LOWER_CASE);
        assert!(identify_naming_style("UpperCC") == NamingStyle::NON_STRICT_UPPER_CAMEL_CASE);
        assert!(identify_naming_style("lowerCC") == NamingStyle::NON_STRICT_LOWER_CAMEL_CASE);

        assert!(identify_naming_style("") == NamingStyle::all());
        assert!(identify_naming_style("Capital-Words") == NamingStyle::empty());
        assert!(identify_naming_style("Capital_Words") == NamingStyle::empty());
    }
}
