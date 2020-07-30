/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@romefrontend/parser-core";
import {
	DiagnosticAdviceAction,
	DiagnosticCategory,
	DiagnosticDescriptionOptionalCategory,
	DiagnosticLocation,
	descriptions,
} from "@romefrontend/diagnostics";
import {
	LintCompilerOptionsDecision,
	LintCompilerOptionsDecisionAction,
} from "../types";
import {ob1Get0, ob1Get1} from "@romefrontend/ob1";
import {AbsoluteFilePath} from "@romefrontend/path";
import {LinterCompilerOptionsPerFile} from "@romefrontend/core/server/linter/Linter";
import {escapeSplit} from "@romefrontend/string-utils";
import {Markup} from "@romefrontend/markup";

type UnexpectedDecision = (
	description: DiagnosticDescriptionOptionalCategory,
) => void;

function validateAction(
	raw: string,
	unexpected: UnexpectedDecision,
): undefined | LintCompilerOptionsDecisionAction {
	if (raw === "fix" || raw === "suppress" || raw === "ignore") {
		return raw;
	} else {
		unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_ACTION(raw));
		return undefined;
	}
}

export function deriveDecisionPositionKey(
	action: LintCompilerOptionsDecisionAction,
	loc: undefined | DiagnosticLocation,
): undefined | string {
	if (loc === undefined) {
		return undefined;
	}

	const {start} = loc;
	if (start === undefined) {
		return undefined;
	}

	if (action === "suppress") {
		return `${ob1Get1(start.line)}`;
	} else {
		return `${ob1Get1(start.line)}:${ob1Get0(start.column)}`;
	}
}

export function parseDecisionStrings(
	decisions: Array<string>,
	cwd: AbsoluteFilePath,
	unexpected: UnexpectedDecision,
): {
	lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile;
	globalDecisions: Array<LintCompilerOptionsDecision>;
} {
	const lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile = {};
	const globalDecisions: Array<LintCompilerOptionsDecision> = [];

	function parseGlobalDecision(parts: Array<string>, i: number) {
		if (parts.length !== 2) {
			unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_PART_COUNT(i));
		}

		const [rawAction, rawCategory] = parts;

		const action = validateAction(rawAction, unexpected);
		if (action === undefined) {
			return;
		}

		const category = (rawCategory as DiagnosticCategory);
		globalDecisions.push({category, action});
	}

	function parseLineDecision(parts: Array<string>, i: number) {
		if (parts.length < 4 || parts.length > 5) {
			unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_PART_COUNT(i));
		}

		const [rawAction, rawCategory, rawFilename, pos, id] = parts;

		const action = validateAction(rawAction, unexpected);
		if (action === undefined) {
			return;
		}

		const category = (rawCategory as DiagnosticCategory);
		const resolvedFilename = cwd.resolve(rawFilename).join();

		let compilerOptions = lintCompilerOptionsPerFile[resolvedFilename];
		if (compilerOptions === undefined) {
			compilerOptions = {
				hasDecisions: true,
				globalDecisions: [],
				decisionsByPosition: {},
			};
			lintCompilerOptionsPerFile[resolvedFilename] = compilerOptions;
		}

		let decisionsForPosition = compilerOptions.decisionsByPosition[pos];
		if (decisionsForPosition === undefined) {
			decisionsForPosition = [];
			compilerOptions.decisionsByPosition[pos] = decisionsForPosition;
		}

		decisionsForPosition.push({
			action,
			category,
			id: id === undefined ? undefined : Number(id),
		});
	}

	for (let i = 0; i < decisions.length; i++) {
		const segment = decisions[i];
		const parts = escapeSplit(segment, "-");

		if (parts[0] === "global") {
			parseGlobalDecision(parts.slice(1), i);
		} else {
			parseLineDecision(parts, i);
		}
	}

	return {lintCompilerOptionsPerFile, globalDecisions};
}

function escapeFilename(filename: string): string {
	return filename.replace(/-/, "\\-");
}

export function buildLintDecisionGlobalString(
	action: LintCompilerOptionsDecisionAction,
	category: DiagnosticCategory,
): string {
	return `global-${action}-${category}`;
}

export function buildLintDecisionString(
	{
		filename,
		action,
		category,
		start,
		id,
	}: {
		filename: string;
		action: LintCompilerOptionsDecisionAction;
		category: DiagnosticCategory;
		start: Position;
		id?: number;
	},
): string {
	const escapedFilename = escapeFilename(filename);
	const pos = deriveDecisionPositionKey(action, {start});

	const parts = [action, category, escapedFilename, pos];

	if (id !== undefined) {
		parts.push(String(id));
	}

	return parts.join("-");
}

export function buildLintDecisionAdviceAction(
	{
		noun,
		instruction,
		filename,
		shortcut,
		decision,
		extra,
	}: {
		extra?: boolean;
		shortcut?: string;
		noun: Markup;
		instruction: Markup;
		filename?: string;
		decision: string;
	},
): DiagnosticAdviceAction {
	return {
		type: "action",
		extra,
		hidden: true,
		command: "check",
		shortcut,
		args: filename === undefined ? [] : [escapeFilename(filename)],
		noun,
		instruction,
		commandFlags: {
			decisions: [decision],
		},
	};
}
