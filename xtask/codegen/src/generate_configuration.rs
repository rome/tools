use case::CaseExt;
use proc_macro2::{Ident, Span};
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
            .insert(meta.name, meta.docs);
    }

    let mut struct_groups = Vec::new();
    let mut line_groups = Vec::new();
    let mut default_for_groups = Vec::new();
    for (group, rules) in groups {
        let mut declarations = Vec::new();

        for (rule, _) in rules {
            let rule = Ident::new(&to_lower_snake_case(rule), Span::call_site());
            let declaration = quote! {
                #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
                pub #rule: RuleConfiguration
            };
            declarations.push(declaration);
        }

        let property_group_name = Ident::new(&to_lower_snake_case(group), Span::call_site());

        let struct_group_name = Ident::new(
            &format!("{}Rules", group.to_capitalized()),
            Span::call_site(),
        );

        let the_struct = quote! {

            #[derive(Deserialize, Default, Serialize, Debug, Clone)]
            #[serde(rename_all = "camelCase", default, deny_unknown_fields)]
            pub struct #struct_group_name {
                #( #declarations ),*
            }

        };

        let line = quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #property_group_name: Option<#struct_group_name>
        };

        let default = quote! {
            #property_group_name: Some(#struct_group_name::default())
        };

        struct_groups.push(the_struct);
        line_groups.push(line);
        default_for_groups.push(default);
    }

    let groups = quote! {
        use serde::{Deserialize, Serialize};
        use crate::RuleConfiguration;


        #[derive(Deserialize, Serialize, Debug, Clone)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct Rules {
            #( #line_groups ),*
        }

        impl Default for Rules {
            fn default() -> Self {
                Self {
                    #( #default_for_groups ),*
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
