/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticLocation} from "@internal/diagnostics";
import {toKebabCase} from "@internal/string-utils";
import {ConsumeSourceLocationRequestTarget} from "@internal/consume";
import {OneIndexed, ZeroIndexed} from "@internal/math";
import {Dict, RequiredProps} from "@internal/typescript-helpers";
import {FlagValue} from "./Parser";
import {AbsoluteFilePath, createUIDPath} from "@internal/path";

export type SerializeCLILocation = RequiredProps<
	DiagnosticLocation,
	"start" | "end" | "sourceText"
>;

export type SerializeCLIOptions = {
	programName: string;
	args: string[];
	flags: Dict<FlagValue>;

	// Optional
	cwd?: AbsoluteFilePath;
	commandName?: string;
	defaultFlags?: Dict<FlagValue>;
	incorrectCaseFlags?: Set<string>;
	shorthandFlags?: Set<string>;
};

type SerializeCLITargetObjects =
	| {
			type: "flag";
			key: string;
			target?: ConsumeSourceLocationRequestTarget;
			index?: number;
		}
	| {
			type: "arg";
			key: number;
		}
	| {
			type: "arg-range";
			from: number;
			to?: number;
		};

export type SerializeCLITarget =
	| SerializeCLITargetObjects
	| "command"
	| "program"
	| "cwd"
	| "none";

function isObjectTarget(
	target: SerializeCLITarget,
): target is SerializeCLITargetObjects {
	return typeof target !== "string";
}

// Used to determine if we should output a -- to disambiguate raw CLI args
function hasConfusingArgs(args: string[]): boolean {
	for (const arg of args) {
		if (arg[0] === "-") {
			return true;
		}
	}

	return false;
}

export function serializeCLIFlags(
	opts: SerializeCLIOptions,
	target: "none",
): RequiredProps<DiagnosticLocation, "sourceText">;
export function serializeCLIFlags(
	opts: SerializeCLIOptions,
	target: Exclude<SerializeCLITarget, "none">,
): SerializeCLILocation;
export function serializeCLIFlags(
	opts: SerializeCLIOptions,
	target: SerializeCLITarget,
): DiagnosticLocation;
export function serializeCLIFlags(
	{
		args,
		flags,
		cwd,
		programName,
		commandName,

		defaultFlags = {},
		shorthandFlags = new Set(),
		incorrectCaseFlags = new Set(),
	}: SerializeCLIOptions,
	target: SerializeCLITarget,
): DiagnosticLocation {
	let startColumn: ZeroIndexed = new ZeroIndexed(-1);
	let endColumn: ZeroIndexed = new ZeroIndexed(-1);
	let code = "";

	function setStartColumn() {
		startColumn = new ZeroIndexed(code.length);
	}

	function setEndColumn() {
		// Never point to a space
		if (code[code.length - 1] === " ") {
			endColumn = new ZeroIndexed(code.length - 1);
		} else {
			endColumn = new ZeroIndexed(code.length);
		}
	}

	function push(str: string, set: boolean) {
		if (set) {
			setStartColumn();
		}

		code += str;

		if (set) {
			setEndColumn();
		}
	}

	function printArgs() {
		for (let i = 0; i < args.length; i++) {
			const arg = args[i];

			let isTarget = false;
			if (isObjectTarget(target) && target.type === "arg" && i === target.key) {
				isTarget = true;
			}
			if (
				isObjectTarget(target) &&
				target.type === "arg-range" &&
				target.from === i
			) {
				isTarget = true;
			}

			if (isTarget) {
				setStartColumn();
			}

			code += `${arg} `;

			let isEndTarget = isTarget;

			// We are the end target if we're within the from-to range or we're greater than from with no to
			if (
				isObjectTarget(target) &&
				target.type === "arg-range" &&
				i > target.from &&
				(target.to === undefined || target.to <= i)
			) {
				isEndTarget = true;
			}

			if (isEndTarget) {
				setEndColumn();
			}
		}
	}

	// Only output cwd if it's the target
	if (cwd !== undefined && target === "cwd") {
		push(cwd.join(), true);
		code += "$ ";
	}

	push(`${programName} `, target === "program");

	if (commandName !== undefined) {
		push(`${commandName} `, target === "command");
	}

	const confusingArgs = hasConfusingArgs(args);

	if (!confusingArgs) {
		printArgs();
	}

	// Add flags
	for (const key in flags) {
		const val = flags[key];

		// Ignore pointless default values
		if (val === defaultFlags[key]) {
			continue;
		}

		const values = Array.isArray(val) ? val : [val];

		let isTarget = false;
		let isEntireTarget = false;
		let targetIndex;
		if (isObjectTarget(target) && target.type === "flag") {
			isTarget = key === target.key;
			if (isTarget) {
				isEntireTarget = target.index === undefined;
				targetIndex = target.index;
			}
		}

		if (isEntireTarget) {
			setStartColumn();
		}

		for (let i = 0; i < values.length; i++) {
			const val = values[i];

			let isIndexTarget = targetIndex === i;
			if (isIndexTarget) {
				setStartColumn();
			}

			const flagPrefix = shorthandFlags.has(key) ? "-" : "--";
			const kebabKey = incorrectCaseFlags.has(key) ? key : toKebabCase(key);
			if (val === false) {
				code += `${flagPrefix}no-${kebabKey} `;
			} else {
				code += `${flagPrefix}${kebabKey} `;
			}

			// Booleans are always indicated with just their flag
			if (typeof val !== "boolean") {
				// Only point to the value for flags that specify it
				if (
					isTarget &&
					isObjectTarget(target) &&
					target.type === "flag" &&
					(target.target === "value" || target.target === "inner-value")
				) {
					startColumn = new ZeroIndexed(code.length);
				}

				// Number or string
				code += `${String(val)} `;
			}

			if (isIndexTarget) {
				setEndColumn();
			}
		}

		if (isEntireTarget) {
			setEndColumn();
		}
	}

	// Disambiguate raw arguments that look like flags
	if (confusingArgs) {
		code += "-- ";
		printArgs();
	}

	if (startColumn.valueOf() === -1 || endColumn.valueOf() === -1) {
		startColumn = new ZeroIndexed(code.length - 1);
		endColumn = startColumn;
	}

	let start: DiagnosticLocation["start"] = {
		line: new OneIndexed(),
		column: startColumn,
	};

	let end: DiagnosticLocation["end"] = {
		line: new OneIndexed(),
		column: endColumn,
	};

	if (target === "none") {
		start = undefined;
		end = undefined;
	}

	return {
		language: "shell",
		integrity: undefined,
		sourceText: code.trimRight(),
		path: createUIDPath("argv"),
		start,
		end,
	};
}
