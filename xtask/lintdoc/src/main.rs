use std::{
    fmt::Write as _,
    io::{self, Write as _},
    path::Path,
    slice,
    str::{self, FromStr},
};

use pulldown_cmark::{html::write_html, CodeBlockKind, Event, LinkType, Parser, Tag};
use rome_console::{
    fmt::{Formatter, HTML},
    markup,
};
use rome_diagnostics::{file::SimpleFile, Diagnostic};
use xtask::{glue::fs2, *};

use rome_analyze::{AnalysisFilter, ControlFlow, RuleCategories};
use rome_js_analyze::{analyze, metadata};
use rome_js_syntax::{Language, LanguageVariant, ModuleKind, SourceType};

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
    writeln!(index, "layout: layouts/page.liquid")?;
    writeln!(index, "layout-type: split")?;
    writeln!(index, "main-class: rules")?;
    writeln!(index, "eleventyNavigation:")?;
    writeln!(index, "  key: lint-rules")?;
    writeln!(index, "  parent: linting")?;
    writeln!(index, "  title: Rules")?;
    writeln!(index, "---")?;
    writeln!(index)?;

    writeln!(index, "# Rules")?;
    writeln!(index)?;

    writeln!(index, "<section>")?;
    // TODO: Update this when rule groups are implemented
    writeln!(index, "<h2>JavaScript</h2>")?;

    // Accumulate errors for all lint rules to print all outstanding issues on
    // failure instead of just the first one
    let mut errors = Vec::new();

    let filter = AnalysisFilter {
        categories: RuleCategories::LINT,
        ..AnalysisFilter::default()
    };

    for (name, docs) in metadata(filter) {
        match generate_rule(&root, name, docs) {
            Ok(summary) => {
                writeln!(index, "<div class=\"rule\">")?;
                writeln!(index, "<h3 data-toc-exclude id=\"{name}\">")?;
                writeln!(index, "	<a href=\"/docs/lint/rules/{name}\">{name}</a>")?;
                writeln!(index, "	<a class=\"header-anchor\" href=\"#{name}\"></a>")?;
                writeln!(index, "</h3>")?;

                write_html(&mut index, summary.into_iter())?;

                writeln!(index, "\n</div>")?;
            }
            Err(err) => {
                errors.push((name, err));
            }
        }
    }

    if !errors.is_empty() {
        bail!(
            "failed to generate documentation pages for the following rules:\n{}",
            errors
                .into_iter()
                .map(|(rule, err)| format!("- {rule}: {err:?}\n"))
                .collect::<String>()
        );
    }

    writeln!(index, "</section>")?;

    fs2::write(root.join("index.md"), index)?;

    Ok(())
}

/// Generates the documentation page for a single lint rule
fn generate_rule(
    root: &Path,
    name: &'static str,
    docs: &'static str,
) -> Result<Vec<Event<'static>>> {
    let mut content = Vec::new();

    // Write the header for this lint rule
    writeln!(content, "---")?;
    writeln!(content, "title: Lint Rule {name}")?;
    writeln!(content, "layout: layouts/rule.liquid")?;
    writeln!(content, "---")?;
    writeln!(content)?;

    writeln!(content, "# {name}")?;
    writeln!(content)?;

    let summary = parse_documentation(name, docs, &mut content)?;

    fs2::write(root.join(format!("{name}.md")), content)?;

    Ok(summary)
}

/// Parse the documentation fragment for a lint rule (in markdown) and generates
/// the content for the corresponding documentation page
fn parse_documentation(
    name: &'static str,
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

                    assert_lint(name, &test, &block, content).context("snapshot test failed")?;

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

            Event::Start(Tag::Link(kind, _, _)) => {
                assert_eq!(kind, LinkType::Inline, "unimplemented link type");
                write!(content, "[")?;
            }
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
    name: &'static str,
    test: &CodeBlockTest,
    code: &str,
    content: &mut Vec<u8>,
) -> Result<()> {
    let file = SimpleFile::new(format!("{name}.js"), code.into());

    let mut write = HTML(content);
    let mut diagnostic_count = 0;

    let mut write_diagnostic = |diag: Diagnostic| {
        // Fail the test if the analysis returns more diagnostics than expected
        if test.expect_diagnostic {
            ensure!(
                diagnostic_count == 0,
                "analysis returned multiple diagnostics"
            );
        } else {
            bail!("analysis returned an unexpected diagnostic");
        }

        Formatter::new(&mut write).write_markup(markup! {
            {diag.display(&file)}
        })?;

        diagnostic_count += 1;
        Ok(())
    };

    let parse = rome_js_parser::parse(code, 0, test.source_type);

    if parse.has_errors() {
        for diag in parse.into_diagnostics() {
            write_diagnostic(diag)?;
        }
    } else {
        let root = parse.tree();
        let filter = AnalysisFilter {
            rules: Some(slice::from_ref(&name)),
            ..AnalysisFilter::default()
        };

        let result = analyze(0, &root, filter, |signal| {
            if let Some(mut diag) = signal.diagnostic() {
                if let Some(action) = signal.action() {
                    diag.suggestions.push(action.into());
                }

                let res = write_diagnostic(diag);

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
        ensure!(diagnostic_count == 1, "analysis returned no diagnostics");
    }

    Ok(())
}
