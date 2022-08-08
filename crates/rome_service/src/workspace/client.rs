use std::{
    io,
    panic::RefUnwindSafe,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use rome_formatter::Printed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_slice, json, to_vec};

use crate::{RomeError, Workspace};

use super::{
    ChangeFileParams, CloseFileParams, FixFileParams, FixFileResult, FormatFileParams,
    FormatOnTypeParams, FormatRangeParams, GetControlFlowGraphParams, GetFormatterIRParams,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, PullActionsParams, PullActionsResult,
    PullDiagnosticsParams, PullDiagnosticsResult, RenameParams, RenameResult,
    SupportsFeatureParams, UpdateSettingsParams,
};

pub(super) struct WorkspaceClient<T> {
    transport: Mutex<T>,
    request_id: AtomicU64,
}

pub trait WorkspaceTransport {
    fn send(&mut self, request: Vec<u8>) -> Result<(), RomeError>;
    fn receive(&mut self) -> Result<Vec<u8>, RomeError>;
}

#[derive(Serialize)]
struct JsonRpcRequest<P> {
    jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    method: &'static str,
    params: P,
}

#[derive(Deserialize)]
struct JsonRpcResponse<'a, R> {
    #[allow(dead_code)]
    jsonrpc: &'a str,
    id: u64,
    #[serde(flatten)]
    status: JsonRpcResult<R>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum JsonRpcResult<R> {
    Ok { result: R },
    Err { error: JsonRpcError },
}

#[derive(Deserialize)]
struct JsonRpcError {
    #[allow(dead_code)]
    code: i64,
    message: String,
    data: Option<RomeError>,
}

#[derive(Deserialize)]
struct InitializeResult {}

impl<T> WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    pub(super) fn new(transport: T) -> Result<Self, RomeError> {
        let client = Self {
            transport: Mutex::new(transport),
            request_id: AtomicU64::new(0),
        };

        // TODO: The current implementation of the JSON-RPC protocol in
        // tower_lsp doesn't allow any request to be sent before a call to
        // initialize, this is something we could be able to lift by using our
        // own RPC protocol implementation
        let _value: InitializeResult = client.request(
            "initialize",
            json!({
                "capabilities": {},
                "client_info": {
                    "name": "rome_service",
                    "version": env!("CARGO_PKG_VERSION")
                },
            }),
        )?;

        Ok(client)
    }

    fn request<P, R>(&self, method: &'static str, params: P) -> Result<R, RomeError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let mut transport = self.transport.lock().unwrap();

        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = JsonRpcRequest {
            jsonrpc: "2.0",
            id: Some(id),
            method,
            params,
        };

        let request = to_vec(&request)?;
        transport.send(request)?;

        let response = transport.receive()?;
        let response: JsonRpcResponse<R> = from_slice(&response)?;

        // This should be true since we don't allow concurrent requests yet
        assert_eq!(response.id, id);

        match response.status {
            JsonRpcResult::Ok { result } => Ok(result),
            JsonRpcResult::Err { error } => match error.data {
                Some(error) => Err(error),
                None => Err(RomeError::IoError(io::Error::new(
                    io::ErrorKind::Other,
                    error.message,
                ))),
            },
        }
    }
}

impl<T> Workspace for WorkspaceClient<T>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync,
{
    fn supports_feature(&self, params: SupportsFeatureParams) -> bool {
        self.request("rome/supports_feature", params)
            .unwrap_or(false)
    }

    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), RomeError> {
        self.request("rome/update_settings", params)
    }

    fn open_file(&self, params: OpenFileParams) -> Result<(), RomeError> {
        self.request("rome/open_file", params)
    }

    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, RomeError> {
        self.request("rome/get_syntax_tree", params)
    }

    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, RomeError> {
        self.request("rome/get_control_flow_graph", params)
    }

    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, RomeError> {
        self.request("rome/get_formatter_ir", params)
    }

    fn change_file(&self, params: ChangeFileParams) -> Result<(), RomeError> {
        self.request("rome/change_file", params)
    }

    fn close_file(&self, params: CloseFileParams) -> Result<(), RomeError> {
        self.request("rome/close_file", params)
    }

    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, RomeError> {
        self.request("rome/pull_diagnostics", params)
    }

    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, RomeError> {
        self.request("rome/pull_actions", params)
    }

    fn format_file(&self, params: FormatFileParams) -> Result<Printed, RomeError> {
        self.request("rome/format_file", params)
    }

    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, RomeError> {
        self.request("rome/format_range", params)
    }

    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, RomeError> {
        self.request("rome/format_on_type", params)
    }

    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, RomeError> {
        self.request("rome/fix_file", params)
    }

    fn rename(&self, params: RenameParams) -> Result<RenameResult, RomeError> {
        self.request("rome/rename", params)
    }
}
