//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::configuration::linter::*;
use crate::configuration::parse::json::linter::are_recommended_and_all_correct;
use crate::Rules;
use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use rome_deserialize::{DeserializationDiagnostic, VisitNode};
use rome_json_syntax::{AnyJsonValue, JsonLanguage};
use rome_rowan::{AstNode, SyntaxNode};
impl VisitJsonNode for Rules {}
impl VisitNode<JsonLanguage> for Rules {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
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
            "recommended" => {
                self.recommended = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "a11y" => {
                let mut visitor = A11y::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.a11y = Some(visitor);
                }
            }
            "complexity" => {
                let mut visitor = Complexity::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.complexity = Some(visitor);
                }
            }
            "correctness" => {
                let mut visitor = Correctness::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.correctness = Some(visitor);
                }
            }
            "nursery" => {
                let mut visitor = Nursery::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.nursery = Some(visitor);
                }
            }
            "performance" => {
                let mut visitor = Performance::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.performance = Some(visitor);
                }
            }
            "security" => {
                let mut visitor = Security::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.security = Some(visitor);
                }
            }
            "style" => {
                let mut visitor = Style::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.style = Some(visitor);
                }
            }
            "suspicious" => {
                let mut visitor = Suspicious::default();
                if are_recommended_and_all_correct(&value, name_text, diagnostics)? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.suspicious = Some(visitor);
                }
            }
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for A11y {}
impl VisitNode<JsonLanguage> for A11y {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noAccessKey",
                "noAutofocus",
                "noBlankTarget",
                "noDistractingElements",
                "noHeaderScope",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAccessKey" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_access_key = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_access_key = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAutofocus" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_autofocus = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_autofocus = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noBlankTarget" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_blank_target = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_blank_target = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDistractingElements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_distracting_elements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_distracting_elements = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noHeaderScope" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_header_scope = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_header_scope = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPositiveTabindex" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_positive_tabindex = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_positive_tabindex = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAltText" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_alt_text = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_alt_text = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAnchorContent" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_anchor_content = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_anchor_content = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useButtonType" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_button_type = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_button_type = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useHtmlLang" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_html_lang = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_html_lang = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useKeyWithClickEvents" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_click_events = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_click_events = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useKeyWithMouseEvents" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_mouse_events = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_key_with_mouse_events = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidAnchor" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_anchor = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_anchor = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Complexity {}
impl VisitNode<JsonLanguage> for Complexity {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noExtraBooleanCast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_boolean_cast = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_boolean_cast = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noMultipleSpacesInRegularExpressionLiterals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_multiple_spaces_in_regular_expression_literals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_multiple_spaces_in_regular_expression_literals = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessFragments" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_fragments = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_fragments = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useFlatMap" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_flat_map = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_flat_map = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useOptionalChain" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_optional_chain = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_optional_chain = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSimplifiedLogicExpression" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_simplified_logic_expression = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_simplified_logic_expression = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Correctness {}
impl VisitNode<JsonLanguage> for Correctness {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noChildrenProp",
                "noConstAssign",
                "noConstructorReturn",
                "noEmptyPattern",
                "noNewSymbol",
                "noPrecisionLoss",
                "noRenderReturnValue",
                "noSetterReturn",
                "noStringCaseMismatch",
                "noUndeclaredVariables",
                "noUnnecessaryContinue",
                "noUnreachable",
                "noUnsafeFinally",
                "noUnusedVariables",
                "noVoidElementsWithChildren",
                "noVoidTypeReturn",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noChildrenProp" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_children_prop = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_children_prop = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstructorReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_constructor_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_constructor_return = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noEmptyPattern" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_pattern = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_pattern = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNewSymbol" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_new_symbol = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_new_symbol = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPrecisionLoss" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_precision_loss = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_precision_loss = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRenderReturnValue" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_render_return_value = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_render_return_value = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSetterReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_setter_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_setter_return = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noStringCaseMismatch" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_string_case_mismatch = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_string_case_mismatch = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUndeclaredVariables" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_undeclared_variables = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_undeclared_variables = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnnecessaryContinue" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unnecessary_continue = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unnecessary_continue = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnreachable" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeFinally" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_finally = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_finally = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedVariables" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_variables = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_variables = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVoidElementsWithChildren" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_elements_with_children = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_elements_with_children = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVoidTypeReturn" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_type_return = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_void_type_return = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidForDirection" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_for_direction = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_for_direction = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Nursery {}
impl VisitNode<JsonLanguage> for Nursery {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noAssignInExpressions",
                "noBannedTypes",
                "noClassAssign",
                "noCommaOperator",
                "noConfusingArrow",
                "noConfusingLabels",
                "noDuplicateCase",
                "noDuplicateClassMembers",
                "noDuplicateJsxProps",
                "noExtraLabels",
                "noExtraSemicolons",
                "noGlobalObjectCalls",
                "noInferrableTypes",
                "noInnerDeclarations",
                "noInvalidConstructorSuper",
                "noNamespace",
                "noNoninteractiveElementToInteractiveRole",
                "noParameterAssign",
                "noParameterProperties",
                "noPrototypeBuiltins",
                "noRedeclare",
                "noRedundantAlt",
                "noRedundantRoles",
                "noRestrictedGlobals",
                "noSelfAssign",
                "noSelfCompare",
                "noSvgWithoutTitle",
                "noSwitchDeclarations",
                "noUnreachableSuper",
                "noUnsafeOptionalChaining",
                "noUnusedLabels",
                "noUselessCatch",
                "noUselessRename",
                "noUselessSwitchCase",
                "noWith",
                "useAriaPropTypes",
                "useAriaPropsForRole",
                "useCamelCase",
                "useExhaustiveDependencies",
                "useHookAtTopLevel",
                "useIframeTitle",
                "useIsNan",
                "useMediaCaption",
                "useNamespaceKeyword",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noAssignInExpressions" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_assign_in_expressions = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_assign_in_expressions = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noBannedTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_banned_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_banned_types = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noClassAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_class_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_class_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCommaOperator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comma_operator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comma_operator = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConfusingArrow" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_arrow = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_arrow = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConfusingLabels" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_labels = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_confusing_labels = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateCase" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_case = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_case = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateClassMembers" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_class_members = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_class_members = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateJsxProps" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_jsx_props = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_jsx_props = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExtraLabels" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_labels = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_labels = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExtraSemicolons" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_semicolons = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_semicolons = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noGlobalObjectCalls" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_global_object_calls = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_global_object_calls = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInferrableTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inferrable_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inferrable_types = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInnerDeclarations" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inner_declarations = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_inner_declarations = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noInvalidConstructorSuper" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_invalid_constructor_super = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_invalid_constructor_super = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNamespace" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_namespace = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_namespace = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNoninteractiveElementToInteractiveRole" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_noninteractive_element_to_interactive_role = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_noninteractive_element_to_interactive_role = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noParameterAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noParameterProperties" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_properties = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_parameter_properties = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noPrototypeBuiltins" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_prototype_builtins = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_prototype_builtins = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedeclare" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redeclare = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redeclare = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantAlt" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_alt = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_alt = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantRoles" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_roles = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_roles = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRestrictedGlobals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_restricted_globals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_restricted_globals = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSelfAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSelfCompare" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_compare = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_self_compare = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSvgWithoutTitle" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_svg_without_title = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_svg_without_title = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSwitchDeclarations" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_switch_declarations = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_switch_declarations = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnreachableSuper" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable_super = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unreachable_super = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeOptionalChaining" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_optional_chaining = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_optional_chaining = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedLabels" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_labels = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_labels = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessCatch" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_catch = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_catch = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessRename" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_rename = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_rename = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUselessSwitchCase" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_switch_case = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_useless_switch_case = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noWith" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_with = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_with = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAriaPropTypes" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_prop_types = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_prop_types = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useAriaPropsForRole" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_props_for_role = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_aria_props_for_role = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useCamelCase" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_camel_case = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_camel_case = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useExhaustiveDependencies" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exhaustive_dependencies = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exhaustive_dependencies = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useHookAtTopLevel" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_hook_at_top_level = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_hook_at_top_level = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useIframeTitle" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_iframe_title = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_iframe_title = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useIsNan" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_is_nan = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_is_nan = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useMediaCaption" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_media_caption = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_media_caption = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useNamespaceKeyword" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_namespace_keyword = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_namespace_keyword = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidAriaProps" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_aria_props = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_aria_props = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidLang" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_lang = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_lang = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useYield" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_yield = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_yield = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Performance {}
impl VisitNode<JsonLanguage> for Performance {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["recommended", "all", "noDelete"], diagnostics)
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDelete" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_delete = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_delete = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Security {}
impl VisitNode<JsonLanguage> for Security {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noDangerouslySetInnerHtml" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDangerouslySetInnerHtmlWithChildren" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html_with_children = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_dangerously_set_inner_html_with_children = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Style {}
impl VisitNode<JsonLanguage> for Style {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noArguments",
                "noImplicitBoolean",
                "noNegationElse",
                "noNonNullAssertion",
                "noShoutyConstants",
                "noUnusedTemplateLiteral",
                "noVar",
                "useBlockStatements",
                "useConst",
                "useDefaultParameterLast",
                "useEnumInitializers",
                "useExponentiationOperator",
                "useFragmentSyntax",
                "useNumericLiterals",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArguments" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_arguments = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_arguments = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noImplicitBoolean" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_implicit_boolean = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_implicit_boolean = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNegationElse" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_negation_else = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_negation_else = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noNonNullAssertion" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_non_null_assertion = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_non_null_assertion = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noShoutyConstants" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shouty_constants = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shouty_constants = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnusedTemplateLiteral" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_template_literal = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unused_template_literal = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noVar" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_var = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_var = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useBlockStatements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_block_statements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_block_statements = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useConst" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_const = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_const = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useDefaultParameterLast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_parameter_last = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_parameter_last = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useEnumInitializers" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_enum_initializers = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_enum_initializers = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useExponentiationOperator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exponentiation_operator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_exponentiation_operator = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useFragmentSyntax" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_fragment_syntax = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_fragment_syntax = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useNumericLiterals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_numeric_literals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_numeric_literals = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSelfClosingElements" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_self_closing_elements = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_self_closing_elements = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useShorthandArrayType" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_shorthand_array_type = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_shorthand_array_type = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSingleCaseStatement" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_case_statement = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_case_statement = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useSingleVarDeclarator" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_var_declarator = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_single_var_declarator = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useTemplate" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_template = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_template = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useWhile" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_while = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_while = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
impl VisitJsonNode for Suspicious {}
impl VisitNode<JsonLanguage> for Suspicious {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(
            node,
            &[
                "recommended",
                "all",
                "noArrayIndexKey",
                "noAsyncPromiseExecutor",
                "noCatchAssign",
                "noCommentText",
                "noCompareNegZero",
                "noConstEnum",
                "noDebugger",
                "noDoubleEquals",
                "noDuplicateObjectKeys",
                "noDuplicateParameters",
                "noEmptyInterface",
                "noExplicitAny",
                "noExtraNonNullAssertion",
                "noFunctionAssign",
                "noImportAssign",
                "noLabelVar",
                "noRedundantUseStrict",
                "noShadowRestrictedNames",
                "noSparseArray",
                "noUnsafeNegation",
                "useDefaultSwitchClauseLast",
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
            "all" => {
                self.all = Some(self.map_to_boolean(&value, name_text, diagnostics)?);
            }
            "noArrayIndexKey" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_array_index_key = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_array_index_key = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noAsyncPromiseExecutor" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_async_promise_executor = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_async_promise_executor = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCatchAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_catch_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_catch_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCommentText" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comment_text = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_comment_text = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noCompareNegZero" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_compare_neg_zero = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_compare_neg_zero = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noConstEnum" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_enum = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_const_enum = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDebugger" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_debugger = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_debugger = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDoubleEquals" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_double_equals = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_double_equals = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateObjectKeys" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_object_keys = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_object_keys = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noDuplicateParameters" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_parameters = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_duplicate_parameters = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noEmptyInterface" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_interface = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_empty_interface = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExplicitAny" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_explicit_any = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_explicit_any = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noExtraNonNullAssertion" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_non_null_assertion = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_extra_non_null_assertion = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noFunctionAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_function_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_function_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noImportAssign" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_import_assign = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_import_assign = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noLabelVar" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_label_var = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_label_var = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noRedundantUseStrict" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_use_strict = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_redundant_use_strict = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noShadowRestrictedNames" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shadow_restricted_names = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_shadow_restricted_names = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noSparseArray" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_sparse_array = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_sparse_array = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "noUnsafeNegation" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_negation = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.no_unsafe_negation = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useDefaultSwitchClauseLast" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_switch_clause_last = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_default_switch_clause_last = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            "useValidTypeof" => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_typeof = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_object(&value, name_text, &mut configuration, diagnostics)?;
                    self.use_valid_typeof = Some(configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            },
            _ => {}
        }
        Some(())
    }
}
