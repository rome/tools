/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import ClientRequest from "./ClientRequest";
import {DiagnosticsPrinter, printDiagnostics} from "@internal/cli-diagnostics";
import {SelectOption} from "@internal/cli-reporter";
import {
	Diagnostic,
	DiagnosticAdviceAction,
	DiagnosticsProcessor,
	derivePositionlessKeyFromDiagnostic,
} from "@internal/diagnostics";
import {ClientRequestFlags, ServerQueryResponse} from "@internal/core";

import {Dict} from "@internal/typescript-helpers";
import {EMPTY_SUCCESS_RESPONSE} from "../server/ServerRequest";
import {markup} from "@internal/markup";
import {ExtendedMap} from "@internal/collections";

type State = {
	initial: boolean;
	seen: Set<string>;
	resolvedCount: number;
};

async function check(
	req: ClientRequest,
	state: State,
): Promise<ServerQueryResponse> {
	const {reporter} = req.client;

	reporter.clearScreen();

	if (state.initial) {
		reporter.info(markup`Fetching initial diagnostics`);
		state.initial = false;
	} else {
		reporter.info(markup`Updating diagnostics`);
	}

	const res = await req.fork({
		...req.query,
		// We want data no matter what
		noData: false,
	}).initCommand();

	if (res.type === "SUCCESS") {
		throw new Error("Expected diagnostics or an error");
	}

	// In case it returned an error
	if (res.type !== "DIAGNOSTICS") {
		return res;
	}

	const diagnostics = res.diagnostics;
	let diag: undefined | Diagnostic;

	for (const _diag of diagnostics) {
		const key = derivePositionlessKeyFromDiagnostic(_diag);
		if (!state.seen.has(key)) {
			state.seen.add(key);
			diag = _diag;
			break;
		}
	}

	if (diag === undefined) {
		return res;
	}

	return await ask(diag, req, state, false, diagnostics.length);
}

async function ask(
	diag: Diagnostic,
	req: ClientRequest,
	state: State,
	showMoreOptions: boolean,
	totalDiagnostics: number,
): Promise<ServerQueryResponse> {
	const {client} = req;
	const {reporter} = client;
	reporter.clearScreen();

	// Extract actions and remove them from the diagnostic
	let {advice = []} = diag.description;
	let hasExtraOptions = false;
	const actions: Array<DiagnosticAdviceAction> = [];
	for (const item of advice) {
		if (item.type === "action") {
			// Only show extra items and hide all non-extra items when `more === true`
			if (item.extra === true) {
				hasExtraOptions = true;
				if (!showMoreOptions) {
					continue;
				}
			} else if (showMoreOptions) {
				continue;
			}

			actions.push(item);
		}
	}
	advice = advice.filter((item) => item.type !== "action");
	diag = {
		...diag,
		description: {
			...diag.description,
			advice,
		},
	};

	const optionToAction: ExtendedMap<string, DiagnosticAdviceAction> = new ExtendedMap(
		"optionToAction",
	);
	const chosenShortcuts: Set<string> = new Set(["n", "escape"]);

	const actionOptions: Dict<SelectOption> = {};

	let counter = 0;
	for (const action of actions) {
		const key = String(counter++);
		let shortcut =
			action.shortcut !== undefined && !chosenShortcuts.has(action.shortcut)
				? action.shortcut
				: undefined;
		optionToAction.set(key, action);
		actionOptions[key] = {
			label: action.noun,
			shortcut,
		};
	}

	const options: {
		ignore: SelectOption;
		exit: SelectOption;
		more?: SelectOption;
		less?: SelectOption;
	} = {
		ignore: {
			label: markup`Do nothing`,
			shortcut: "n",
		},
		...actionOptions,
		exit: {
			label: markup`Exit`,
			shortcut: "escape",
		},
	};

	if (hasExtraOptions) {
		if (showMoreOptions) {
			options.more = {
				label: markup`Less options...`,
				shortcut: "l",
			};
		} else {
			options.more = {
				label: markup`More options...`,
				shortcut: "m",
			};
		}
	}

	reporter.heading(
		markup`Reviewing diagnostics (<emphasis>${totalDiagnostics}</emphasis><dim>/</dim><emphasis>${totalDiagnostics +
		state.resolvedCount}</emphasis>)`,
	);

	const printer = new DiagnosticsPrinter({
		processor: new DiagnosticsProcessor(),
		reporter,
		wrapErrors: true,
	});
	diag = printer.processor.addDiagnosticAssert(diag);
	await printer.print();

	const answer = await reporter.radio(
		markup`How do you want to resolve this?`,
		{
			options,
		},
	);

	// Check if this diagnostic is now out of date
	await printer.fetchFileSources([diag]);
	const outdatedFiles = printer.getOutdatedFiles(diag);
	if (outdatedFiles.size > 0) {
		const files = Array.from(
			outdatedFiles,
			(path) => markup`<emphasis>${path}</emphasis>`,
		);

		reporter.br();

		if (files.length === 1) {
			reporter.warn(
				markup`The file ${files[0]} changed while waiting for your response.`,
			);
		} else {
			reporter.warn(
				markup`The following diagnostic dependencies changed while waiting for your response.`,
			);
			reporter.list(files);
		}

		await reporter.confirm("Press any key to try again");

		return await check(req, state);
	}

	if (answer === "less") {
		return await ask(diag, req, state, false, totalDiagnostics);
	}

	if (answer === "more") {
		return await ask(diag, req, state, true, totalDiagnostics);
	}

	if (answer === "ignore") {
		return await check(req, state);
	}

	if (answer === "exit") {
		return EMPTY_SUCCESS_RESPONSE;
	}

	const action = optionToAction.assert(answer);

	const requestFlags: Partial<ClientRequestFlags> = {
		...action.requestFlags,
	};

	// Execute action
	const actionRes = await client.query(
		{
			commandName: action.command,
			args: action.args,
			commandFlags: action.commandFlags,
			requestFlags,
		},
		"server",
	);
	if (actionRes.type !== "DIAGNOSTICS" && actionRes.type !== "SUCCESS") {
		return actionRes;
	}

	state.resolvedCount++;
	return await check(req, state);
}

export default async function review(
	req: ClientRequest,
): Promise<ServerQueryResponse> {
	const {reporter} = req.client;
	const state: State = {
		initial: true,
		seen: new Set(),
		resolvedCount: 0,
	};
	const res = await check(req, state);

	reporter.clearScreen();

	if (state.seen.size === 0) {
		reporter.success(markup`Nothing to review!`);
	} else {
		if (res.type === "DIAGNOSTICS") {
			await printDiagnostics({
				diagnostics: res.diagnostics,
				suppressions: [],
				excludeFooter: true,
				printerOptions: {
					processor: new DiagnosticsProcessor(),
					reporter,
				},
			});
			reporter.hr();
			reporter.error(
				markup`<number emphasis>${String(res.diagnostics.length)}</number> unresolved <grammarNumber plural="issues" singular="issue">${String(
					res.diagnostics.length,
				)}</grammarNumber> remaining`,
			);
		}

		reporter.success(
			markup`<number emphasis>${String(state.resolvedCount)}</number> <grammarNumber plural="issues" singular="issue">${String(
				state.resolvedCount,
			)}</grammarNumber> resolved`,
		);
	}

	return res;
}
