import { RequestType, RequestHandler } from 'vscode-jsonrpc';
import { Definition, DefinitionLink, LocationLink, Location } from 'vscode-languageserver-types';
import { TextDocumentRegistrationOptions, StaticRegistrationOptions, TextDocumentPositionParams } from './protocol';
export interface TypeDefinitionClientCapabilities {
    /**
     * The text document client capabilities
     */
    textDocument?: {
        /**
         * Capabilities specific to the `textDocument/typeDefinition`
         */
        typeDefinition?: {
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
export interface TypeDefinitionServerCapabilities {
    /**
     * The server provides Goto Type Definition support.
     */
    typeDefinitionProvider?: boolean | (TextDocumentRegistrationOptions & StaticRegistrationOptions);
}
/**
 * A request to resolve the type definition locations of a symbol at a given text
 * document position. The request's parameter is of type [TextDocumentPositioParams]
 * (#TextDocumentPositionParams) the response is of type [Definition](#Definition) or a
 * Thenable that resolves to such.
 */
export declare namespace TypeDefinitionRequest {
    const type: RequestType<TextDocumentPositionParams, Location | Location[] | LocationLink[] | null, void, TextDocumentRegistrationOptions>;
    type HandlerSignature = RequestHandler<TextDocumentPositionParams, Definition | DefinitionLink[] | null, void>;
}
