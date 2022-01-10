import { ExtensionContext, workspace } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(_context: ExtensionContext) {
	const command = workspace.getConfiguration().get("rome.lspBin") as string;

	const serverOptions: ServerOptions =
		{ command, transport: TransportKind.stdio };

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: 'file', language: 'javascript' }, { scheme: 'file', language: 'typescript' }],
	};

	client = new LanguageClient(
		'rome_lsp',
		'Language Server Rome',
		serverOptions,
		clientOptions
	);

	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}