/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0} from "@internal/ob1";
import {JSONArray, JSONObject, JSONPropertyValue} from "@internal/codec-json";

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
	params?: JSONArray | JSONObject;
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

export type LSPCommand = {
	/**
	 * Title of the command, like `save`.
	 */
	title: string;
	/**
	 * The identifier of the actual command handler.
	 */
	command: string;
	/**
	 * Arguments that the command handler should be
	 * invoked with.
	 */
	arguments?: Array<string>;
};

export type LSPCodeActionKind = string;

export type LSPCodeAction = {
	/**
	 * A short, human-readable, title for this code action.
	 */
	title: string;

	/**
	 * The kind of the code action.
	 *
	 * Used to filter code actions.
	 */
	kind?: LSPCodeActionKind;

	/**
	 * The diagnostics that this code action resolves.
	 */
	diagnostics?: Array<LSPDiagnostic>;

	/**
	 * Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can be targeted
	 * by keybindings.
	 *
	 * A quick fix should be marked preferred if it properly addresses the underlying error.
	 * A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
	 *
	 * @since 3.15.0
	 */
	isPreferred?: boolean;

	/**
	 * The workspace edit this code action performs.
	 */
	edit?: LSPWorkspaceEdit;

	/**
	 * A command this code action executes. If a code action
	 * provides an edit and a command, first the edit is
	 * executed and then the command.
	 */
	command?: LSPCommand;
};

export type LSPWorkspaceEdit = {
	/**
	 * Holds changes to existing resources.
	 */
	changes?: {
		[uri: string]: Array<LSPTextEdit>;
	};

	/**
	 * Depending on the client capability `workspace.workspaceEdit.resourceOperations` document changes
	 * are either an array of `TextDocumentEdit`s to express changes to n different text documents
	 * where each text document edit addresses a specific version of a text document. Or it can contain
	 * above `TextDocumentEdit`s mixed with create, rename and delete file / folder operations.
	 *
	 * Whether a client supports versioned document edits is expressed via
	 * `workspace.workspaceEdit.documentChanges` client capability.
	 *
	 * If a client neither supports `documentChanges` nor `workspace.workspaceEdit.resourceOperations` then
	 * only plain `TextEdit`s using the `changes` property are supported.
	 */
	documentChanges?: Array<LSPTextDocumentEdit>;
};

export type LSPTextDocumentEdit = {
	/**
	 * The text document to change.
	 */
	textDocument: LSPVersionedTextDocumentIdentifier;

	/**
	 * The edits to be applied.
	 */
	edits: Array<LSPTextEdit>;
};

export type LSPTextDocumentIdentifier = {
	/**
	 * The text document's URI.
	 */
	uri: LSPDocumentUri;
};

export type LSPVersionedTextDocumentIdentifier = LSPTextDocumentIdentifier & {
	/**
	 * The version number of this document. If a versioned text document identifier
	 * is sent from the server to the client and the file is not open in the editor
	 * (the server has not received an open notification before) the server can send
	 * `null` to indicate that the version is known and the content on disk is the
	 * master (as speced with document content ownership).
	 *
	 * The version number of a document will increase after each change, including
	 * undo/redo. The number doesn't need to be consecutive.
	 */
	version: number | null;
};
