import { StaticFeature, BaseLanguageClient } from './client';
import { ClientCapabilities, ConfigurationRequest } from 'vscode-languageserver-protocol';
export interface ConfigurationWorkspaceMiddleware {
    configuration?: ConfigurationRequest.MiddlewareSignature;
}
export declare class ConfigurationFeature implements StaticFeature {
    private _client;
    constructor(_client: BaseLanguageClient);
    fillClientCapabilities(capabilities: ClientCapabilities): void;
    initialize(): void;
    private getConfiguration;
}
