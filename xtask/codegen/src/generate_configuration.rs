use case::CaseExt;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use rome_analyze::{AnalysisFilter, RuleCategories};
use rome_js_analyze::metadata;
use std::collections::BTreeMap;
use xtask::*;
use xtask_codegen::{to_lower_snake_case, update};

pub(crate) fn generate_rules_configuration(mode: Mode) -> Result<()> {
    let config_root = project_root().join("crates/rome_service/src/configuration/linter");

    let filter = AnalysisFilter {
        categories: RuleCategories::LINT,
        ..AnalysisFilter::default()
    };

    // Ensure the list of rules is stored alphabetically
    let mut groups = BTreeMap::new();
    for meta in metadata(filter) {
        groups
            .entry(meta.group)
            .or_insert_with(BTreeMap::new)
            .insert(meta.rule.name, meta.rule.recommended);
    }

    let mut struct_groups = Vec::new();
    let mut line_groups = Vec::new();
    let mut default_for_groups = Vec::new();
    let mut group_rules_union = Vec::new();
    let mut group_match_code = Vec::new();
    let mut group_get_severity = Vec::new();
    for (group, rules) in groups {
        let mut lines_recommended_rule = Vec::new();
        let mut lines_recommended_rule_as_filter = Vec::new();
        let mut declarations = Vec::new();
        let mut lines_rule = Vec::new();
        let mut schema_lines_rules = Vec::new();
        let property_group_name = Ident::new(&to_lower_snake_case(group), Span::call_site());

        let mut number_of_recommended_rules: u8 = 0;
        let number_of_rules = Literal::u8_unsuffixed(rules.len() as u8);
        for (index, (rule, recommended)) in rules.iter().enumerate() {
            let rule_position = Literal::u8_unsuffixed(index as u8);
            let rule_identifier = Ident::new(&to_lower_snake_case(rule), Span::call_site());
            let declaration = quote! {
                #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
                pub #rule_identifier: RuleConfiguration
            };
            declarations.push(declaration);
            if *recommended {
                lines_recommended_rule_as_filter.push(quote! {
                    RuleFilter::Rule(#group, Self::CATEGORY_RULES[#rule_position])
                });

                lines_recommended_rule.push(quote! {
                    #rule
                });
                number_of_recommended_rules += 1;
            }
            lines_rule.push(quote! {
                 #rule
            });
            schema_lines_rules.push(quote! {
                #rule_identifier: Option<RuleConfiguration>
            });
        }

        let group_struct_name = Ident::new(&group.to_capitalized().to_string(), Span::call_site());
        let group_schema_struct_name = Ident::new(
            &format!("{}Schema", &group.to_capitalized().to_string()),
            Span::call_site(),
        );
        let group_schema_struct_name_as_literal =
            Literal::string(&format!("{}Schema", &group.to_capitalized().to_string()));

        let number_of_recommended_rules = Literal::u8_unsuffixed(number_of_recommended_rules);
        let deserialize_function_string = Literal::string(&format!("deserialize_{}_rules", group));
        let deserialize_function_ident =
            Ident::new(&format!("deserialize_{}_rules", group), Span::call_site());

        let group_struct = quote! {
            #[derive(Deserialize, Default, Serialize, Debug, Clone)]
            #[cfg_attr(feature = "schemars", derive(JsonSchema))]
            #[serde(rename_all = "camelCase", default)]
            pub struct #group_struct_name {
                /// It enables the recommended rules for this group
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                /// List of rules for the current group
                #[serde(skip_serializing_if = "IndexMap::is_empty", deserialize_with = #deserialize_function_string, flatten)]
                #[cfg_attr(feature = "schemars", schemars(with = #group_schema_struct_name_as_literal))]
                pub rules: IndexMap<String, RuleConfiguration>,
            }

            // Struct is only used when generating the JSON schema
            #[cfg_attr(feature = "schemars", derive(JsonSchema), serde(rename_all = "camelCase"))]
            #[allow(dead_code)]
            /// A list of rules that belong to this group
            struct #group_schema_struct_name {
                #( #schema_lines_rules ),*
            }


            impl #group_struct_name {

                const CATEGORY_NAME: &'static str = #group;
                pub(crate) const CATEGORY_RULES: [&'static str; #number_of_rules] = [
                    #( #lines_rule ),*
                ];

                const RECOMMENDED_RULES: [&'static str; #number_of_recommended_rules] = [
                    #( #lines_recommended_rule ),*
                ];

                const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; #number_of_recommended_rules] = [
                    #( #lines_recommended_rule_as_filter ),*
                ];




                pub(crate) fn is_recommended(&self) -> bool {
                    !matches!(self.recommended, Some(false))
                }


                pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
                   IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
                        if conf.is_enabled() {
                            Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
                        } else {
                            None
                        }
                    }))
                }

                pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
                   IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
                        if conf.is_disabled() {
                            Some(RuleFilter::Rule(Self::CATEGORY_NAME, key))
                        } else {
                            None
                        }
                    }))
                }

                /// Checks if, given a rule name, matches one of the rules contained in this category
                pub(crate) fn has_rule(rule_name: &str) -> bool {
                    Self::CATEGORY_RULES.contains(&rule_name)
                }

                /// Checks if, given a rule name, it is marked as recommended
                pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
                     Self::RECOMMENDED_RULES.contains(&rule_name)
                }

                pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; #number_of_recommended_rules] {
                    Self::RECOMMENDED_RULES_AS_FILTERS
                }
            }

            fn #deserialize_function_ident<'de, D>(
                deserializer: D,
            ) -> Result<IndexMap<String, RuleConfiguration>, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let value: IndexMap<String, RuleConfiguration> = Deserialize::deserialize(deserializer)?;
                for rule_name in value.keys() {
                    if !#group_struct_name::CATEGORY_RULES.contains(&rule_name.as_str()) {
                        return Err(serde::de::Error::custom(RomeError::Configuration(
                            ConfigurationError::UnknownRule(rule_name.to_string()),
                        )));
                    }
                }
                Ok(value)
            }
        };

        struct_groups.push(group_struct);
        line_groups.push(quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #property_group_name: Option<#group_struct_name>
        });
        default_for_groups.push(quote! {
            #property_group_name: None
        });
        group_rules_union.push(quote! {
            if let Some(group) = self.#property_group_name.as_ref() {
                if self.is_recommended() && group.is_recommended() {
                    enabled_rules.extend(#group_struct_name::recommended_rules_as_filters());
                }
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            } else if self.is_recommended() {
                enabled_rules.extend(#group_struct_name::recommended_rules_as_filters());
            }
        });

        group_get_severity.push(quote! {
            #group => self
                .#property_group_name
                .as_ref()
                .and_then(|#property_group_name| #property_group_name.rules.get(rule_name))
                .map(|rule_setting| rule_setting.into())
                .unwrap_or_else(|| {
                    if #group_struct_name::is_recommended_rule(rule_name) {
                        Severity::Error
                    } else {
                        Severity::Warning
                    }
                })
        });
        group_match_code.push(quote! {
           #group => #group_struct_name::has_rule(rule_name).then(|| (category, rule_name))
        });
    }

    let groups = quote! {
        use serde::{Deserialize, Serialize};
        #[cfg(feature = "schemars")]
        use schemars::JsonSchema;
        use crate::{ConfigurationError, RomeError, RuleConfiguration};
        use rome_analyze::RuleFilter;
        use indexmap::{IndexMap, IndexSet};
        use rome_console::codespan::Severity;

        #[derive(Deserialize, Serialize, Debug, Clone)]
        #[cfg_attr(feature = "schemars", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct Rules {
            /// It enables the lint rules recommended by Rome. `true` by default.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub recommended: Option<bool>,

            #( #line_groups ),*
        }

        impl Default for Rules {
            fn default() -> Self {
                Self {
                    recommended: Some(true),
                    #( #default_for_groups ),*
                }
            }
        }
        impl Rules {

            /// Checks if the code coming from [rome_diagnostics::Diagnostic] corresponds to a rule.
            /// Usually the code is built like {category}/{rule_name}
            pub fn matches_diagnostic_code<'a>(
                &self,
                category: Option<&'a str>,
                rule_name: Option<&'a str>,
            ) -> Option<(&'a str, &'a str)> {
                match (category, rule_name) {
                    (Some(category), Some(rule_name)) => match category {
                        #( #group_match_code ),*,

                        _ => None
                    },
                    _ => None
                }
            }

            /// Given a code coming from [Diagnostic](rome_diagnostics::Diagnostic), this function returns
            /// the [Severity](rome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_code(&self, code: &str) -> Option<Severity> {
                let mut split_code = code.split('/');

                let _lint = split_code.next();
                debug_assert_eq!(_lint, Some("lint"));

                let group = split_code.next();
                let rule_name = split_code.next();

                if let Some((group, rule_name)) = self.matches_diagnostic_code(group, rule_name) {
                    let severity = match group {
                        #( #group_get_severity ),*,

                        _ => unreachable!("this group should not exist, found {}", group),
                    };
                    Some(severity)
                } else {
                    None
                }
            }

            pub(crate) fn is_recommended(&self) -> bool {
                // It is only considered _not_ recommended when
                // the configuration is `"recommended": false`.
                // Hence, omission of the setting or set to `true` are considered recommneded.
                !matches!(self.recommended, Some(false))
            }

            /// It returns a tuple of filters. The first element of the tuple are the enabled rules,
            /// while the second element are the disabled rules.
            ///
            /// Only one element of the tuple is [Some] at the time.
            ///
            /// The enabled rules are calculated from the difference with the disabled rules.
            pub fn as_enabled_rules(&self) -> IndexSet<RuleFilter> {
                let mut enabled_rules = IndexSet::new();
                let mut disabled_rules = IndexSet::new();
                // computing the disabled rules
                #( #group_rules_union )*

                // computing the enabled rules
                #( #group_rules_union )*

                enabled_rules.difference(&disabled_rules).cloned().collect()
            }
        }

        #( #struct_groups )*
    };

    let ast = groups.to_string();

    let pretty = xtask::reformat(ast)?;

    update(&config_root.join("rules.rs"), &pretty, &mode)?;

    Ok(())
}
