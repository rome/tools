import { RequestType, TextDocumentIdentifier } from "vscode-languageclient";

export interface SyntaxTreeParams {
	textDocument: TextDocumentIdentifier;
}

/**
 * Request to send to the server when showing the syntax tree of a document
 */
export const syntaxTreeRequest = new RequestType<
	SyntaxTreeParams,
	string,
	void
>("rome_lsp/syntaxTree");

// Empty parameters
export interface ConfigurationChangeParams {}
/**
 * Request to send to the server when the configuration file changed
 */
export const configChangeRequest = new RequestType<
	ConfigurationChangeParams,
	string,
	void
>("rome_lsp/configurationUpdated");
