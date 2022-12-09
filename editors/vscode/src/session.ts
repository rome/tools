import { ExtensionContext, commands, Disposable, window } from "vscode";
import { Commands } from "./commands";
import { LanguageClient } from "vscode-languageclient/node";
import { isRomeEditor, RomeEditor } from "./utils";

export type Command = (...args: unknown[]) => unknown;

/**
 * Client session of the LSP
 */
export class Session {
	context: ExtensionContext;
	client: LanguageClient;

	constructor(context: ExtensionContext, client: LanguageClient) {
		this.context = context;
		this.client = client;
	}

	registerCommand(name: Commands, factory: Command) {
		const disposable = commands.registerCommand(name, factory);
		this.context.subscriptions.push(disposable);
	}

	get subscriptions(): Disposable[] {
		return this.context.subscriptions;
	}

	get editor(): RomeEditor | undefined {
		const editor = window.activeTextEditor;
		return editor && isRomeEditor(editor) ? editor : undefined;
	}
}
