use case::CaseExt;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use pulldown_cmark::{Event, Parser, Tag};
use quote::quote;
use rome_analyze::{
    GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup, RuleMetadata,
};
use rome_js_syntax::JsLanguage;
use rome_json_syntax::JsonLanguage;
use std::collections::BTreeMap;
use xtask::*;
use xtask_codegen::{to_lower_snake_case, update};

pub(crate) fn generate_rules_configuration(mode: Mode) -> Result<()> {
    let config_root = project_root().join("crates/rome_service/src/configuration/linter");
    let config_parsing_root =
        project_root().join("crates/rome_service/src/configuration/parse/json/");
    let push_rules_directory = project_root().join("crates/rome_service/src/configuration");

    #[derive(Default)]
    struct LintRulesVisitor {
        groups: BTreeMap<&'static str, BTreeMap<&'static str, RuleMetadata>>,
    }

    impl RegistryVisitor<JsLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_insert_with(BTreeMap::new)
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    impl RegistryVisitor<JsonLanguage> for LintRulesVisitor {
        fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
            if matches!(C::CATEGORY, RuleCategory::Lint) {
                C::record_groups(self);
            }
        }

        fn record_rule<R>(&mut self)
        where
            R: Rule + 'static,
            R::Query: Queryable<Language = JsonLanguage>,
            <R::Query as Queryable>::Output: Clone,
        {
            self.groups
                .entry(<R::Group as RuleGroup>::NAME)
                .or_insert_with(BTreeMap::new)
                .insert(R::METADATA.name, R::METADATA);
        }
    }

    let mut visitor = LintRulesVisitor::default();
    rome_js_analyze::visit_registry(&mut visitor);
    rome_json_analyze::visit_registry(&mut visitor);

    let LintRulesVisitor { groups } = visitor;

    let mut struct_groups = Vec::new();
    let mut line_groups = Vec::new();
    let mut default_for_groups = Vec::new();
    let mut group_rules_union = Vec::new();
    let mut group_match_code = Vec::new();
    let mut group_get_severity = Vec::new();
    let mut group_name_list = vec!["recommended", "all"];
    let mut rule_visitor_call = Vec::new();
    let mut visitor_rule_list = Vec::new();
    let mut push_rule_list = Vec::new();
    for (group, rules) in groups {
        group_name_list.push(group);
        let property_group_name = Ident::new(&to_lower_snake_case(group), Span::call_site());
        let group_struct_name = Ident::new(&group.to_capitalized(), Span::call_site());
        let group_name_string_literal = Literal::string(group);

        struct_groups.push(generate_struct(group, &rules));
        visitor_rule_list.push(generate_visitor(group, &rules));
        push_rule_list.push(generate_push_to_analyzer_rules(group));
        line_groups.push(quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[bpaf(external, hide, optional)]
            pub #property_group_name: Option<#group_struct_name>
        });
        default_for_groups.push(quote! {
            #property_group_name: None
        });

        let global_recommended = if group == "nursery" {
            quote! { self.is_recommended() && rome_flags::is_unstable() }
        } else {
            quote! { self.is_recommended() }
        };

        group_rules_union.push(quote! {
            if let Some(group) = self.#property_group_name.as_ref() {
                group.collect_preset_rules(self.is_recommended(), &mut enabled_rules, &mut disabled_rules);
                enabled_rules.extend(&group.get_enabled_rules());
                disabled_rules.extend(&group.get_disabled_rules());
            } else if self.is_all() {
                enabled_rules.extend(#group_struct_name::all_rules_as_filters());
            } else if self.is_not_all() {
                disabled_rules.extend(#group_struct_name::all_rules_as_filters());
            } else if #global_recommended {
                enabled_rules.extend(#group_struct_name::recommended_rules_as_filters());
            }
        });

        group_get_severity.push(quote! {
            #group => self
                .#property_group_name
                .as_ref()
                .and_then(|#property_group_name| #property_group_name.get_rule_configuration(rule_name))
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
           #group => #group_struct_name::has_rule(rule_name).then_some((category, rule_name))
        });

        rule_visitor_call.push(quote! {
            #group_name_string_literal => {
                let mut visitor = #group_struct_name::default();
                if are_recommended_and_all_correct(
                    &value,
                    name_text,
                    diagnostics,
                )? {
                    self.map_to_object(&value, name_text, &mut visitor, diagnostics)?;
                    self.#property_group_name = Some(visitor);
                }
            }
        });
    }

    let groups = quote! {
        use serde::{Deserialize, Serialize};
        #[cfg(feature = "schema")]
        use schemars::JsonSchema;
        use crate::RuleConfiguration;
        use rome_analyze::RuleFilter;
        use indexmap::IndexSet;
        use bpaf::Bpaf;
        use rome_diagnostics::{Category, Severity};

        #[derive(Deserialize, Serialize, Debug, Clone, Bpaf)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct Rules {
            /// It enables the lint rules recommended by Rome. `true` by default.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[bpaf(hide)]
            pub recommended: Option<bool>,

            /// It enables ALL rules. The rules that belong to `nursery` won't be enabled.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[bpaf(hide)]
            pub all: Option<bool>,

            #( #line_groups ),*
        }

        impl Default for Rules {
            fn default() -> Self {
                Self {
                    recommended: Some(true),
                    all: None,
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

            /// Given a category coming from [Diagnostic](rome_diagnostics::Diagnostic), this function returns
            /// the [Severity](rome_diagnostics::Severity) associated to the rule, if the configuration changed it.
            ///
            /// If not, the function returns [None].
            pub fn get_severity_from_code(&self, category: &Category) -> Option<Severity> {
                let mut split_code = category.name().split('/');

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

            pub(crate) const fn is_recommended(&self) -> bool {
                // It is only considered _not_ recommended when
                // the configuration is `"recommended": false`.
                // Hence, omission of the setting or set to `true` are considered recommended.
                !matches!(self.recommended, Some(false))
            }

            pub(crate) const fn is_all(&self) -> bool {
                matches!(self.all, Some(true))
            }

            pub(crate) const fn is_not_all(&self) -> bool {
                matches!(self.all, Some(false))
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
                #( #group_rules_union )*

                enabled_rules.difference(&disabled_rules).cloned().collect()
            }
        }

        #( #struct_groups )*
    };

    let visitors = quote! {
        use crate::configuration::linter::*;
        use crate::Rules;
        use rome_deserialize::json::{has_only_known_keys, VisitJsonNode};
        use rome_deserialize::{DeserializationDiagnostic, VisitNode};
        use rome_json_syntax::{AnyJsonValue, JsonLanguage};
        use rome_rowan::{AstNode, SyntaxNode};
        use crate::configuration::parse::json::linter::are_recommended_and_all_correct;

        impl VisitJsonNode for Rules {}

        impl VisitNode<JsonLanguage> for Rules {
            fn visit_member_name(
                &mut self,
                node: &SyntaxNode<JsonLanguage>,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<()> {
                has_only_known_keys(node, &[#( #group_name_list ),*], diagnostics)
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

                    #( #rule_visitor_call )*

                    _ => {}
                }

                Some(())
            }
        }

        #( #visitor_rule_list )*
    };

    let push_rules = quote! {
        use crate::configuration::linter::*;
        use crate::{RuleConfiguration, Rules};
        use rome_analyze::{AnalyzerRules, MetadataRegistry};

        pub(crate) fn push_to_analyzer_rules(
            rules: &Rules,
            metadata: &MetadataRegistry,
            analyzer_rules: &mut AnalyzerRules,
        ) {
            #( #push_rule_list )*
        }
    };

    let configuration = groups.to_string();
    let visitors = visitors.to_string();
    let push_rules = push_rules.to_string();

    update(
        &config_root.join("rules.rs"),
        &xtask::reformat(configuration)?,
        &mode,
    )?;

    update(
        &config_parsing_root.join("rules.rs"),
        &xtask::reformat(visitors)?,
        &mode,
    )?;

    update(
        &push_rules_directory.join("generated.rs"),
        &xtask::reformat(push_rules)?,
        &mode,
    )?;

    Ok(())
}

fn generate_struct(group: &str, rules: &BTreeMap<&'static str, RuleMetadata>) -> TokenStream {
    let mut lines_recommended_rule = Vec::new();
    let mut lines_recommended_rule_as_filter = Vec::new();
    let mut lines_all_rule_as_filter = Vec::new();
    let mut declarations = Vec::new();
    let mut lines_rule = Vec::new();
    let mut schema_lines_rules = Vec::new();
    let mut rule_enabled_check_line = Vec::new();
    let mut rule_disabled_check_line = Vec::new();
    let mut get_rule_configuration_line = Vec::new();

    let mut number_of_recommended_rules: u8 = 0;
    let number_of_rules = Literal::u8_unsuffixed(rules.len() as u8);
    for (index, (rule, metadata)) in rules.iter().enumerate() {
        let summary = {
            let mut docs = String::new();
            let parser = Parser::new(metadata.docs);
            for event in parser {
                match event {
                    Event::Text(text) => {
                        docs.push_str(text.as_ref());
                    }
                    Event::Code(text) => {
                        docs.push_str(text.as_ref());
                    }
                    Event::SoftBreak => {
                        docs.push(' ');
                    }

                    Event::Start(Tag::Paragraph) => {}
                    Event::End(Tag::Paragraph) => {
                        break;
                    }

                    Event::Start(tag) => match tag {
                        Tag::Strong | Tag::Paragraph => {
                            continue;
                        }

                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    Event::End(tag) => match tag {
                        Tag::Strong | Tag::Paragraph => {
                            continue;
                        }
                        _ => panic!("Unimplemented tag {:?}", { tag }),
                    },

                    _ => {
                        panic!("Unimplemented event {:?}", { event })
                    }
                }
            }
            docs
        };

        let rule_position = Literal::u8_unsuffixed(index as u8);
        let rule_identifier = Ident::new(&to_lower_snake_case(rule), Span::call_site());
        let rule_cli_identifier = Literal::string(&to_lower_snake_case(rule).to_dashed());
        let declaration = quote! {
            #[serde(skip_serializing_if = "RuleConfiguration::is_err")]
            pub #rule_identifier: RuleConfiguration
        };
        declarations.push(declaration);
        if metadata.recommended {
            lines_recommended_rule_as_filter.push(quote! {
                RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[#rule_position])
            });

            lines_recommended_rule.push(quote! {
                #rule
            });
            number_of_recommended_rules += 1;
        }
        lines_all_rule_as_filter.push(quote! {
            RuleFilter::Rule(Self::GROUP_NAME, Self::GROUP_RULES[#rule_position])
        });
        lines_rule.push(quote! {
             #rule
        });
        schema_lines_rules.push(quote! {
            #[doc = #summary]
            #[bpaf(long(#rule_cli_identifier), argument("on|off|warn"), optional, hide)]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub #rule_identifier: Option<RuleConfiguration>
        });

        rule_enabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref() {
                if rule.is_enabled() {
                    index_set.insert(RuleFilter::Rule(
                        Self::GROUP_NAME,
                        Self::GROUP_RULES[#rule_position],
                    ));
                }
            }
        });
        rule_disabled_check_line.push(quote! {
            if let Some(rule) = self.#rule_identifier.as_ref() {
                if rule.is_disabled() {
                    index_set.insert(RuleFilter::Rule(
                        Self::GROUP_NAME,
                        Self::GROUP_RULES[#rule_position],
                    ));
                }
            }
        });

        get_rule_configuration_line.push(quote! {
            #rule => self.#rule_identifier.as_ref()
        });
    }

    let group_struct_name = Ident::new(&group.to_capitalized(), Span::call_site());

    let number_of_recommended_rules = Literal::u8_unsuffixed(number_of_recommended_rules);
    let (group_recommended, parent_parameter) = if group == "nursery" {
        (
            quote! { self.is_recommended() },
            quote! { _parent_is_recommended: bool, },
        )
    } else {
        (
            quote! { parent_is_recommended || self.is_recommended() },
            quote! { parent_is_recommended: bool, },
        )
    };
    quote! {
        #[derive(Deserialize, Default, Serialize, Debug, Clone, Bpaf)]
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        #[serde(rename_all = "camelCase", default)]
        /// A list of rules that belong to this group
        pub struct #group_struct_name {
            /// It enables the recommended rules for this group
            #[serde(skip_serializing_if = "Option::is_none")]
            #[bpaf(hide)]
            pub recommended: Option<bool>,

            /// It enables ALL rules for this group.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[bpaf(hide)]
            pub all: Option<bool>,

            #( #schema_lines_rules ),*
        }

        impl #group_struct_name {

            const GROUP_NAME: &'static str = #group;
            pub(crate) const GROUP_RULES: [&'static str; #number_of_rules] = [
                #( #lines_rule ),*
            ];

            const RECOMMENDED_RULES: [&'static str; #number_of_recommended_rules] = [
                #( #lines_recommended_rule ),*
            ];

            const RECOMMENDED_RULES_AS_FILTERS: [RuleFilter<'static>; #number_of_recommended_rules] = [
                #( #lines_recommended_rule_as_filter ),*
            ];

            const ALL_RULES_AS_FILTERS: [RuleFilter<'static>; #number_of_rules] = [
                #( #lines_all_rule_as_filter ),*
            ];

            /// Retrieves the recommended rules
            pub(crate) fn is_recommended(&self) -> bool {
                // we should inject recommended rules only when they are set to "true"
                matches!(self.recommended, Some(true))
            }

            pub(crate) const fn is_not_recommended(&self) -> bool {
                matches!(self.recommended, Some(false))
            }

            pub(crate) fn is_all(&self) -> bool {
                matches!(self.all, Some(true))
            }

            pub(crate) fn is_not_all(&self) -> bool {
                matches!(self.all, Some(false))
            }

            pub(crate) fn get_enabled_rules(&self) -> IndexSet<RuleFilter> {
               let mut index_set = IndexSet::new();
               #( #rule_enabled_check_line )*
               index_set
            }

            pub(crate) fn get_disabled_rules(&self) -> IndexSet<RuleFilter> {
               let mut index_set = IndexSet::new();
               #( #rule_disabled_check_line )*
               index_set
            }

            /// Checks if, given a rule name, matches one of the rules contained in this category
            pub(crate) fn has_rule(rule_name: &str) -> bool {
                Self::GROUP_RULES.contains(&rule_name)
            }

            /// Checks if, given a rule name, it is marked as recommended
            pub(crate) fn is_recommended_rule(rule_name: &str) -> bool {
                 Self::RECOMMENDED_RULES.contains(&rule_name)
            }

            pub(crate) fn recommended_rules_as_filters() -> [RuleFilter<'static>; #number_of_recommended_rules] {
                Self::RECOMMENDED_RULES_AS_FILTERS
            }

            pub(crate) fn all_rules_as_filters() -> [RuleFilter<'static>; #number_of_rules] {
                Self::ALL_RULES_AS_FILTERS
            }

            /// Select preset rules
            pub(crate) fn collect_preset_rules(
                &self,
                #parent_parameter
                enabled_rules: &mut IndexSet<RuleFilter>,
                disabled_rules: &mut IndexSet<RuleFilter>,
            ) {
                if self.is_all() {
                    enabled_rules.extend(Self::all_rules_as_filters());
                } else if #group_recommended {
                    enabled_rules.extend(Self::recommended_rules_as_filters());
                }
                if self.is_not_all() {
                    disabled_rules.extend(Self::all_rules_as_filters());
                } else if self.is_not_recommended() {
                    disabled_rules.extend(Self::recommended_rules_as_filters());
                }
            }

            pub(crate) fn get_rule_configuration(&self, rule_name: &str) -> Option<&RuleConfiguration> {
                match rule_name {
                    #( #get_rule_configuration_line ),*,
                    _ => None
                }
            }
        }
    }
}

fn generate_visitor(group: &str, rules: &BTreeMap<&'static str, RuleMetadata>) -> TokenStream {
    let group_struct_name = Ident::new(&group.to_capitalized(), Span::call_site());
    let mut group_rules = vec![Literal::string("recommended"), Literal::string("all")];
    let mut visitor_rule_line = Vec::new();

    for rule_name in rules.keys() {
        let rule_identifier = Ident::new(&to_lower_snake_case(rule_name), Span::call_site());
        group_rules.push(Literal::string(rule_name));
        visitor_rule_line.push(quote! {
            #rule_name => match value {
                AnyJsonValue::JsonStringValue(_) => {
                    let mut configuration = RuleConfiguration::default();
                    self.map_to_known_string(&value, name_text, &mut configuration, diagnostics)?;
                    self.#rule_identifier = Some(configuration);
                }
                AnyJsonValue::JsonObjectValue(_) => {
                    let mut rule_configuration = RuleConfiguration::default();
                    rule_configuration.map_rule_configuration(&value, name_text, #rule_name, diagnostics)?;
                    self.#rule_identifier = Some(rule_configuration);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "object or string",
                        value.range(),
                    ));
                }
            }
        });
    }

    quote! {
        impl VisitJsonNode for #group_struct_name {}

        impl VisitNode<JsonLanguage> for #group_struct_name {
            fn visit_member_name(
                &mut self,
                node: &SyntaxNode<JsonLanguage>,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<()> {
                has_only_known_keys(node, &[#( #group_rules ),*], diagnostics)
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

                    #( #visitor_rule_line ),*,
                    _ => {}
                }

                Some(())
            }
        }
    }
}

fn generate_push_to_analyzer_rules(group: &str) -> TokenStream {
    let group_struct_name = Ident::new(&group.to_capitalized(), Span::call_site());
    let group_identifier = Ident::new(group, Span::call_site());
    quote! {
       if let Some(rules) = rules.#group_identifier.as_ref() {
            for rule_name in &#group_struct_name::GROUP_RULES {
                if let Some(RuleConfiguration::WithOptions(rule_options)) =
                    rules.get_rule_configuration(rule_name)
                {
                    if let Some(possible_options) = &rule_options.options {
                        if let Some(rule_key) = metadata.find_rule(#group, rule_name) {
                        let rule_options = possible_options.extract_option(&rule_key);
                        analyzer_rules.push_rule(rule_key, rule_options);
                        }
                    }
                }
            }
        }
    }
}
