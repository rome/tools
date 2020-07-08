import { RequestType } from 'vscode-jsonrpc';
import { TextDocumentIdentifier } from 'vscode-languageserver-types';
import { TextDocumentRegistrationOptions, StaticRegistrationOptions } from './protocol';
export interface FoldingRangeClientCapabilities {
    /**
     * The text document client capabilities
     */
    textDocument?: {
        /**
         * Capabilities specific to `textDocument/foldingRange` requests
         */
        foldingRange?: {
            /**
             * Whether implementation supports dynamic registration for folding range providers. If this is set to `true`
             * the client supports the new `(FoldingRangeProviderOptions & TextDocumentRegistrationOptions & StaticRegistrationOptions)`
             * return value for the corresponding server capability as well.
             */
            dynamicRegistration?: boolean;
            /**
             * The maximum number of folding ranges that the client prefers to receive per document. The value serves as a
             * hint, servers are free to follow the limit.
             */
            rangeLimit?: number;
            /**
             * If set, the client signals that it only supports folding complete lines. If set, client will
             * ignore specified `startCharacter` and `endCharacter` properties in a FoldingRange.
             */
            lineFoldingOnly?: boolean;
        };
    };
}
export interface FoldingRangeProviderOptions {
}
export interface FoldingRangeServerCapabilities {
    /**
     * The server provides folding provider support.
     */
    foldingRangeProvider?: boolean | FoldingRangeProviderOptions | (FoldingRangeProviderOptions & TextDocumentRegistrationOptions & StaticRegistrationOptions);
}
/**
 * Enum of known range kinds
 */
export declare enum FoldingRangeKind {
    /**
     * Folding range for a comment
     */
    Comment = "comment",
    /**
     * Folding range for a imports or includes
     */
    Imports = "imports",
    /**
     * Folding range for a region (e.g. `#region`)
     */
    Region = "region"
}
/**
 * Represents a folding range.
 */
export interface FoldingRange {
    /**
     * The zero-based line number from where the folded range starts.
     */
    startLine: number;
    /**
     * The zero-based character offset from where the folded range starts. If not defined, defaults to the length of the start line.
     */
    startCharacter?: number;
    /**
     * The zero-based line number where the folded range ends.
     */
    endLine: number;
    /**
     * The zero-based character offset before the folded range ends. If not defined, defaults to the length of the end line.
     */
    endCharacter?: number;
    /**
     * Describes the kind of the folding range such as `comment' or 'region'. The kind
     * is used to categorize folding ranges and used by commands like 'Fold all comments'. See
     * [FoldingRangeKind](#FoldingRangeKind) for an enumeration of standardized kinds.
     */
    kind?: string;
}
/**
 * Parameters for a [FoldingRangeRequest](#FoldingRangeRequest).
 */
export interface FoldingRangeParams {
    /**
     * The text document.
     */
    textDocument: TextDocumentIdentifier;
}
/**
 * A request to provide folding ranges in a document. The request's
 * parameter is of type [FoldingRangeParams](#FoldingRangeParams), the
 * response is of type [FoldingRangeList](#FoldingRangeList) or a Thenable
 * that resolves to such.
 */
export declare namespace FoldingRangeRequest {
    const type: RequestType<FoldingRangeParams, FoldingRange[] | null, any, any>;
}
