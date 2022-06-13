use std::fmt::Write;

use anyhow::{Context, Ok, Result};
use quote::{format_ident, quote};
use xtask::{glue::fs2, project_root};

pub fn generate_analyzer() -> Result<()> {
    let mut analyzers = Vec::new();
    generate_module("analyzers", &mut analyzers)?;

    let mut assists = Vec::new();
    generate_module("assists", &mut assists)?;

    update_registry_builder(analyzers, assists)
}

fn generate_module(name: &'static str, entries: &mut Vec<String>) -> Result<()> {
    let path = project_root().join("crates/rome_analyze/src").join(name);

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
            .join("crates/rome_analyze/src")
            .join(format!("{name}.rs")),
        tokens,
    )?;

    Ok(())
}

fn update_registry_builder(analyzers: Vec<String>, assists: Vec<String>) -> Result<()> {
    let path = project_root().join("crates/rome_analyze/src/registry.rs");

    let mut content = fs2::read_to_string(&path)?;

    // For now the builder is generated registry.rs using a macro invocation,
    // so it gets generated using a simple string find and replace. Eventually
    // the builders will be moved to language-specific crates and the whole file /
    // module will be generated instead
    let start_index = content
        .find("impl_registry_builders!(")
        .context("could not find start of impl_registry_builders macro")?;
    let end_index = content[start_index..]
        .find(");")
        .context("could not find end of impl_registry_builders macro")?;

    let mut builder_macro = String::new();

    writeln!(builder_macro, "impl_registry_builders!(")?;
    writeln!(builder_macro, "    // Analyzers")?;

    for analyzer in analyzers {
        writeln!(builder_macro, "    {analyzer},")?;
    }

    writeln!(builder_macro, "    // Assists")?;

    for assist in assists {
        writeln!(builder_macro, "    {assist},")?;
    }

    write!(builder_macro, ");")?;

    let range = start_index..(start_index + end_index + 2);
    content.replace_range(range, &builder_macro);
    fs2::write(path, &content)?;

    Ok(())
}
