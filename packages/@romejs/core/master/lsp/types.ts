import {Number0} from "@romejs/ob1";
import {JSONPropertyValue} from "@romejs/codec-json";

/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
export type LSPRequestMessage = {
	/**
   * The request id.
   */
	id: number | string;

	/**
   * The method to be invoked.
   */
	method: string;

	/**
   * The method's params.
   */
	params?: unknown;
};

export type LSPResponseMessage = {
	/**
   * The request id.
   */
	id: number | string | null;

	/**
   * The result of a request. This member is REQUIRED on success.
   * This member MUST NOT exist if there was an error invoking the method.
   */
	result?: JSONPropertyValue;

	/**
   * The error object in case a request fails.
   */
	error?: LSPResponseError;
};

export type LSPResponseError = {
	/**
   * A number indicating the error type that occurred.
   */
	code: LSPErrorCodes[keyof LSPErrorCodes];

	/**
   * A string providing a short description of the error.
   */
	message: string;

	/**
   * A primitive or structured value that contains additional
   * information about the error. Can be omitted.
   */
	data?: JSONPropertyValue;
};

export type LSPNotificationMessage = {
	/**
   * The method to be invoked.
   */
	method: string;

	/**
   * The notification's params.
   */
	params?: JSONPropertyValue;
};

export type LSPErrorCodes = {
	// Defined by JSON RPC
	ParseError: -32700;
	InvalidRequest: -32600;
	MethodNotFound: -32601;
	InvalidParams: -32602;
	InternalError: -32603;
	serverErrorStart: -32099;
	serverErrorEnd: -32000;
	ServerNotInitialized: -32002;
	UnknownErrorCode: -32001;

	// Defined by the protocol.
	RequestCancelled: -32800;
	ContentModified: -32801;
};

export type LSPPosition = {
	/**
   * Line position in a document (zero-based).
   */
	line: Number0;

	/**
   * Character offset on a line in a document (zero-based). Assuming that the line is
   * represented as a string, the `character` value represents the gap between the
   * `character` and `character + 1`.
   *
   * If the character value is greater than the line length it defaults back to the
   * line length.
   */
	character: Number0;
};

export type LSPRange = {
	/**
   * The range's start position.
   */
	start: LSPPosition;

	/**
   * The range's end position.
   */
	end: LSPPosition;
};

export type LSPDocumentUri = string;

export type LSPLocation = {
	uri: LSPDocumentUri;
	range: LSPRange;
};

export type LSPLocationLink = {
	/**
   * Span of the origin of this link.
   *
   * Used as the underlined span for mouse interaction. Defaults to the word range at
   * the mouse position.
   */
	originSelectionRange?: LSPRange;

	/**
   * The target resource identifier of this link.
   */
	targetUri: LSPDocumentUri;

	/**
   * The full target range of this link. If the target for example is a symbol then target range is the
   * range enclosing this symbol not including leading/trailing whitespace but everything else
   * like comments. This information is typically used to highlight the range in the editor.
   */
	targetRange: LSPRange;

	/**
   * The range that should be selected and revealed when this link is being followed, e.g the name of a function.
   * Must be contained by the the `targetRange`. See also `DocumentSymbol#range`
   */
	targetSelectionRange: LSPRange;
};

export type LSPDiagnostic = {
	/**
   * The range at which the message applies.
   */
	range: LSPRange;

	/**
   * The diagnostic's severity. Can be omitted. If omitted it is up to the
   * client to interpret diagnostics as error, warning, info or hint.
   */
	severity?: LSPDiagnosticSeverity[keyof LSPDiagnosticSeverity];

	/**
   * The diagnostic's code, which might appear in the user interface.
   */
	code?: number | string;

	/**
   * A human-readable string describing the source of this
   * diagnostic, e.g. 'typescript' or 'super lint'.
   */
	source?: string;

	/**
   * The diagnostic's message.
   */
	message: string;

	/**
   * Additional metadata about the diagnostic.
   *
   * @since 3.15.0
   */
	tags?: Array<LSPDiagnosticTag[keyof LSPDiagnosticTag]>;

	/**
   * An array of related diagnostic information, e.g. when symbol-names within
   * a scope collide all definitions can be marked via this property.
   */
	relatedInformation?: Array<LSPDiagnosticRelatedInformation>;
};

export type LSPDiagnosticSeverity = {
	/**
   * Reports an error.
   */
	Error: 1;
	/**
   * Reports a warning.
   */
	Warning: 2;
	/**
   * Reports an information.
   */
	Information: 3;
	/**
   * Reports a hint.
   */
	Hint: 4;
};

/**
 * The diagnostic tags.
 *
 * @since 3.15.0
 */
export type LSPDiagnosticTag = {
	/**
   * Unused or unnecessary code.
   *
   * Clients are allowed to render diagnostics with this tag faded out instead of having
   * an error squiggle.
   */
	Unnecessary: 1;
	/**
   * Deprecated or obsolete code.
   *
   * Clients are allowed to rendered diagnostics with this tag strike through.
   */
	Deprecated: 2;
};

/**
 * Represents a related message and source code location for a diagnostic. This should be
 * used to point to code locations that cause or are related to a diagnostics, e.g when duplicating
 * a symbol in a scope.
 */
export type LSPDiagnosticRelatedInformation = {
	/**
   * The location of this related diagnostic information.
   */
	location: LSPLocation;

	/**
   * The message of this related diagnostic information.
   */
	message: string;
};

export type LSPTextEdit = {
	/**
   * The range of the text document to be manipulated. To insert
   * text into a document create a range where start === end.
   */
	range: LSPRange;

	/**
   * The string to be inserted. For delete operations use an
   * empty string.
   */
	newText: string;
};
