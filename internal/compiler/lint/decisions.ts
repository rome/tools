/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, addPositions} from "@internal/parser-core";
import {
	DiagnosticAdviceAction,
	DiagnosticCategory,
	DiagnosticDescriptionOptional,
	DiagnosticLocation,
	descriptions,
	joinCategoryName,
} from "@internal/diagnostics";
import {
	LintCompilerOptionsDecision,
	LintCompilerOptionsDecisionAction,
} from "../types";
import {ob1Coerce0, ob1Get0, ob1Get1, ob1Number1} from "@internal/ob1";
import {AbsoluteFilePath, AnyPath} from "@internal/path";
import {LinterCompilerOptionsPerFile} from "@internal/core/server/linter/Linter";
import {escapeSplit} from "@internal/string-utils";
import {StaticMarkup} from "@internal/markup";
import {parseCommentSuppressionLoneCategory} from "../suppressionsParser";

type UnexpectedDecision = (description: DiagnosticDescriptionOptional) => void;

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
	loc: undefined | Partial<DiagnosticLocation>,
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

function addPartPositionOffset(pos: Position, part: string): Position {
	return addPositions(
		pos,
		{line: ob1Number1, column: ob1Coerce0(part.length + 1)},
	);
}

export function parseDecisionStrings(
	{decisions, cwd, path, unexpected}: {
		path: AnyPath;
		decisions: {
			start: Position;
			value: string;
		}[];
		cwd: AbsoluteFilePath;
		unexpected: UnexpectedDecision;
	},
): {
	lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile;
	globalDecisions: LintCompilerOptionsDecision[];
} {
	const lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile = {};
	const globalDecisions: LintCompilerOptionsDecision[] = [];

	function parseGlobalDecision(start: Position, parts: string[], i: number) {
		if (parts.length !== 2) {
			unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_PART_COUNT(i));
		}

		const [rawAction, rawCategory] = parts;

		const action = validateAction(rawAction, unexpected);
		if (action === undefined) {
			return;
		}

		const {category, categoryValue} = parseCommentSuppressionLoneCategory({
			input: rawCategory,
			path,
			offsetPosition: start,
		});
		globalDecisions.push({category, categoryValue, action});
	}

	function parseLineDecision(start: Position, parts: string[], i: number) {
		if (parts.length < 4 || parts.length > 5) {
			unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_PART_COUNT(i));
		}

		const [rawAction, rawCategory, rawFilename, pos, id] = parts;

		const action = validateAction(rawAction, unexpected);
		if (action === undefined) {
			return;
		}

		const {category, categoryValue} = parseCommentSuppressionLoneCategory({
			input: rawCategory,
			path,
			offsetPosition: addPartPositionOffset(start, action),
		});
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
			categoryValue,
			id: id === undefined ? undefined : Number(id),
		});
	}

	for (let i = 0; i < decisions.length; i++) {
		const {start, value} = decisions[i];
		const parts = escapeSplit(value, "-");

		if (parts[0] === "global") {
			parseGlobalDecision(
				addPartPositionOffset(start, "global"),
				parts.slice(1),
				i,
			);
		} else {
			parseLineDecision(start, parts, i);
		}
	}

	return {lintCompilerOptionsPerFile, globalDecisions};
}

function escapePart(str: string): string {
	return str.replace(/-/, "\\-");
}

export function buildLintDecisionGlobalString(
	action: LintCompilerOptionsDecisionAction,
	category: DiagnosticCategory,
	categoryValue: undefined | string,
): string {
	return `global-${action}-${joinCategoryName({category, categoryValue})}`;
}

export function buildLintDecisionString(
	{
		filename,
		action,
		category,
		categoryValue,
		start,
		id,
	}: {
		filename: string;
		action: LintCompilerOptionsDecisionAction;
		category: DiagnosticCategory;
		categoryValue: undefined | string;
		start: Position;
		id?: number;
	},
): string {
	const pos = deriveDecisionPositionKey(action, {start});
	if (pos === undefined) {
		throw new Error(
			"We explicitly provide start so this should not be undefined",
		);
	}

	const parts = [
		action,
		joinCategoryName({category, categoryValue}),
		filename,
		pos,
	];

	if (id !== undefined) {
		parts.push(String(id));
	}

	if (parts.findIndex((part) => part.replace === undefined) !== -1) {
		throw new Error(JSON.stringify({parts, id, pos}));
	}

	return parts.map((part) => escapePart(part)).join("-");
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
		noun: StaticMarkup;
		instruction: StaticMarkup;
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
		args: filename === undefined ? [] : [escapePart(filename)],
		noun,
		instruction,
		commandFlags: {
			decisions: [decision],
		},
	};
}
