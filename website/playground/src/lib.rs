#![allow(clippy::unused_unit)] // Bug in wasm_bindgen creates unused unit warnings. See wasm_bindgen#2774

use std::fmt::Display;

use js_sys::Error;
use rome_console::fmt::HTML;
use rome_diagnostics::file::SimpleFile;
use wasm_bindgen::prelude::*;

use rome_console::{fmt::Formatter, markup};
use rome_diagnostics::Diagnostic;
use rome_service::workspace::{
    self, ChangeFileParams, CloseFileParams, FixFileParams, FormatFileParams, FormatOnTypeParams,
    FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams, GetSyntaxTreeParams,
    PullActionsParams, PullDiagnosticsParams, RenameParams, UpdateSettingsParams,
};
use rome_service::workspace::{OpenFileParams, SupportsFeatureParams};

include!(concat!(env!("OUT_DIR"), "/ts_types.rs"));

#[wasm_bindgen]
pub struct Workspace {
    inner: Box<dyn workspace::Workspace>,
}

#[wasm_bindgen]
impl Workspace {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Workspace {
        Workspace {
            inner: workspace::server(),
        }
    }

    pub fn supports_feature(&self, params: ISupportsFeatureParams) -> Result<bool, Error> {
        let params: SupportsFeatureParams = params.into_serde().map_err(into_error)?;
        Ok(self.inner.supports_feature(params))
    }

    pub fn update_settings(&self, params: IUpdateSettingsParams) -> Result<(), Error> {
        let params: UpdateSettingsParams = params.into_serde().map_err(into_error)?;
        self.inner.update_settings(params).map_err(into_error)
    }

    pub fn open_file(&self, params: IOpenFileParams) -> Result<(), Error> {
        let params: OpenFileParams = params.into_serde().map_err(into_error)?;
        self.inner.open_file(params).map_err(into_error)
    }

    pub fn get_syntax_tree(
        &self,
        params: IGetSyntaxTreeParams,
    ) -> Result<IGetSyntaxTreeResult, Error> {
        let params: GetSyntaxTreeParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.get_syntax_tree(params).map_err(into_error)?;
        JsValue::from_serde(&result)
            .map(IGetSyntaxTreeResult::from)
            .map_err(into_error)
    }

    pub fn get_control_flow_graph(
        &self,
        params: IGetControlFlowGraphParams,
    ) -> Result<String, Error> {
        let params: GetControlFlowGraphParams = params.into_serde().map_err(into_error)?;
        self.inner
            .get_control_flow_graph(params)
            .map_err(into_error)
    }

    pub fn get_formatter_ir(&self, params: IGetFormatterIRParams) -> Result<String, Error> {
        let params: GetFormatterIRParams = params.into_serde().map_err(into_error)?;
        self.inner.get_formatter_ir(params).map_err(into_error)
    }

    pub fn change_file(&self, params: IChangeFileParams) -> Result<(), Error> {
        let params: ChangeFileParams = params.into_serde().map_err(into_error)?;
        self.inner.change_file(params).map_err(into_error)
    }

    pub fn close_file(&self, params: ICloseFileParams) -> Result<(), Error> {
        let params: CloseFileParams = params.into_serde().map_err(into_error)?;
        self.inner.close_file(params).map_err(into_error)
    }

    pub fn pull_diagnostics(
        &self,
        params: IPullDiagnosticsParams,
    ) -> Result<IPullDiagnosticsResult, Error> {
        let params: PullDiagnosticsParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.pull_diagnostics(params).map_err(into_error)?;
        JsValue::from_serde(&result)
            .map(IPullDiagnosticsResult::from)
            .map_err(into_error)
    }

    pub fn pull_actions(&self, params: IPullActionsParams) -> Result<IPullActionsResult, Error> {
        let params: PullActionsParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.pull_actions(params).map_err(into_error)?;
        JsValue::from_serde(&result)
            .map(IPullActionsResult::from)
            .map_err(into_error)
    }

    pub fn format_file(&self, params: IFormatFileParams) -> Result<JsValue, Error> {
        let params: FormatFileParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.format_file(params).map_err(into_error)?;
        JsValue::from_serde(&result).map_err(into_error)
    }

    pub fn format_range(&self, params: IFormatRangeParams) -> Result<JsValue, Error> {
        let params: FormatRangeParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.format_range(params).map_err(into_error)?;
        JsValue::from_serde(&result).map_err(into_error)
    }

    pub fn format_on_type(&self, params: IFormatOnTypeParams) -> Result<JsValue, Error> {
        let params: FormatOnTypeParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.format_on_type(params).map_err(into_error)?;
        JsValue::from_serde(&result).map_err(into_error)
    }

    pub fn fix_file(&self, params: IFixFileParams) -> Result<IFixFileResult, Error> {
        let params: FixFileParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.fix_file(params).map_err(into_error)?;
        JsValue::from_serde(&result)
            .map(IFixFileResult::from)
            .map_err(into_error)
    }

    pub fn rename(&self, params: IRenameParams) -> Result<IRenameResult, Error> {
        let params: RenameParams = params.into_serde().map_err(into_error)?;
        let result = self.inner.rename(params).map_err(into_error)?;
        JsValue::from_serde(&result)
            .map(IRenameResult::from)
            .map_err(into_error)
    }
}

#[wasm_bindgen]
pub struct DiagnosticPrinter {
    file: SimpleFile,
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl DiagnosticPrinter {
    #[wasm_bindgen(constructor)]
    pub fn new(file_name: String, file_source: String) -> Self {
        Self {
            file: SimpleFile::new(file_name, file_source),
            buffer: Vec::new(),
        }
    }

    pub fn print(&mut self, diagnostic: IDiagnostic) -> Result<(), Error> {
        let diag: Diagnostic = diagnostic.into_serde().map_err(into_error)?;

        let mut html = HTML(&mut self.buffer);
        Formatter::new(&mut html)
            .write_markup(markup!({ diag.display(&self.file) }))
            .map_err(into_error)?;

        Ok(())
    }

    pub fn finish(self) -> Result<String, Error> {
        String::from_utf8(self.buffer).map_err(into_error)
    }
}

fn into_error<E: Display>(err: E) -> Error {
    Error::new(&err.to_string())
}
