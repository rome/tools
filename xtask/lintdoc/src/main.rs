use pulldown_cmark::{html::write_html, CodeBlockKind, Event, LinkType, Parser, Tag};
use rome_analyze::{
    AnalysisFilter, AnalyzerOptions, ControlFlow, GroupCategory, Queryable, RegistryVisitor, Rule,
    RuleCategory, RuleFilter, RuleGroup, RuleMetadata,
};
use rome_console::{
    fmt::{Formatter, HTML},
    markup, Console, Markup,
};
use rome_diagnostics::file::FileId;
use rome_diagnostics::v2::{Diagnostic, DiagnosticExt, PrintDiagnostic};
use rome_js_analyze::{analyze, visit_registry};
use rome_js_syntax::{JsLanguage, Language, LanguageVariant, ModuleKind, SourceType};
use rome_service::settings::WorkspaceSettings;
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    io::{self, Write as _},
    path::Path,
    slice,
    str::{self, FromStr},
};
use xtask::{glue::fs2, *};

fn main() -> Result<()> {
    let root = project_root().join("website/src/docs/lint/rules");

    // Clear the rules directory ignoring "not found" errors
    if let Err(err) = fs2::remove_dir_all(&root) {
        let is_not_found = err
            .source()
            .and_then(|err| err.downcast_ref::<io::Error>())
            .map_or(false, |err| matches!(err.kind(), io::ErrorKind::NotFound));

        if !is_not_found {
            return Err(err);
        }
    }

    fs2::create_dir_all(&root)?;

    // Content of the index page
    let mut index = Vec::new();
    writeln!(index, "---")?;
    writeln!(index, "title: Lint Rules")?;
    writeln!(index, "layout: layouts/docs.liquid")?;
    writeln!(index, "main-class: rules")?;
    writeln!(index, "---")?;
    writeln!(index)?;

    writeln!(index, "# Rules")?;
    writeln!(index)?;

    // Accumulate errors for all lint rules to print all outstanding issues on
    // failure instead of just the first one
    let mut errors = Vec::new();

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

    let mut visitor = LintRulesVisitor::default();
    visit_registry(&mut visitor);

    let LintRulesVisitor { mut groups } = visitor;

    let nursery_rules = groups
        .remove("nursery")
        .expect("Expected nursery group to exist");

    for (group, rules) in groups {
        generate_group(group, rules, &root, &mut index, &mut errors)?;
    }

    generate_group("nursery", nursery_rules, &root, &mut index, &mut errors)?;

    if !errors.is_empty() {
        bail!(
            "failed to generate documentation pages for the following rules:\n{}",
            errors
                .into_iter()
                .map(|(rule, err)| format!("- {rule}: {err:?}\n"))
                .collect::<String>()
        );
    }

    fs2::write(root.join("index.md"), index)?;

    Ok(())
}

