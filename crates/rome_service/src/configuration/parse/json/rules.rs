//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::Rules;
use rome_deserialize::json::{has_only_known_keys, VisitConfigurationAsJson};
use rome_deserialize::{DeserializationDiagnostic, VisitConfigurationNode};
use rome_json_syntax::JsonLanguage;
use rome_rowan::SyntaxNode;
impl VisitConfigurationAsJson for Rules {}
impl VisitConfigurationNode<JsonLanguage> for Rules {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "a11y",
                "complexity",
                "correctness",
                "nursery",
                "performance",
                "security",
                "style",
                "suspicious",
            ],
            diagnostics,
        )
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
            "a11y" => {
                let mut visitor = A11y::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.a11y = Some(visitor);
            }
            "complexity" => {
                let mut visitor = Complexity::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.complexity = Some(visitor);
            }
            "correctness" => {
                let mut visitor = Correctness::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.correctness = Some(visitor);
            }
            "nursery" => {
                let mut visitor = Nursery::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.nursery = Some(visitor);
            }
            "performance" => {
                let mut visitor = Performance::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.performance = Some(visitor);
            }
            "security" => {
                let mut visitor = Security::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.security = Some(visitor);
            }
            "style" => {
                let mut visitor = Style::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.style = Some(visitor);
            }
            "suspicious" => {
                let mut visitor = Suspicious::default();
                self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                self.suspicious = Some(visitor);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for A11y {}
impl VisitConfigurationNode<JsonLanguage> for A11y {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noAutofocus",
                "noBlankTarget",
                "noPositiveTabindex",
                "useAltText",
                "useAnchorContent",
                "useButtonType",
                "useHtmlLang",
                "useKeyWithClickEvents",
                "useKeyWithMouseEvents",
                "useValidAnchor",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAutofocus" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_autofocus = Some(configuration);
            }
            "noBlankTarget" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_blank_target = Some(configuration);
            }
            "noPositiveTabindex" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_positive_tabindex = Some(configuration);
            }
            "useAltText" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_alt_text = Some(configuration);
            }
            "useAnchorContent" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_anchor_content = Some(configuration);
            }
            "useButtonType" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_button_type = Some(configuration);
            }
            "useHtmlLang" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_html_lang = Some(configuration);
            }
            "useKeyWithClickEvents" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_key_with_click_events = Some(configuration);
            }
            "useKeyWithMouseEvents" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_key_with_mouse_events = Some(configuration);
            }
            "useValidAnchor" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_valid_anchor = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Complexity {}
impl VisitConfigurationNode<JsonLanguage> for Complexity {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noExtraBooleanCast",
                "noMultipleSpacesInRegularExpressionLiterals",
                "noUselessFragments",
                "useFlatMap",
                "useOptionalChain",
                "useSimplifiedLogicExpression",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noExtraBooleanCast" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_extra_boolean_cast = Some(configuration);
            }
            "noMultipleSpacesInRegularExpressionLiterals" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_multiple_spaces_in_regular_expression_literals = Some(configuration);
            }
            "noUselessFragments" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_useless_fragments = Some(configuration);
            }
            "useFlatMap" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_flat_map = Some(configuration);
            }
            "useOptionalChain" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_optional_chain = Some(configuration);
            }
            "useSimplifiedLogicExpression" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_simplified_logic_expression = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Correctness {}
impl VisitConfigurationNode<JsonLanguage> for Correctness {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noChildrenProp",
                "noConstAssign",
                "noEmptyPattern",
                "noNewSymbol",
                "noRenderReturnValue",
                "noUndeclaredVariables",
                "noUnnecessaryContinue",
                "noUnreachable",
                "noUnusedVariables",
                "noVoidElementsWithChildren",
                "useValidForDirection",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noChildrenProp" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_children_prop = Some(configuration);
            }
            "noConstAssign" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_const_assign = Some(configuration);
            }
            "noEmptyPattern" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_empty_pattern = Some(configuration);
            }
            "noNewSymbol" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_new_symbol = Some(configuration);
            }
            "noRenderReturnValue" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_render_return_value = Some(configuration);
            }
            "noUndeclaredVariables" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_undeclared_variables = Some(configuration);
            }
            "noUnnecessaryContinue" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unnecessary_continue = Some(configuration);
            }
            "noUnreachable" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unreachable = Some(configuration);
            }
            "noUnusedVariables" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unused_variables = Some(configuration);
            }
            "noVoidElementsWithChildren" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_void_elements_with_children = Some(configuration);
            }
            "useValidForDirection" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_valid_for_direction = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Nursery {}
