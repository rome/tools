import { commands, TextDocument, TextEditor } from "vscode";

const SUPPORTED_LANGUAGES = new Set(["javascript", "typescript"]);

export type RomeDocument = TextDocument & {
	languageId: keyof typeof SUPPORTED_LANGUAGES;
};
export type RomeEditor = TextEditor & { document: RomeDocument };

/** Sets ['when'](https://code.visualstudio.com/docs/getstarted/keybindings#_when-clause-contexts) clause contexts */
export function setContextValue(key: string, value: any): Thenable<void> {
	return commands.executeCommand("setContext", key, value);
}

/**
 * Checks if the current document is supported by Rome
 *
 * @param {TextDocument} document
 */
export function isRomeDocument(document: TextDocument) {
	return SUPPORTED_LANGUAGES.has(document.languageId);
}

export function isRomeEditor(editor: TextEditor): editor is RomeEditor {
	return isRomeDocument(editor.document);
}