fn generate_group(
    group: &'static str,
    rules: BTreeMap<&'static str, RuleMetadata>,
    root: &Path,
    mut index: &mut dyn io::Write,
    errors: &mut Vec<(&'static str, Error)>,
) -> io::Result<()> {
    let (group_name, description) = match group {
        "a11y" => (
            "Accessibility",
            markup! {
                "Rules focused on preventing accessibility problems."
            },
        ),
        "correctness" => (
            "Correctness",
            markup! {
                "Rules that detect incorrect or useless code."
            },
        ),

        "nursery" => (
            "Nursery",
            markup! {
                "New rules that are still under development.

Nursery rules require explicit opt-in via configuration because they may still have bugs or performance problems.
Nursery rules get promoted to other groups once they become stable or may be removed.

Rules that belong to this group "<Emphasis>"are not subject to semantic version"</Emphasis>"."
            },
        ),
        "style" => (
            "Style",
            markup! {
                "Rules enforcing a consistent way of writing your code. "
            },
        ),
        "complexity" => (
            "Complexity",
            markup! {
                "Rules that focus on inspecting complex code that could be simplified."
            },
        ),
        "security" => (
            "Security",
            markup! {
                "Rules that detect potential security flaws."
            },
        ),
        _ => panic!("Unknown group ID {group:?}"),
    };

    writeln!(index, "\n## {group_name}")?;
    writeln!(index)?;
    markup_to_string(index, description)?;
    writeln!(index)?;

    writeln!(index, "<div class=\"category-rules\">")?;
    for (rule, meta) in rules {
        match generate_rule(root, group, rule, meta.docs, meta.version, meta.recommended) {
            Ok(summary) => {
                writeln!(index, "<section class=\"rule\">")?;
                writeln!(index, "<h3 data-toc-exclude id=\"{rule}\">")?;
                writeln!(index, "	<a href=\"/docs/lint/rules/{rule}\">{rule}</a>")?;

                if meta.recommended {
                    writeln!(index, "	<span class=\"recommended\">recommended</span>")?;
                }
                writeln!(index, "</h3>")?;

                write_html(&mut index, summary.into_iter())?;

                writeln!(index, "\n</section>")?;
            }
            Err(err) => {
                errors.push((rule, err));
            }
        }
    }
    writeln!(index, "\n</div>")?;

    Ok(())
}

/// Generates the documentation page for a single lint rule
fn generate_rule(
    root: &Path,
    group: &'static str,
    rule: &'static str,
    docs: &'static str,
    version: &'static str,
    recommended: bool,
) -> Result<Vec<Event<'static>>> {
    let mut content = Vec::new();

    // Write the header for this lint rule
    writeln!(content, "---")?;
    writeln!(content, "title: Lint Rule {rule}")?;
    writeln!(content, "layout: layouts/docs.liquid")?;
    writeln!(content, "---")?;
    writeln!(content)?;

    writeln!(content, "# {rule} (since v{version})")?;
    writeln!(content)?;

    if recommended {
        writeln!(content, "> This rule is recommended by Rome.")?;
        writeln!(content)?;
    }

    let summary = parse_documentation(group, rule, docs, &mut content)?;

    fs2::write(root.join(format!("{rule}.md")), content)?;

    Ok(summary)
}

