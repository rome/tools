use rome_formatter::IndentStyle;
use rome_js_formatter::options::JsFormatOptions;

use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let options = parse_format_options(&mut session)?;

    let is_write = session.args.contains("--write");
    let ignore_errors = session.args.contains("--skip-errors");

    traverse(
        TraversalMode::Format {
            options,
            ignore_errors,
            write: is_write,
        },
        session,
    )
}

pub(crate) fn parse_format_options(
    session: &mut CliSession,
) -> Result<JsFormatOptions, Termination> {
    let mut options = JsFormatOptions::default();

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-size",
            source,
        })?;

    let indent_style = session
        .args
        .opt_value_from_str("--indent-style")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-style",
            source,
        })?;

    match indent_style {
        Some(IndentStyle::Tab) => {
            options.indent_style = IndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
        }
        None => {}
    }

    let quote_style = session
        .args
        .opt_value_from_str("--quote-style")
        .map_err(|source| Termination::ParseError {
            argument: "--quote-style",
            source,
        })?;

    if let Some(quote_style) = quote_style {
        options.quote_style = quote_style;
    }

    let line_width = session
        .args
        .opt_value_from_str("--line-width")
        .map_err(|source| Termination::ParseError {
            argument: "--line-width",
            source,
        })?;

    if let Some(line_width) = line_width {
        options.line_width = line_width;
    }

    Ok(options)
}
