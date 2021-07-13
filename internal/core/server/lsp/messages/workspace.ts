import {PartialServerQueryRequest} from "@internal/core/common/bridges/ServerBridge";
import {createAbsoluteFilePath} from "@internal/path";
import {diffTextEdits, getPathFromTextDocument} from "../utils";
import {LSPNotificationHandler, LSPRequestHandler} from "./types";

export const didChangeWorkspaceFolders: LSPNotificationHandler = async (
	lsp,
	params,
) => {
	const event = params.get("event");
	for (const elem of event.get("added").asIterable()) {
		await lsp.initProject(getPathFromTextDocument(elem));
	}
	for (const elem of event.get("removed").asIterable()) {
		lsp.unwatchProject(getPathFromTextDocument(elem));
	}
	await lsp.watchPendingProjects();
};

export const executeCommand: LSPRequestHandler = async (lsp, params) => {
	const command = params.get("command").asString();
	const filename = params.get("arguments").getIndex(0).asString();

	const path = createAbsoluteFilePath(filename);
	const startVersion = lsp.fileVersions.get(path);

	let req: PartialServerQueryRequest | undefined;

	if (command === "rome.check.apply") {
		req = {
			commandName: "check",
			args: [filename],
			commandFlags: {apply: true},
			noFileWrites: true,
		};
	}

	if (command === "rome.check.decisions") {
		const decisions = params.get("arguments").getIndex(1).asString();
		req = {
			commandName: "check",
			args: [filename],
			commandFlags: {decisions, suppressionExplanation: "suppressed via editor"},
			noFileWrites: true,
		};
	}

	if (req === undefined) {
		return null;
	}

	const response = await lsp.sendClientRequest(req);

	if (response.type === "SUCCESS" || response.type === "DIAGNOSTICS") {
		const original = await lsp.request.requestWorkerGetBuffer(path);
		const saveFile = response.files[filename];
		if (original === undefined || saveFile === undefined) {
			return null;
		}
		const endVersion = lsp.fileVersions.get(path);
		if (startVersion !== endVersion) {
			lsp.logMessage(path, `Can't update ${filename} because it was modified,`);
			return null;
		}

		const edits = diffTextEdits(original, saveFile.content);

		await lsp.transport.request({
			method: "workspace/applyEdit",
			params: {
				label: "Rome Action",
				edit: {
					documentChanges: [
						{
							textDocument: {
								uri: `file://${filename}`,
								version: endVersion,
							},
							edits,
						},
					],
				},
			},
		});
	}

	return null;
};