impl VisitConfigurationNode<JsonLanguage> for Nursery {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noAccessKey",
                "noAssignInExpressions",
                "noBannedTypes",
                "noClassAssign",
                "noCommaOperator",
                "noConstEnum",
                "noConstructorReturn",
                "noDistractingElements",
                "noDuplicateCase",
                "noDuplicateJsxProps",
                "noDuplicateObjectKeys",
                "noEmptyInterface",
                "noExtraNonNullAssertion",
                "noExtraSemicolons",
                "noGlobalObjectCalls",
                "noHeaderScope",
                "noInnerDeclarations",
                "noInvalidConstructorSuper",
                "noNonNullAssertion",
                "noNoninteractiveElementToInteractiveRole",
                "noPrecisionLoss",
                "noPrototypeBuiltins",
                "noRedundantAlt",
                "noRedundantUseStrict",
                "noRestrictedGlobals",
                "noSelfCompare",
                "noSetterReturn",
                "noStringCaseMismatch",
                "noUnreachableSuper",
                "noUnsafeFinally",
                "noUnusedLabels",
                "noUselessSwitchCase",
                "noVar",
                "noVoidTypeReturn",
                "noWith",
                "useAriaPropTypes",
                "useAriaPropsForRole",
                "useCamelCase",
                "useConst",
                "useDefaultParameterLast",
                "useDefaultSwitchClauseLast",
                "useEnumInitializers",
                "useExhaustiveDependencies",
                "useExponentiationOperator",
                "useHookAtTopLevel",
                "useIframeTitle",
                "useIsNan",
                "useMediaCaption",
                "useNumericLiterals",
                "useValidAriaProps",
                "useValidLang",
                "useYield",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAccessKey" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_access_key = Some(configuration);
            }
            "noAssignInExpressions" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_assign_in_expressions = Some(configuration);
            }
            "noBannedTypes" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_banned_types = Some(configuration);
            }
            "noClassAssign" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_class_assign = Some(configuration);
            }
            "noCommaOperator" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_comma_operator = Some(configuration);
            }
            "noConstEnum" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_const_enum = Some(configuration);
            }
            "noConstructorReturn" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_constructor_return = Some(configuration);
            }
            "noDistractingElements" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_distracting_elements = Some(configuration);
            }
            "noDuplicateCase" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_duplicate_case = Some(configuration);
            }
            "noDuplicateJsxProps" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_duplicate_jsx_props = Some(configuration);
            }
            "noDuplicateObjectKeys" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_duplicate_object_keys = Some(configuration);
            }
            "noEmptyInterface" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_empty_interface = Some(configuration);
            }
            "noExtraNonNullAssertion" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_extra_non_null_assertion = Some(configuration);
            }
            "noExtraSemicolons" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_extra_semicolons = Some(configuration);
            }
            "noGlobalObjectCalls" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_global_object_calls = Some(configuration);
            }
            "noHeaderScope" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_header_scope = Some(configuration);
            }
            "noInnerDeclarations" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_inner_declarations = Some(configuration);
            }
            "noInvalidConstructorSuper" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_invalid_constructor_super = Some(configuration);
            }
            "noNonNullAssertion" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_non_null_assertion = Some(configuration);
            }
            "noNoninteractiveElementToInteractiveRole" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_noninteractive_element_to_interactive_role = Some(configuration);
            }
            "noPrecisionLoss" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_precision_loss = Some(configuration);
            }
            "noPrototypeBuiltins" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_prototype_builtins = Some(configuration);
            }
            "noRedundantAlt" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_redundant_alt = Some(configuration);
            }
            "noRedundantUseStrict" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_redundant_use_strict = Some(configuration);
            }
            "noRestrictedGlobals" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_restricted_globals = Some(configuration);
            }
            "noSelfCompare" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_self_compare = Some(configuration);
            }
            "noSetterReturn" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_setter_return = Some(configuration);
            }
            "noStringCaseMismatch" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_string_case_mismatch = Some(configuration);
            }
            "noUnreachableSuper" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unreachable_super = Some(configuration);
            }
            "noUnsafeFinally" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unsafe_finally = Some(configuration);
            }
            "noUnusedLabels" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unused_labels = Some(configuration);
            }
            "noUselessSwitchCase" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_useless_switch_case = Some(configuration);
            }
            "noVar" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_var = Some(configuration);
            }
            "noVoidTypeReturn" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_void_type_return = Some(configuration);
            }
            "noWith" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_with = Some(configuration);
            }
            "useAriaPropTypes" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_aria_prop_types = Some(configuration);
            }
            "useAriaPropsForRole" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_aria_props_for_role = Some(configuration);
            }
            "useCamelCase" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_camel_case = Some(configuration);
            }
            "useConst" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_const = Some(configuration);
            }
            "useDefaultParameterLast" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_default_parameter_last = Some(configuration);
            }
            "useDefaultSwitchClauseLast" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_default_switch_clause_last = Some(configuration);
            }
            "useEnumInitializers" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_enum_initializers = Some(configuration);
            }
            "useExhaustiveDependencies" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_exhaustive_dependencies = Some(configuration);
            }
            "useExponentiationOperator" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_exponentiation_operator = Some(configuration);
            }
            "useHookAtTopLevel" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_hook_at_top_level = Some(configuration);
            }
            "useIframeTitle" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_iframe_title = Some(configuration);
            }
            "useIsNan" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_is_nan = Some(configuration);
            }
            "useMediaCaption" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_media_caption = Some(configuration);
            }
            "useNumericLiterals" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_numeric_literals = Some(configuration);
            }
            "useValidAriaProps" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_valid_aria_props = Some(configuration);
            }
            "useValidLang" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_valid_lang = Some(configuration);
            }
            "useYield" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_yield = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Performance {}
