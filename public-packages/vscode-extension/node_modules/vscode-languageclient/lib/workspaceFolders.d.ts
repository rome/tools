import { WorkspaceFoldersChangeEvent as VWorkspaceFoldersChangeEvent } from 'vscode';
import { DynamicFeature, RegistrationData, BaseLanguageClient, NextSignature } from './client';
import { ClientCapabilities, InitializeParams, RPCMessageType, ServerCapabilities, WorkspaceFoldersRequest } from 'vscode-languageserver-protocol';
export interface WorkspaceFolderWorkspaceMiddleware {
    workspaceFolders?: WorkspaceFoldersRequest.MiddlewareSignature;
    didChangeWorkspaceFolders?: NextSignature<VWorkspaceFoldersChangeEvent, void>;
}
export declare class WorkspaceFoldersFeature implements DynamicFeature<undefined> {
    private _client;
    private _listeners;
    constructor(_client: BaseLanguageClient);
    readonly messages: RPCMessageType;
    fillInitializeParams(params: InitializeParams): void;
    fillClientCapabilities(capabilities: ClientCapabilities): void;
    initialize(capabilities: ServerCapabilities): void;
    register(_message: RPCMessageType, data: RegistrationData<undefined>): void;
    unregister(id: string): void;
    dispose(): void;
    private asProtocol;
}
