use rome_formatter::{IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use salsa::query_group;

use crate::{database::Parser, settings::SettingsHandle, RomeError};

#[query_group(FormatterStorage)]
pub(crate) trait Formatter: Parser {
    fn format(&self, name: RomePath, indent_style: IndentStyle) -> Result<Printed, RomeError>;
    fn format_range(
        &self,
        name: RomePath,
        indent_style: IndentStyle,
        range: TextRange,
    ) -> Result<Printed, RomeError>;
    fn format_on_type(
        &self,
        name: RomePath,
        indent_style: IndentStyle,
        offset: TextSize,
    ) -> Result<Printed, RomeError>;
}

fn format(
    db: &dyn Formatter,
    name: RomePath,
    indent_style: IndentStyle,
) -> Result<Printed, RomeError> {
    let features = db.language_features(());
    let settings = db.settings(());

    let capabilities = features.get_capabilities(&name);
    let formatter = capabilities
        .format
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    if !settings.format.format_with_errors && db.has_errors(name.clone()) {
        return Err(RomeError::FormatWithErrorsDisabled);
    }

    let settings = SettingsHandle::new(db, indent_style);
    formatter(&name, parse, settings)
}

fn format_range(
    db: &dyn Formatter,
    name: RomePath,
    indent_style: IndentStyle,
    range: TextRange,
) -> Result<Printed, RomeError> {
    let features = db.language_features(());
    let settings = db.settings(());

    let capabilities = features.get_capabilities(&name);
    let formatter = capabilities
        .format_range
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    if !settings.format.format_with_errors && db.has_errors(name.clone()) {
        return Err(RomeError::FormatWithErrorsDisabled);
    }

    let settings = SettingsHandle::new(db, indent_style);
    formatter(&name, parse, settings, range)
}

fn format_on_type(
    db: &dyn Formatter,
    name: RomePath,
    indent_style: IndentStyle,
    offset: TextSize,
) -> Result<Printed, RomeError> {
    let features = db.language_features(());
    let settings = db.settings(());

    let capabilities = features.get_capabilities(&name);
    let format_on_type = capabilities
        .format_on_type
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    if !settings.format.format_with_errors && db.has_errors(name.clone()) {
        return Err(RomeError::FormatWithErrorsDisabled);
    }

    let settings = SettingsHandle::new(db, indent_style);
    format_on_type(&name, parse, settings, offset)
}
