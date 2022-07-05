use anyhow::{Context, Ok, Result};
use quote::{format_ident, quote};
use xtask::{glue::fs2, project_root};

pub fn generate_analyzer() -> Result<()> {
    let mut analyzers = Vec::new();
    generate_module("analyzers", &mut analyzers)?;
    generate_module("semantic_analyzers", &mut analyzers)?;

    let mut assists = Vec::new();
    generate_module("assists", &mut assists)?;

    update_registry_builder(analyzers, assists)
}

fn generate_module(name: &'static str, entries: &mut Vec<String>) -> Result<()> {
    let path = project_root().join("crates/rome_js_analyze/src").join(name);

    let mut rules = Vec::new();
    for entry in fs2::read_dir(path)? {
        let entry = entry?.path();
        let file_name = entry
            .file_stem()
            .context("path has no file name")?
            .to_str()
            .context("could not convert file name to string")?;

        let mut rule_name = String::new();
        let mut chars = file_name.char_indices();

        while let Some((index, mut char)) = chars.next() {
            if index == 0 {
                char = char.to_ascii_uppercase();
            }

            if char == '_' {
                let (_, next_char) = chars.next().context("iterator is empty")?;
                char = next_char.to_ascii_uppercase();
            }

            rule_name.push(char);
        }

        let index = entries.binary_search(&rule_name).unwrap_err();
        entries.insert(index, rule_name.clone());

        let module_name = format_ident!("{}", file_name);
        let rule_name = format_ident!("{}", rule_name);

        rules.insert(
            index,
            quote! {
                mod #module_name;
                pub(crate) use #module_name::#rule_name;
            },
        );
    }

    let tokens = xtask::reformat(quote! {
        #( #rules )*
    })?;

    fs2::write(
        project_root()
            .join("crates/rome_js_analyze/src")
            .join(format!("{name}.rs")),
        tokens,
    )?;

    Ok(())
}

fn update_registry_builder(analyzers: Vec<String>, assists: Vec<String>) -> Result<()> {
    let path = project_root().join("crates/rome_js_analyze/src/registry.rs");

    let rules: Vec<_> = analyzers
        .into_iter()
        .chain(assists)
        .map(|rule| {
            let rule = format_ident!("{}", rule);
            quote! {
                if filter.match_rule::<#rule>() {
                    rules.push::<#rule>();
                }
            }
        })
        .collect();

    let tokens = xtask::reformat(quote! {
        use rome_analyze::{AnalyzerSignal, AnalysisFilter, ControlFlow, RuleRegistry};
        use rome_js_syntax::JsLanguage;

        use crate::{analyzers::*, semantic_analyzers::*, assists::*};

        pub(crate) fn build_registry<'a, F, B>(
            filter: &AnalysisFilter,
            callback: F,
        ) -> RuleRegistry<'a, JsLanguage, B>
        where
            F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
        {
            let mut rules = RuleRegistry::new(callback);
            #( #rules )*
            rules
        }
    })?;

    fs2::write(path, tokens)?;

    Ok(())
}
