import { RequestType, RequestHandler, HandlerResult, CancellationToken } from 'vscode-jsonrpc';
export interface ConfigurationClientCapabilities {
    /**
     * The workspace client capabilities
     */
    workspace?: {
        /**
        * The client supports `workspace/configuration` requests.
        */
        configuration?: boolean;
    };
}
/**
 * The 'workspace/configuration' request is sent from the server to the client to fetch a certain
 * configuration setting.
 *
 * This pull model replaces the old push model were the client signaled configuration change via an
 * event. If the server still needs to react to configuration changes (since the server caches the
 * result of `workspace/configuration` requests) the server should register for an empty configuration
 * change event and empty the cache if such an event is received.
 */
export declare namespace ConfigurationRequest {
    const type: RequestType<ConfigurationParams, any[], void, void>;
    type HandlerSignature = RequestHandler<ConfigurationParams, any[], void>;
    type MiddlewareSignature = (params: ConfigurationParams, token: CancellationToken, next: HandlerSignature) => HandlerResult<any[], void>;
}
export interface ConfigurationItem {
    /**
     * The scope to get the configuration section for.
     */
    scopeUri?: string;
    /**
     * The configuration section asked for.
     */
    section?: string;
}
/**
 * The parameters of a configuration request.
 */
export interface ConfigurationParams {
    items: ConfigurationItem[];
}
