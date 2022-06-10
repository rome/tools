use std::{fmt::Write as _, io::Write as _, path::Path, slice};

use pulldown_cmark::{escape::escape_html, CodeBlockKind, Event, LinkType, Options, Parser, Tag};
use rome_console::{
    fmt::{Formatter, HTML},
    markup,
};
use rome_diagnostics::{file::SimpleFile, Diagnostic};
use xtask::{glue::fs2, *};

use rome_analyze::{AnalysisFilter, ControlFlow};
use rome_js_analyze::{analyze, metadata};
use rome_js_syntax::SourceType;

fn main() -> Result<()> {
    let root = project_root().join("website/src/docs/lint/rules");

    // Clear the rules directory
    fs2::remove_dir_all(&root)?;
    fs2::create_dir_all(&root)?;

    // Content of the index page
    let mut index = String::new();
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

    for (name, docs) in metadata() {
        match generate_rule(&root, name, docs) {
            Ok(summary) => {
                writeln!(index, "<div class=\"rule\">")?;
                writeln!(index, "<h3 data-toc-exclude id=\"{name}\">")?;
                writeln!(index, "	<a href=\"/docs/lint/rules/{name}\">{name}</a>")?;
                writeln!(index, "	<a class=\"header-anchor\" href=\"#{name}\"></a>")?;
                writeln!(index, "</h3>")?;
                escape_html(&mut index, &summary)?;
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
fn generate_rule(root: &Path, name: &'static str, docs: &'static str) -> Result<String> {
    let mut content = Vec::new();

    // Write the header for this lint rule
    writeln!(content, "---")?;
    writeln!(content, "title: Lint Rule {name}")?;
    writeln!(content, "layout: layouts/rule.liquid")?;
    writeln!(content, "---")?;
    writeln!(content)?;

    writeln!(content, "# {name}")?;
    writeln!(content)?;

    let summary = if !docs.is_empty() {
        parse_documentation(name, docs, &mut content)?
    } else {
        // Default content if the rule has no documentation
        writeln!(content, "MISSING DOCUMENTATION")?;
        String::from("MISSING DOCUMENTATION")
    };

    fs2::write(root.join(format!("{name}.md")), content)?;

    Ok(summary)
}

/// Parse the documentation fragment for a lint rule (in markdown) and generates
/// the content for the corresponding documentation page
fn parse_documentation(
    name: &'static str,
    docs: &'static str,
    content: &mut Vec<u8>,
) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(docs, options);

    // Content of the first paragraph of documentation, used as a short summary
    // of what the rule does in the rules index
    let mut summary = String::new();
    let mut is_summary = true;

    // Tracks the content of the current code block if it's using a
    // language supported for analysis
    let mut language = None;

    // Tracks whether the current section of documentation is expected to
    // contain failing or passing tests
    let mut section = SectionKind::None;

    enum SectionKind {
        None,
        Invalid,
        Valid,
    }

    for event in parser {
        match event {
            // CodeBlock-specific handling
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(meta))) => {
                // If this is a `valid` or `invalid` section, track the content
                // of code blocks to pass them through the analyzer
                if !matches!(section, SectionKind::None) {
                    match meta.as_ref() {
                        "js" | "javascript" => {
                            language = Some((SourceType::js_module(), String::new()));
                        }
                        "jsx" => {
                            language = Some((SourceType::jsx(), String::new()));
                        }

                        // TODO: Should all language names be explicitly
                        // supported, of silently ignore unknown languages ?
                        other => bail!("unsupported code block language {other:?}"),
                    }
                }

                writeln!(content, "```{meta}")?;
            }

            Event::End(Tag::CodeBlock(_)) => {
                writeln!(content, "```")?;
                writeln!(content)?;

                if let Some((source_type, block)) = language.take() {
                    let should_fail = matches!(section, SectionKind::Invalid);

                    if should_fail {
                        write!(
                            content,
                            "{{% raw %}}<pre class=\"language-text\"><code class=\"language-text\">"
                        )?;
                    }

                    assert_lint(name, source_type, &block, should_fail, content)
                        .context("snapshot test failed")?;

                    if should_fail {
                        writeln!(content, "</code></pre>{{% endraw %}}")?;
                        writeln!(content)?;
                    }
                }
            }

            Event::Text(text) => {
                if is_summary {
                    write!(summary, "{text}")?;
                }

                if let Some((_, block)) = &mut language {
                    write!(block, "{text}")?;
                }

                write!(content, "{text}")?;
            }

            // Other markdown events are emitted as-is
            Event::Start(Tag::Heading(level, fragment, _)) => {
                write!(content, "{} ", "#".repeat(level as usize))?;

                match fragment {
                    Some("valid") => {
                        section = SectionKind::Valid;
                    }
                    Some("invalid") => {
                        section = SectionKind::Invalid;
                    }
                    _ => {}
                }
            }
            Event::End(Tag::Heading(..)) => {
                writeln!(content)?;
                writeln!(content)?;
            }

            Event::Start(Tag::Paragraph) => {}
            Event::End(Tag::Paragraph) => {
                // Stop the summary at the first paragraph end
                is_summary = false;

                writeln!(content)?;
                writeln!(content)?;
            }

            Event::Code(text) => {
                write!(content, "`{text}`")?;

                if is_summary {
                    write!(summary, "`{text}`")?;
                }
            }

            Event::Start(Tag::Link(kind, _, _)) => {
                assert_eq!(kind, LinkType::Inline, "unimplemented link type");
                write!(content, "[")?;

                if is_summary {
                    write!(summary, "[")?;
                }
            }
            Event::End(Tag::Link(_, url, title)) => {
                write!(content, "]({url}")?;
                if !title.is_empty() {
                    write!(content, " \"{title}\"")?;
                }
                write!(content, ")")?;

                if is_summary {
                    write!(summary, "]({url}")?;
                    if !title.is_empty() {
                        write!(summary, " \"{title}\"")?;
                    }
                    write!(summary, ")")?;
                }
            }

            Event::SoftBreak => {
                if is_summary {
                    writeln!(summary)?;
                }

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

/// Parse and analyze the provided code block, and asserts that it emits
/// exactly zero or one diagnostic depending on the value of `should_fail`.
/// That diagnostic is then emitted as text into the `content` buffer
fn assert_lint(
    name: &'static str,
    source_type: SourceType,
    code: &str,
    should_fail: bool,
    content: &mut Vec<u8>,
) -> Result<()> {
    let file = SimpleFile::new(format!("{name}.js"), code.into());

    // TODO: Emit markup as HTML
    let mut write = HTML(content);
    let mut diagnostic_count = 0;

    let mut write_diagnostic = |diag: Diagnostic| {
        // Fail the test if the analysis returns more diagnostics than expected
        if should_fail {
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

    let parse = rome_js_parser::parse(code, 0, source_type);

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

    if should_fail {
        // Fail the test if the analysis didn't emit any diagnostic
        ensure!(diagnostic_count == 1, "analysis returned no diagnostics");
    }

    Ok(())
}
