use bpaf::Bpaf;
use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_deserialize::{
    json::{has_only_known_keys, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use rome_js_syntax::JsModuleSource;
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};
use rome_rowan::{AstNode, SyntaxTokenText};
use serde::{Deserialize, Serialize};
use std::{path::Path, str::FromStr};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_rule! {
    /// Forbids importing from nested modules.
    ///
    /// For larger code bases, it can be undesirable to let any file arbitrarily import any other
    /// files. Arbitrary imports can lead to cycles that may be hard to debug, so it may be
    /// advisable to specify which files may import from which other files.
    ///
    /// A useful rule of thumb is that for modules that consist of several files, only the module's
    /// `index.js` or `index.ts` may be imported directly from outside that module, while symbols
    /// from other files should only be considered "public" if they're re-exported from the index.
    ///
    /// This rule treats nested imports as an attempt to access "private" internals of a module.
    /// Only exports defined by the `index.js` or `index.ts` are allowed to be imported externally.
    /// Effectively, this means that you may not directly import any files or subdirectories that
    /// are not siblings to the file you're in, or any of its ancestors.
    ///
    /// This rule only applies to relative imports, since the API from external dependencies is
    /// often out of your control.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { privateInternals } from "../aunt/cousin";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { publicExport } from "./sibling";
    /// import { reexportedInternals } from "../aunt";
    /// ```
    ///
    pub(crate) NoNestedModuleImports {
        version: "next",
        name: "noNestedModuleImports",
        recommended: false,
    }
}

impl Rule for NoNestedModuleImports {
    type Query = Ast<JsModuleSource>;
    type State = ();
    type Signals = Vec<Self::State>;
    type Options = ImportOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let Ok(path) = binding.inner_string_text() else {
            return Vec::new();
        };

        if is_public_import(path, ctx.options()) {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        ctx.query().inner_string_text().ok().map(|path| {
            let path = path.text();
            let parent = Path::new(path)
                .parent()
                .and_then(Path::to_str)
                .unwrap_or_default();

            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Importing from nested modules is not allowed."
                },
            )
            .note(markup! {
                "Please import from "<Emphasis>{parent}</Emphasis>" instead "
                "(you may need to re-export from "<Emphasis>{path}</Emphasis>")."
            })
        })
    }
}

fn is_public_import(module_path: SyntaxTokenText, options: &ImportOptions) -> bool {
    if !module_path.starts_with('.') {
        return true;
    }

    // Make an exception for loading resoures directly:
    for allowed_extension in &options.allowed_extensions {
        if module_path.ends_with(allowed_extension) {
            return true;
        }
    }

    module_path
        .split('/')
        .filter(|&part| part != "." && part != "..")
        .count()
        <= 1
}

/// Options for the rule `noNestedModuleImports`.
#[derive(Default, Deserialize, Serialize, Debug, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImportOptions {
    /// List of extensions that are always allowed to be imported.
    pub allowed_extensions: Vec<String>,
}

impl FromStr for ImportOptions {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default())
    }
}

impl VisitJsonNode for ImportOptions {}
impl VisitNode<JsonLanguage> for ImportOptions {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, &["allowedExtensions"], diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "allowedExtensions" {
            let array = value.as_json_array_value()?;

            for element in array.elements() {
                let element = element.ok()?;

                if let Some(extension) = element.as_json_string_value() {
                    self.allowed_extensions.push(extension.to_string());
                } else {
                    diagnostics.push(DeserializationDiagnostic::new(markup! {
                        "The field "<Emphasis>"allowedExtensions"</Emphasis>" must contain an array of strings"
                    })
                    .with_range(element.range()));
                }
            }
        }
        Some(())
    }
}