/// Parse the documentation fragment for a lint rule (in markdown) and generates
/// the content for the corresponding documentation page
fn parse_documentation(
    group: &'static str,
    rule: &'static str,
    docs: &'static str,
    content: &mut Vec<u8>,
) -> Result<Vec<Event<'static>>> {
    let parser = Parser::new(docs);

    // Parser events for the first paragraph of documentation in the resulting
    // content, used as a short summary of what the rule does in the rules page
    let mut summary = Vec::new();
    let mut is_summary = false;

    // Tracks the content of the current code block if it's using a
    // language supported for analysis
    let mut language = None;
    let mut list_order = None;
    for event in parser {
        if is_summary {
            if matches!(event, Event::End(Tag::Paragraph)) {
                is_summary = false;
            } else {
                summary.push(event.clone());
            }
        }

        match event {
            // CodeBlock-specific handling
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(meta))) => {
                // Track the content of code blocks to pass them through the analyzer
                let test = CodeBlockTest::from_str(meta.as_ref())?;

                // Erase the lintdoc-specific attributes in the output by
                // re-generating the language ID from the source type
                write!(content, "```")?;
                if !meta.is_empty() {
                    match test.source_type.language() {
                        Language::JavaScript => write!(content, "js")?,
                        Language::TypeScript { .. } => write!(content, "ts")?,
                    }
                    match test.source_type.variant() {
                        LanguageVariant::Standard => {}
                        LanguageVariant::Jsx => write!(content, "x")?,
                    }
                }
                writeln!(content)?;

                language = Some((test, String::new()));
            }

            Event::End(Tag::CodeBlock(_)) => {
                writeln!(content, "```")?;
                writeln!(content)?;

                if let Some((test, block)) = language.take() {
                    if test.expect_diagnostic {
                        write!(
                            content,
                            "{{% raw %}}<pre class=\"language-text\"><code class=\"language-text\">"
                        )?;
                    }

                    assert_lint(group, rule, &test, &block, content)
                        .context("snapshot test failed")?;

                    if test.expect_diagnostic {
                        writeln!(content, "</code></pre>{{% endraw %}}")?;
                        writeln!(content)?;
                    }
                }
            }

            Event::Text(text) => {
                if let Some((_, block)) = &mut language {
                    write!(block, "{text}")?;
                }

                write!(content, "{text}")?;
            }

            // Other markdown events are emitted as-is
            Event::Start(Tag::Heading(level, ..)) => {
                write!(content, "{} ", "#".repeat(level as usize))?;
            }
            Event::End(Tag::Heading(..)) => {
                writeln!(content)?;
                writeln!(content)?;
            }

            Event::Start(Tag::Paragraph) => {
                if summary.is_empty() && !is_summary {
                    is_summary = true;
                }
            }
            Event::End(Tag::Paragraph) => {
                writeln!(content)?;
                writeln!(content)?;
            }

            Event::Code(text) => {
                write!(content, "`{text}`")?;
            }

            Event::Start(Tag::Link(kind, _, _)) => match kind {
                LinkType::Inline => {
                    write!(content, "[")?;
                }
                LinkType::Shortcut => {
                    write!(content, "[")?;
                }
                _ => {
                    panic!("unimplemented link type")
                }
            },
            Event::End(Tag::Link(_, url, title)) => {
                write!(content, "]({url}")?;
                if !title.is_empty() {
                    write!(content, " \"{title}\"")?;
                }
                write!(content, ")")?;
            }

            Event::SoftBreak => {
                writeln!(content)?;
            }

            Event::Start(Tag::List(num)) => {
                if let Some(num) = num {
                    list_order = Some(num);
                }
            }

            Event::End(Tag::List(_)) => {
                list_order = None;
                writeln!(content)?;
            }
            Event::Start(Tag::Item) => {
                if let Some(num) = list_order {
                    write!(content, "{num}. ")?;
                } else {
                    write!(content, "- ")?;
                }
            }

            Event::End(Tag::Item) => {
                list_order = list_order.map(|item| item + 1);
                writeln!(content)?;
            }

            Event::Start(Tag::Strong) => {
                write!(content, "**")?;
            }

            Event::End(Tag::Strong) => {
                write!(content, "**")?;
            }

            Event::Start(Tag::Emphasis) => {
                write!(content, "_")?;
            }

            Event::End(Tag::Emphasis) => {
                write!(content, "_")?;
            }

            Event::Start(Tag::Strikethrough) => {
                write!(content, "~")?;
            }

            Event::End(Tag::Strikethrough) => {
                write!(content, "~")?;
            }

            Event::Start(Tag::BlockQuote) => {
                write!(content, ">")?;
            }

            Event::End(Tag::BlockQuote) => {
                writeln!(content)?;
            }

            _ => {
                // TODO: Implement remaining events as required
                bail!("unimplemented event {event:?}")
            }
        }
    }

    Ok(summary)
}

struct CodeBlockTest {
    source_type: SourceType,
    expect_diagnostic: bool,
}

impl FromStr for CodeBlockTest {
    type Err = xtask::Error;

