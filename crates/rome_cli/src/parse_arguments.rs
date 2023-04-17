use crate::{CliDiagnostic, CliSession};
use rome_formatter::IndentStyle;
use rome_service::configuration::vcs::{VcsClientKind, VcsConfiguration};
use rome_service::configuration::{
    FormatterConfiguration, JavascriptConfiguration, JavascriptFormatter, PlainIndentStyle,
};
use rome_service::Configuration;

/// Read the formatting options for the command line arguments and inject them
/// into the workspace settings
pub(crate) fn apply_format_settings_from_cli(
    session: &mut CliSession,
    configuration: &mut Configuration,
) -> Result<(), CliDiagnostic> {
    let formatter = configuration
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .map_err(|source| CliDiagnostic::parse_error("--indent-size", source))?;

    let indent_style = session
        .args
        .opt_value_from_str("--indent-style")
        .map_err(|source| CliDiagnostic::parse_error("--indent-style", source))?;

    let line_width = session
        .args
        .opt_value_from_str("--line-width")
        .map_err(|source| CliDiagnostic::parse_error("--line-width", source))?;

    match indent_style {
        Some(IndentStyle::Tab) => {
            formatter.indent_style = PlainIndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            formatter.indent_style = PlainIndentStyle::Space;
            formatter.indent_size = size.unwrap_or(default_size);
        }
        None => {}
    }

    if let Some(line_width) = line_width {
        formatter.line_width = line_width;
    }

    let quote_properties = session
        .args
        .opt_value_from_str("--quote-properties")
        .map_err(|source| CliDiagnostic::parse_error("--quote-properties", source))?;

    let quote_style = session
        .args
        .opt_value_from_str("--quote-style")
        .map_err(|source| CliDiagnostic::parse_error("--quote-style", source))?;

    let trailing_comma = session
        .args
        .opt_value_from_str("--trailing-comma")
        .map_err(|source| CliDiagnostic::parse_error("--trailing-comma", source))?;

    let semicolons = session
        .args
        .opt_value_from_str("--semicolons")
        .map_err(|source| CliDiagnostic::parse_error("--semicolons", source))?;

    let javascript = configuration
        .javascript
        .get_or_insert_with(JavascriptConfiguration::default);
    let javascript_formatter = javascript
        .formatter
        .get_or_insert_with(JavascriptFormatter::default);

    if let Some(quote_properties) = quote_properties {
        javascript_formatter.quote_properties = quote_properties;
    }

    if let Some(quote_style) = quote_style {
        javascript_formatter.quote_style = quote_style;
    }

    if let Some(trailing_comma) = trailing_comma {
        javascript_formatter.trailing_comma = trailing_comma;
    }

    if let Some(semicolons) = semicolons {
        javascript_formatter.semicolons = semicolons;
    }

    Ok(())
}

pub(crate) fn apply_files_settings_from_cli(
    session: &mut CliSession,
    configuration: &mut Configuration,
) -> Result<(), CliDiagnostic> {
    let files_max_size = session
        .args
        .opt_value_from_str("--files-max-size")
        .map_err(|source| CliDiagnostic::parse_error("--files-max-size", source))?;

    if let Some(files_max_size) = files_max_size {
        let files = configuration.files.get_or_insert_with(Default::default);
        files.max_size = Some(files_max_size);
    }

    Ok(())
}

pub(crate) fn apply_vcs_settings_from_cli(
    session: &mut CliSession,
    configuration: &mut Configuration,
) -> Result<(), CliDiagnostic> {
    let vcs = configuration
        .vcs
        .get_or_insert_with(VcsConfiguration::default);

    let enabled = session
        .args
        .opt_value_from_str("--vcs-enabled")
        .map_err(|source| CliDiagnostic::parse_error("--vcs-enabled", source))?;
    let client_kind = session
        .args
        .opt_value_from_str("--vcs-client-kind")
        .map_err(|source| CliDiagnostic::parse_error("--vcs-client-kind", source))?;

    let use_ignore_file = session
        .args
        .opt_value_from_str("--vcs-use-ignore-file")
        .map_err(|source| CliDiagnostic::parse_error("--vcs-use-ignore-file", source))?;
    let root = session
        .args
        .opt_value_from_str("--vcs-root")
        .map_err(|source| CliDiagnostic::parse_error("--vcs-root", source))?;

    if let Some(enabled) = enabled {
        vcs.enabled = enabled;
    }

    match client_kind {
        None => {}
        Some(VcsClientKind::Git) => {
            vcs.client_kind = Some(VcsClientKind::Git);
        }
    }

    vcs.use_ignore_file = use_ignore_file;
    vcs.root = root;

    Ok(())
}
