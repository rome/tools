import { RequestType, RequestHandler } from 'vscode-jsonrpc';
import { Definition, DefinitionLink, Location, LocationLink } from 'vscode-languageserver-types';
import { TextDocumentRegistrationOptions, StaticRegistrationOptions, TextDocumentPositionParams } from './protocol';
export interface ImplementationClientCapabilities {
    /**
     * The text document client capabilities
     */
    textDocument?: {
        /**
         * Capabilities specific to the `textDocument/implementation`
         */
        implementation?: {
            /**
             * Whether implementation supports dynamic registration. If this is set to `true`
             * the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
             * return value for the corresponding server capability as well.
             */
            dynamicRegistration?: boolean;
            /**
             * The client supports additional metadata in the form of definition links.
             */
            linkSupport?: boolean;
        };
    };
}
export interface ImplementationServerCapabilities {
    /**
     * The server provides Goto Implementation support.
     */
    implementationProvider?: boolean | (TextDocumentRegistrationOptions & StaticRegistrationOptions);
}
/**
 * A request to resolve the implementation locations of a symbol at a given text
 * document position. The request's parameter is of type [TextDocumentPositioParams]
 * (#TextDocumentPositionParams) the response is of type [Definition](#Definition) or a
 * Thenable that resolves to such.
 */
export declare namespace ImplementationRequest {
    const type: RequestType<TextDocumentPositionParams, Location | Location[] | LocationLink[] | null, void, TextDocumentRegistrationOptions>;
    type HandlerSignature = RequestHandler<TextDocumentPositionParams, Definition | DefinitionLink[] | null, void>;
}
