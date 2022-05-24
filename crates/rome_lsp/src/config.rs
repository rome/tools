use rome_formatter::{IndentStyle, LineWidth};
use rome_js_formatter::options::QuoteStyle;
use rome_service::settings;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tracing::{info, trace};

pub const CONFIGURATION_SECTION: &str = "rome";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Specific settings for Rome formatter
pub struct FormatterWorkspaceSettings {
    /// Allows to format code that might contain syntax errors
    pub format_with_syntax_errors: bool,
    /// The width of a single line, specified by the user
    pub line_width: u16,
    /// The indent style, specified by the user
    pub indent_style: String,
    /// The quote style, specified by the user
    pub quote_style: String,
    /// The number of spaces, specified by the user and applied only when using Spaces
    pub space_quantity: u8,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Settings for Rome Analysis
pub struct AnalysisWorkspaceSettings {
    /// Allows rome to compute and publish diagnostics
    pub enable_diagnostics: bool,
    /// Allows rome to compute and provide code actions
    pub enable_code_actions: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// The settings applied to the workspace by the LSP
pub struct WorkspaceSettings {
    /// Formatter settings
    #[serde(default)]
    pub formatter: FormatterWorkspaceSettings,

    /// Analysis settings
    #[serde(default)]
    pub analysis: AnalysisWorkspaceSettings,

    /// Unstable features enabled
    #[serde(default)]
    pub unstable: bool,
}

#[derive(Debug)]
pub(crate) struct Config {
    settings: WorkspaceSettings,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            settings: WorkspaceSettings::default(),
        }
    }

    pub fn get_workspace_settings(&self) -> WorkspaceSettings {
        self.settings.clone()
    }

    pub fn set_workspace_settings(&mut self, value: Value) -> Result<(), Error> {
        let workspace_settings = serde_json::from_value(value)?;
        self.settings = workspace_settings;
        trace!(
            "Correctly stored the settings coming from the client: {:?}",
            self.settings
        );
        Ok(())
    }

    /// Convert the current configuration to an instance of [settings::WorkspaceSettings]
    pub fn as_workspace_settings(&self) -> settings::WorkspaceSettings {
        let mut settings = settings::WorkspaceSettings::default();

        settings.format.format_with_errors = self.settings.formatter.format_with_syntax_errors;

        let custom_ident_style: IndentStyle = self
            .settings
            .formatter
            .indent_style
            .parse()
            .unwrap_or_default();

        if custom_ident_style != IndentStyle::default() {
            // merge settings with the ones provided by the editor
            match custom_ident_style {
                IndentStyle::Space(_) => {
                    settings.format.indent_style =
                        Some(IndentStyle::Space(self.settings.formatter.space_quantity));
                }
                IndentStyle::Tab => {
                    settings.format.indent_style = Some(custom_ident_style);
                }
            }

            info!(
                "Using user setting indent style: {:?}",
                settings.format.indent_style
            );
        }

        let custom_quote_style: QuoteStyle = self
            .settings
            .formatter
            .quote_style
            .parse()
            .unwrap_or_default();

        if custom_quote_style != QuoteStyle::default() {
            settings.languages.javascript.format.quote_style = Some(custom_quote_style);
            info!("Using user setting quote style: {}", custom_quote_style);
        }

        // apply the new line width only if they are different
        let custom_line_width: LineWidth = self
            .settings
            .formatter
            .line_width
            .try_into()
            .unwrap_or_default();

        if custom_line_width != LineWidth::default() {
            settings.format.line_width = Some(custom_line_width);
            info!(
                "Using user setting line width: {}",
                custom_line_width.value()
            );
        }

        settings
    }
}