impl VisitConfigurationNode<JsonLanguage> for Performance {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["recommended", "noDelete"], diagnostics)
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDelete" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_delete = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Security {}
impl VisitConfigurationNode<JsonLanguage> for Security {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noDangerouslySetInnerHtml",
                "noDangerouslySetInnerHtmlWithChildren",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDangerouslySetInnerHtml" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_dangerously_set_inner_html = Some(configuration);
            }
            "noDangerouslySetInnerHtmlWithChildren" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_dangerously_set_inner_html_with_children = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Style {}
impl VisitConfigurationNode<JsonLanguage> for Style {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noArguments",
                "noImplicitBoolean",
                "noNegationElse",
                "noShoutyConstants",
                "noUnusedTemplateLiteral",
                "useBlockStatements",
                "useFragmentSyntax",
                "useSelfClosingElements",
                "useShorthandArrayType",
                "useSingleCaseStatement",
                "useSingleVarDeclarator",
                "useTemplate",
                "useWhile",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArguments" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_arguments = Some(configuration);
            }
            "noImplicitBoolean" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_implicit_boolean = Some(configuration);
            }
            "noNegationElse" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_negation_else = Some(configuration);
            }
            "noShoutyConstants" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_shouty_constants = Some(configuration);
            }
            "noUnusedTemplateLiteral" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unused_template_literal = Some(configuration);
            }
            "useBlockStatements" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_block_statements = Some(configuration);
            }
            "useFragmentSyntax" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_fragment_syntax = Some(configuration);
            }
            "useSelfClosingElements" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_self_closing_elements = Some(configuration);
            }
            "useShorthandArrayType" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_shorthand_array_type = Some(configuration);
            }
            "useSingleCaseStatement" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_single_case_statement = Some(configuration);
            }
            "useSingleVarDeclarator" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_single_var_declarator = Some(configuration);
            }
            "useTemplate" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_template = Some(configuration);
            }
            "useWhile" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_while = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitConfigurationAsJson for Suspicious {}
impl VisitConfigurationNode<JsonLanguage> for Suspicious {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "noArrayIndexKey",
                "noAsyncPromiseExecutor",
                "noCatchAssign",
                "noCommentText",
                "noCompareNegZero",
                "noDebugger",
                "noDoubleEquals",
                "noDuplicateParameters",
                "noExplicitAny",
                "noFunctionAssign",
                "noImportAssign",
                "noLabelVar",
                "noShadowRestrictedNames",
                "noSparseArray",
                "noUnsafeNegation",
                "useValidTypeof",
            ],
            diagnostics,
        )
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArrayIndexKey" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_array_index_key = Some(configuration);
            }
            "noAsyncPromiseExecutor" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_async_promise_executor = Some(configuration);
            }
            "noCatchAssign" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_catch_assign = Some(configuration);
            }
            "noCommentText" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_comment_text = Some(configuration);
            }
            "noCompareNegZero" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_compare_neg_zero = Some(configuration);
            }
            "noDebugger" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_debugger = Some(configuration);
            }
            "noDoubleEquals" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_double_equals = Some(configuration);
            }
            "noDuplicateParameters" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_duplicate_parameters = Some(configuration);
            }
            "noExplicitAny" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_explicit_any = Some(configuration);
            }
            "noFunctionAssign" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_function_assign = Some(configuration);
            }
            "noImportAssign" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_import_assign = Some(configuration);
            }
            "noLabelVar" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_label_var = Some(configuration);
            }
            "noShadowRestrictedNames" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_shadow_restricted_names = Some(configuration);
            }
            "noSparseArray" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_sparse_array = Some(configuration);
            }
            "noUnsafeNegation" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.no_unsafe_negation = Some(configuration);
            }
            "useValidTypeof" => {
                let mut configuration = RuleConfiguration::default();
                self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                self.use_valid_typeof = Some(configuration);
            }
            _ => {}
        }
        Some(())
    }
}
