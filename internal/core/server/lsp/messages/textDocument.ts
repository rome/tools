import {
	catchDiagnostics,
	formatCategoryDescription,
	getActionAdviceFromDiagnostic,
} from "@internal/diagnostics";
import {readMarkup} from "@internal/markup";
import {LSPCodeAction} from "../types";
import {
	convertDiagnosticLocationToLSPRange,
	diffTextEdits,
	doRangesOverlap,
	getDecisionFromAdviceAction,
	getLSPRange,
	getPathFromTextDocument,
	getWorkerBufferPatches,
} from "../utils";
import {LSPNotificationHandler, LSPRequestHandler} from "./types";

export const formatting: LSPRequestHandler = async (lsp, params) => {
	const path = getPathFromTextDocument(params.get("textDocument"));

	const project = lsp.server.projectManager.findLoadedProject(path);
	if (project === undefined) {
		// Not in a Rome project
		return null;
	}

	const {value, diagnostics} = await catchDiagnostics(async () => {
		return lsp.request.requestWorkerFormat(path, {}, {});
	});

	lsp.logDiagnostics(path, diagnostics);

	if (value === undefined) {
		// Not a file we support formatting or has diagnostics
		return null;
	}

	return diffTextEdits(value.original, value.formatted);
};

export const codeAction: LSPRequestHandler = async (lsp, params) => {
	const path = getPathFromTextDocument(params.get("textDocument"));
	const codeActionRange = getLSPRange(params.get("range"));

	const codeActions: LSPCodeAction[] = [];
	const seenDecisions = new Set<string>();

	const diagnostics = lsp.diagnosticsProcessor.getDiagnosticsForPath(path);
	if (diagnostics.length === 0) {
		return codeActions;
	}

	for (const diag of diagnostics) {
		const diagRange = convertDiagnosticLocationToLSPRange(diag.location);

		if (!doRangesOverlap(diagRange, codeActionRange)) {
			continue;
		}
		for (const item of getActionAdviceFromDiagnostic(diag)) {
			if (item.secondary) {
				continue;
			}

			const decision = getDecisionFromAdviceAction(item);
			if (decision === undefined || seenDecisions.has(decision)) {
				continue;
			}
			seenDecisions.add(decision);

			codeActions.push({
				title: `${readMarkup(item.description)}: ${formatCategoryDescription(
					diag.description,
				)}`,
				command: {
					title: "Rome: Check",
					command: "rome.check.decisions",
					arguments: [path.join(), decision],
				},
			});
		}
	}

	codeActions.push({
		title: "Rome: Fix All",
		kind: "source.fixAll.rome",
		command: {
			title: "Rome: Fix All",
			command: "rome.check.apply",
			arguments: [path.join()],
		},
	});

	return codeActions;
};

export const didOpen: LSPNotificationHandler = async (lsp, params) => {
	const textDocument = params.get("textDocument");
	const path = getPathFromTextDocument(textDocument);
	const project = lsp.server.projectManager.findLoadedProject(path);
	if (project === undefined) {
		return;
	}
	lsp.fileVersions.set(path, textDocument.get("version").asNumber());
	const content = textDocument.get("text").asString();
	await lsp.request.requestWorkerUpdateBuffer(path, content);
	lsp.fileBuffers.add(path);
	lsp.logMessage(path, `Opened: ${path.join()}`);
};

export const didChange: LSPNotificationHandler = async (lsp, params) => {
	const textDocument = params.get("textDocument");
	const path = getPathFromTextDocument(textDocument);
	if (!lsp.fileBuffers.has(path)) {
		return;
	}
	lsp.fileVersions.set(path, textDocument.get("version").asNumber());
	const contentChanges = params.get("contentChanges");

	if (contentChanges.getIndex(0).has("range")) {
		const patches = getWorkerBufferPatches(contentChanges);
		await lsp.request.requestWorkerPatchBuffer(path, patches);
	} else {
		const content = contentChanges.getIndex(0).get("text").asString();
		await lsp.request.requestWorkerUpdateBuffer(path, content);
	}
};

export const didClose: LSPNotificationHandler = async (lsp, params) => {
	const path = getPathFromTextDocument(params.get("textDocument"));
	if (!lsp.fileBuffers.has(path)) {
		return;
	}
	lsp.fileBuffers.delete(path);
	lsp.fileVersions.delete(path);
	await lsp.request.requestWorkerClearBuffer(path);
	lsp.logMessage(path, `Closed: ${path.join()}`);
};