    fn from_str(input: &str) -> Result<Self> {
        // This is based on the parsing logic for code block languages in `rustdoc`:
        // https://github.com/rust-lang/rust/blob/6ac8adad1f7d733b5b97d1df4e7f96e73a46db42/src/librustdoc/html/markdown.rs#L873
        let tokens = input
            .split(|c| c == ',' || c == ' ' || c == '\t')
            .map(str::trim)
            .filter(|token| !token.is_empty());

        let mut test = CodeBlockTest {
            source_type: SourceType::default(),
            expect_diagnostic: false,
        };

        for token in tokens {
            match token {
                // Determine the language, using the same list of extensions as `compute_source_type_from_path_or_extension`
                "cjs" => {
                    test.source_type = SourceType::js_module().with_module_kind(ModuleKind::Script);
                }
                "js" | "mjs" | "jsx" => {
                    test.source_type = SourceType::jsx();
                }
                "ts" | "mts" => {
                    test.source_type = SourceType::ts();
                }
                "cts" => {
                    test.source_type = SourceType::ts().with_module_kind(ModuleKind::Script);
                }
                "tsx" => {
                    test.source_type = SourceType::tsx();
                }

                // Other attributes
                "expect_diagnostic" => {
                    test.expect_diagnostic = true;
                }

                _ => {
                    bail!("unknown code block attribute {token:?}")
                }
            }
        }

        Ok(test)
    }
}

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `expect_diagnostic`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    group: &'static str,
    rule: &'static str,
    test: &CodeBlockTest,
    code: &str,
    content: &mut Vec<u8>,
) -> Result<()> {
    let file = format!("{group}/{rule}.js");

    let mut write = HTML(content);
    let mut diagnostic_count = 0;

    let mut all_diagnostics = vec![];

    let mut write_diagnostic = |code: &str, diag: rome_diagnostics::v2::Error| {
        let category = diag.category().map_or("", |code| code.name());
        Formatter::new(&mut write).write_markup(markup! {
            {PrintDiagnostic(&diag)}
        })?;

        all_diagnostics.push(diag);
        // Fail the test if the analysis returns more diagnostics than expected
        if test.expect_diagnostic {
            // Print all diagnostics to help the user
            if all_diagnostics.len() > 1 {
                let mut console = rome_console::EnvConsole::new(false);
                for diag in all_diagnostics.iter() {
                    console.print(
                        rome_console::LogLevel::Error,
                        markup! {
                            {PrintDiagnostic(diag)}
                        },
                    );
                }
            }

            ensure!(
                diagnostic_count == 0,
                "analysis returned multiple diagnostics, code snippet: \n\n{}",
                code
            );
        } else {
            // Print all diagnostics to help the user
            let mut console = rome_console::EnvConsole::new(false);
            for diag in all_diagnostics.iter() {
                console.print(
                    rome_console::LogLevel::Error,
                    markup! {
                        {PrintDiagnostic(diag)}
                    },
                );
            }

            bail!(format!(
                "analysis returned an unexpected diagnostic, code `snippet:\n\n{:?}\n\n{}",
                category, code
            ));
        }

        diagnostic_count += 1;
        Ok(())
    };

    let parse = rome_js_parser::parse(code, FileId::zero(), test.source_type);

    if parse.has_errors() {
        for diag in parse.into_diagnostics() {
            let error = diag
                .with_file_path((file.clone(), FileId::zero()))
                .with_file_source_code(code);
            write_diagnostic(code, error)?;
        }
    } else {
        let root = parse.tree();

        let settings = WorkspaceSettings::default();

        let rule_filter = RuleFilter::Rule(group, rule);
        let filter = AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        };

        let options = AnalyzerOptions::default();
        let result = analyze(FileId::zero(), &root, filter, &options, |signal| {
            if let Some(mut diag) = signal.diagnostic() {
                let category = diag.category().expect("linter diagnostic has no code");
                let severity = settings.get_severity_from_rule_code(category).expect(
                    "If you see this error, it means you need to run cargo codegen-configuration",
                );
                diag.set_severity(severity);

                if let Some(action) = signal.action() {
                    diag.add_code_suggestion(action.into());
                }

                let error = diag
                    .with_file_path((file.clone(), FileId::zero()))
                    .with_file_source_code(code);
                let res = write_diagnostic(code, error);

                // Abort the analysis on error
                if let Err(err) = res {
                    return ControlFlow::Break(err);
                }
            }

            ControlFlow::Continue(())
        });

        // Result is Some(_) if analysis aborted with an error
        if let Some(err) = result {
            return Err(err);
        }
    }

    if test.expect_diagnostic {
        // Fail the test if the analysis didn't emit any diagnostic
        ensure!(
            diagnostic_count == 1,
            "analysis returned no diagnostics.\n code snippet:\n {}",
            code
        );
    }

    Ok(())
}

pub fn markup_to_string(buffer: &mut dyn io::Write, markup: Markup) -> io::Result<()> {
    let mut write = HTML(buffer);
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup)
}
