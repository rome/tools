// mod extract;

// use crate::project_root;
// use convert_case::{Case, Casing};
// use extract::*;
// use quote::ToTokens;
// use rslint_config::ConfigRepr;
// use schemars::schema_for;
// use std::collections::HashMap;
// use std::error::Error;
// use std::fs::{read_dir, read_to_string, write};

// const GROUPS_ROOT: &str = "crates/rslint_core/src/groups";

// const REPO: &str = "https://github.com/rslint/rslint/tree/master";

// pub fn run() {
//     let mut groups = vec![];
//     for file in read_dir(project_root().join(GROUPS_ROOT))
//         .expect("Unreadable groups dir")
//         .filter_map(Result::ok)
//     {
//         if file.file_type().expect("No file type").is_dir() {
//             let res = parse_group_mod(
//                 &read_to_string(file.path().join("assignment")).expect("No mod file for group"),
//             );
//             let meta = res.expect("No group! declaration in group");

//             let dir = project_root().join("website/docs/rules").join(&meta.name);

//             let res = extract_group(&meta.name).expect("Failed to extract group rule data");
//             let mut v: Vec<_> = res.into_iter().collect();
//             v.sort_by(|x, y| x.0.cmp(&y.0));

//             let data = group_markdown(&v, &meta);

//             write(dir.join("README.md"), data).expect("Failed to write group md");
//             for (name, rule) in v {
//                 write(
//                     dir.join(name.replace("_", "-")).with_extension("md"),
//                     rule_markdown(rule, &meta),
//                 )
//                 .expect("Failed to write rule markdown");
//             }
//             groups.push(meta);
//         }
//     }
//     write(
//         project_root().join("website/docs/rules/README.md"),
//         rules_markdown(groups),
//     )
//     .expect("Failed to write rules readme");
//     write(
//         project_root().join("editors/vscode/schema.json"),
//         serde_json::to_string_pretty(&schema_for!(ConfigRepr)).unwrap(),
//     )
//     .expect("Failed to write schema")
// }

// const RULES_PRELUDE: &str =
// "
// <!--
// generated docs file, do not edit by hand, see xtask/docgen
// -->

// User documentation for RSLint rules. RSLint groups rules by their scope, each group
// has a specific scope. Grouping rules allows RSLint to distinctly group rules for a better project structure,
// as well as allowing users to disable a whole group of rules.

// ";

// pub fn rules_markdown(groups: Vec<Group>) -> String {
//     let mut ret = RULES_PRELUDE.to_string();

//     ret.push_str("\n## Groups \n");
//     ret.push_str("| Name | Description |\n");
//     ret.push_str("| ---- | ----------- |\n");

//     for group in groups.into_iter() {
//         ret.push_str(&format!(
//             "| [{}](./{}) | {} |\n",
//             group.name,
//             group.name,
//             group.docstring.replace("\n", "<br>")
//         ));
//     }

//     ret
// }

// pub fn group_markdown(data: &[(String, RuleFile)], group: &Group) -> String {
//     let mut ret = String::new();
//     ret.insert_str(
//         0,
//         "<!--\n generated docs file, do not edit by hand, see xtask/docgen \n-->\n",
//     );
//     ret.push_str(&format!("\n# {}\n\n", group.name.to_case(Case::Pascal)));
//     ret.push_str(group.docstring.trim());

//     ret.push_str("\n## Rules\n");
//     ret.push_str("| Name | Description |\n");
//     ret.push_str("| ---- | ----------- |\n");

//     for (name, rule) in data {
//         let user_facing_name = &name.replace("_", "-")[..name.len() - 3];
//         ret.push_str(&format!(
//             "| [{}](./{}.md) | {} |\n",
//             user_facing_name,
//             user_facing_name,
//             first_sentence(&rule.lint_declaration.docstring.clone().unwrap_or_default())
//                 .unwrap_or_default()
//         ));
//     }
//     ret.push_str(&format!(
//         "\n[Source]({}/crates/rslint_core/src/groups/{})",
//         REPO, group.name
//     ));
//     ret
// }

// pub fn rule_src(group_name: &str, rule_name: &str) -> String {
//     format!(
//         "{}/crates/rslint_core/src/groups/{}/{}.rs",
//         REPO, group_name, rule_name
//     )
// }

// pub fn first_sentence(string: &str) -> Option<&str> {
//     string.trim().split('\n').next().map(|x| x.trim())
// }

// pub fn extract_group(group_name: &str) -> Result<HashMap<String, RuleFile>, Box<dyn Error>> {
//     let dir = read_dir(project_root().join(GROUPS_ROOT).join(group_name))?;
//     let mut res = HashMap::new();
//     for file in dir.filter_map(Result::ok) {
//         if let Some(parsed) = parse_rule_file(&read_to_string(file.path())?)? {
//             res.insert(file.file_name().to_string_lossy().to_string(), parsed);
//         }
//     }
//     Ok(res)
// }

// pub fn rule_markdown(rule: RuleFile, group: &Group) -> String {
//     let mut ret = rule
//         .lint_declaration
//         .docstring
//         .unwrap_or_default()
//         .replace("```ignore", "```js");
//     ret.insert_str(
//         0,
//         &format!(
//             "<!--\n generated docs file, do not edit by hand, see xtask/docgen \n-->\n# {}\n\n",
//             rule.lint_declaration.name
//         ),
//     );

//     if !rule.lint_declaration.config_fields.is_empty() {
//         ret.push_str("\n## Config\n");
//         ret.push_str("| Name | Type | Description |\n");
//         ret.push_str("| ---- | ---- | ----------- |\n");

//         for config in rule.lint_declaration.config_fields.iter() {
//             ret.push_str(&format!(
//                 "| `{}` | {} | {} |\n",
//                 config
//                     .field
//                     .ident
//                     .as_ref()
//                     .unwrap()
//                     .to_string()
//                     .to_case(Case::Camel),
//                 config.field.ty.to_token_stream().to_string(),
//                 config
//                     .docstring
//                     .clone()
//                     .unwrap_or_default()
//                     .replace("\n", "<br>")
//             ));
//         }
//     }

//     if let Some(tests) = rule.tests {
//         if !tests.err_examples.is_empty() {
//             ret.push_str("\n::: details More incorrect examples\n");
//             for example in tests.err_examples {
//                 ret.push_str(&format!(
//                     "{}\n```js\n{}\n```\n",
//                     example.docstring.unwrap_or_default(),
//                     example.source
//                 ));
//             }
//             ret.push_str(":::");
//         }
//         if !tests.ok_examples.is_empty() {
//             ret.push_str("\n::: details More correct examples\n");
//             for example in tests.ok_examples {
//                 ret.push_str(&format!(
//                     "{}\n```js\n{}\n```\n",
//                     example.docstring.unwrap_or_default(),
//                     example.source
//                 ));
//             }
//             ret.push_str(":::");
//         }
//     }

//     ret.push_str(&format!(
//         "\n\n[Source]({})",
//         rule_src(&group.name, &rule.lint_declaration.name.replace("-", "_"))
//     ));
//     ret
// }
