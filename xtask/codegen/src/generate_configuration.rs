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
            .insert(meta.name, meta.recommended);
    }

    let mut struct_groups = Vec::new();
    let mut line_groups = Vec::new();
    let mut default_for_groups = Vec::new();
    let mut group_line_recommended_rules = Vec::new();
    let mut group_rules_union = Vec::new();
    for (group, rules) in groups {
        let mut lines_recommended_rule = Vec::new();
        let mut declarations = Vec::new();
        let mut lines_rule = Vec::new();
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
                lines_recommended_rule.push(quote! {
                    RuleFilter::Rule(#group, Self::GROUP_RULES[#rule_position])
                });
                number_of_recommended_rules += 1;
            }
            lines_rule.push(quote! {
                 #rule
            });
        }

        let group_struct_name = Ident::new(&group.to_capitalized().to_string(), Span::call_site());

        let number_of_recommended_rules = Literal::u8_unsuffixed(number_of_recommended_rules);
        let deserialize_function_string = Literal::string(&format!("deserialize_{}_rules", group));
        let deserialize_function_ident =
            Ident::new(&format!("deserialize_{}_rules", group), Span::call_site());

        let group_struct = quote! {
            #[derive(Deserialize, Default, Serialize, Debug, Clone)]
            #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
            pub struct #group_struct_name {
                /// It enables the recommended rules for this group
                #[serde(skip_serializing_if = "Option::is_none")]
                pub recommended: Option<bool>,

                /// List of rules for the current group
                #[serde(skip_serializing_if = "IndexMap::is_empty", deserialize_with = #deserialize_function_string)]
                pub rules: IndexMap<String, RuleConfiguration>,
            }


            impl #group_struct_name {

                const GROUP_NAME: &'static str = #group;
                pub(crate) const GROUP_RULES: [&'static str; #number_of_rules] = [
                    #( #lines_rule ),*
                ];

                const RECOMMENDED_RULES: [RuleFilter<'static>; #number_of_recommended_rules] = [
                    #( #lines_recommended_rule ),*
                ];

                pub(crate) fn is_recommended(&self) -> bool {
                    matches!(self.recommended, Some(true))
                }


                pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
                   IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
                        if conf.is_enabled() {
                            Some(RuleFilter::Rule(Self::GROUP_NAME, key))
                        } else {
                            None
                        }
                    }))
                }

                pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
                   IndexSet::from_iter(self.rules.iter().filter_map(|(key, conf)| {
                        if conf.is_disabled() {
                            Some(RuleFilter::Rule(Self::GROUP_NAME, key))
                        } else {
                            None
                        }
                    }))
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
                    if !#group_struct_name::GROUP_RULES.contains(&rule_name.as_str()) {
                        return Err(serde::de::Error::custom(RomeError::Configuration(
                            ConfigurationError::DeserializationError(format!("Invalid rule name `{rule_name}`")),
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
        group_line_recommended_rules.push(quote! {
            enabled_rules.extend(#group_struct_name::RECOMMENDED_RULES);
        });
        group_rules_union.push(quote! {
            if let Some(group) = self.#property_group_name.as_ref() {
                if group.is_recommended() {
                    enabled_rules.extend(&Js::RECOMMENDED_RULES);
                }
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            }
        });
    }

    let groups = quote! {
        use serde::{Deserialize, Serialize};
        use crate::{ConfigurationError, RomeError, RuleConfiguration};
        use rome_analyze::RuleFilter;
        use indexmap::{IndexMap, IndexSet};

        #[derive(Deserialize, Serialize, Debug, Clone)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct Rules {
            /// It enables a preset of rules of any group recommended by Rome. `true` by default.
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
            pub(crate) fn is_recommended(&self) -> bool {
                matches!(self.recommended, Some(true))
            }

            /// It returns a tuple of filters. The first element of the tuple are the enabled rules,
            /// while the second element are the disabled rules.
            ///
            /// Only one element of the tuple is [Some] at the time.
            ///
            /// The enabled rules are calculated from the difference with the disabled rules.
            pub fn as_analysis_filters(&self) -> (Option<IndexSet<RuleFilter>>, Option<IndexSet<RuleFilter>>) {
                let mut enabled_rules = IndexSet::new();
                let mut disabled_rules = IndexSet::new();
                if self.is_recommended() {
                    #( #group_line_recommended_rules );*
                }
                // computing the disabled rules
                #( #group_rules_union )*

                // computing the enabled rules
                #( #group_rules_union )*

                if enabled_rules.len() > disabled_rules.len() {
                    (None, Some(disabled_rules))
                } else {
                    (
                        Some(enabled_rules.difference(&disabled_rules).cloned().collect()),
                        None,
                    )
                }
            }
        }

        #( #struct_groups )*
    };

    let ast = groups.to_string();

    let pretty = xtask::reformat(ast)?;

    update(&config_root.join("rules.rs"), &pretty, &mode)?;

    Ok(())
}
