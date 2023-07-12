use crate::workspace::{
    AutoSearchParams, FileFeaturesResult, GetFileContentParams, IsPathIgnoredParams,
    OrganizeImportsParams, OrganizeImportsResult, PathExistsParams, RageParams, RageResult,
    ServerInfo,
};
use crate::{TransportError, Workspace, WorkspaceError};
use rome_formatter::Printed;
use rome_fs::AutoSearchResult;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicU64, Ordering},
};

use super::{
    ChangeFileParams, CloseFileParams, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameParams, RenameResult,
    SupportsFeatureParams, UpdateSettingsParams,
};

pub struct WorkspaceClient<T> {
    transport: T,
    request_id: AtomicU64,
    server_info: Option<ServerInfo>,
}

pub trait WorkspaceTransport {
    fn request<P, R>(&self, request: TransportRequest<P>) -> Result<R, TransportError>
    where
        P: Serialize,
        R: DeserializeOwned;
}

#[derive(Debug)]
pub struct TransportRequest<P> {
    pub id: u64,
    pub method: &'static str,
    pub params: P,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// Information about the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
}

impl<T> WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    pub fn new(transport: T) -> Result<Self, WorkspaceError> {
        let mut client = Self {
            transport,
            request_id: AtomicU64::new(0),
            server_info: None,
        };

        // TODO: The current implementation of the JSON-RPC protocol in
        // tower_lsp doesn't allow any request to be sent before a call to
        // initialize, this is something we could be able to lift by using our
        // own RPC protocol implementation
        let value: InitializeResult = client.request_with_payload(
            "initialize",
            json!({
                "capabilities": {},
                "clientInfo": {
                    "name": env!("CARGO_PKG_NAME"),
                    "version": crate::VERSION
                },
            }),
        )?;

        client.server_info = value.server_info;

        Ok(client)
    }

    fn request_with_payload<P, R>(
        &self,
        method: &'static str,
        params: P,
    ) -> Result<R, WorkspaceError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = TransportRequest { id, method, params };

        let response = self.transport.request(request)?;

        Ok(response)
    }

    fn request<R>(&self, method: &'static str) -> Result<R, WorkspaceError>
    where
        R: DeserializeOwned,
    {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = TransportRequest {
            id,
            method,
            params: (),
        };

        let response = self.transport.request(request)?;

        Ok(response)
    }

    pub fn shutdown(self) -> Result<(), WorkspaceError> {
        self.request_with_payload("rome/shutdown", ())
    }
}

impl<T> Workspace for WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        self.request_with_payload("rome/file_features", params)
    }

    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError> {
        self.request_with_payload("rome/is_path_ignored", params)
    }

    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError> {
        self.request_with_payload("rome/update_settings", params)
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError> {
        self.request_with_payload("rome/open_file", params)
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        self.request_with_payload("rome/get_syntax_tree", params)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError> {
        self.request_with_payload("rome/get_control_flow_graph", params)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError> {
        self.request_with_payload("rome/get_formatter_ir", params)
    }

    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError> {
        self.request_with_payload("rome/get_file_content", params)
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError> {
        self.request_with_payload("rome/change_file", params)
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError> {
        self.request_with_payload("rome/close_file", params)
    }

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.request_with_payload("rome/pull_diagnostics", params)
    }

    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError> {
        self.request_with_payload("rome/pull_actions", params)
    }

    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError> {
        self.request_with_payload("rome/format_file", params)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError> {
        self.request_with_payload("rome/format_range", params)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError> {
        self.request_with_payload("rome/format_on_type", params)
    }

    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError> {
        self.request_with_payload("rome/fix_file", params)
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError> {
        self.request_with_payload("rome/rename", params)
    }

    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError> {
        self.request_with_payload("rome/rage", params)
    }

    fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    fn organize_imports(
        &self,
        params: OrganizeImportsParams,
    ) -> Result<OrganizeImportsResult, WorkspaceError> {
        self.request_with_payload("rome/organize_imports", params)
    }

    fn config_name(&self) -> Result<String, WorkspaceError> {
        self.request("rome/config_name")
    }

    fn path_exists(&self, params: PathExistsParams) -> Result<bool, WorkspaceError> {
        self.request_with_payload("rome/path_exists", params)
    }

    fn auto_search(
        &self,
        params: AutoSearchParams,
    ) -> Result<Option<AutoSearchResult>, WorkspaceError> {
        self.request_with_payload("rome/auto_search", params)
    }

    fn working_directory(&self) -> Result<Option<PathBuf>, WorkspaceError> {
        self.request("rome/working_directory")
    }
}
