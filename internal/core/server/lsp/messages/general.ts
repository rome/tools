import {createAbsoluteFilePath} from "@internal/path";
import {safeProcessExit} from "@internal/resources";
import {getPathFromTextDocument} from "../utils";
import {LSPNotificationHandler, LSPRequestHandler} from "./types";

export const shutdown: LSPRequestHandler = async (lsp) => {
	await lsp.shutdown();
	return null;
};

export const exit: LSPNotificationHandler = async () => {
	await safeProcessExit(0);
};

export const initialized: LSPNotificationHandler = async (lsp) => {
	await lsp.watchPendingProjects();
};

export const initialize: LSPRequestHandler = async (lsp, params) => {
	const rootUri = params.get("rootUri");
	if (rootUri.exists()) {
		const workspacePath = createAbsoluteFilePath(rootUri.asString());
		await lsp.initProject(workspacePath);
	}

	const workspaceFolders = params.get("workspaceFolders");
	if (workspaceFolders.exists()) {
		for (const elem of workspaceFolders.asIterable()) {
			await lsp.initProject(getPathFromTextDocument(elem));
		}
	}

	return {
		capabilities: {
			textDocumentSync: {
				openClose: true,
				// This sends over incremental patches on change
				change: 2,
			},
			documentFormattingProvider: true,
			codeActionProvider: true,
			executeCommandProvider: {
				commands: ["rome.check.decisions", "rome.check.apply"],
			},
			workspace: {
				workspaceFolders: {
					supported: true,
					changeNotifications: true,
				},
			},
		},
		serverInfo: {
			name: "rome",
		},
	};
};
