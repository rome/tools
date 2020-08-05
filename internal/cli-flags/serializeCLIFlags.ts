/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticLocation} from "@internal/diagnostics";
import {toKebabCase} from "@internal/string-utils";
import {ConsumeSourceLocationRequestTarget} from "@internal/consume";
import {Number0, ob1Coerce0, ob1Number0Neg1, ob1Number1} from "@internal/ob1";
import {Dict, RequiredProps} from "@internal/typescript-helpers";
import {FlagValue} from "./Parser";
import {AbsoluteFilePath} from "@internal/path";

export type SerializeCLIOptions = {
	programName: string;
	args: Array<string>;
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
	| "none"
	| "command"
	| "program"
	| "cwd";

function isObjectTarget(
	target: SerializeCLITarget,
): target is SerializeCLITargetObjects {
	return typeof target !== "string";
}

// Used to determine if we should output a -- to disambiguate raw CLI args
function hasConfusingArgs(args: Array<string>): boolean {
	for (const arg of args) {
		if (arg[0] === "-") {
			return true;
		}
	}

	return false;
}

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
): RequiredProps<DiagnosticLocation, "sourceText"> {
	let startColumn: Number0 = ob1Number0Neg1;
	let endColumn: Number0 = ob1Number0Neg1;
	let code = "";

	function setStartColumn() {
		startColumn = ob1Coerce0(code.length);
	}

	function setEndColumn() {
		// Never point to a space
		if (code[code.length - 1] === " ") {
			endColumn = ob1Coerce0(code.length - 1);
		} else {
			endColumn = ob1Coerce0(code.length);
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

		const isTarget =
			isObjectTarget(target) && target.type === "flag" && key === target.key;

		if (isTarget) {
			setStartColumn();
		}

		for (const val of values) {
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
					startColumn = ob1Coerce0(code.length);
				}

				// Number or string
				code += `${String(val)} `;
			}
		}

		if (isTarget) {
			setEndColumn();
		}
	}

	// Disambiguate raw arguments that look like flags
	if (confusingArgs) {
		code += "-- ";
		printArgs();
	}

	if (startColumn === ob1Number0Neg1 || endColumn === ob1Number0Neg1) {
		startColumn = ob1Coerce0(code.length - 1);
		endColumn = startColumn;
	}

	let start: DiagnosticLocation["start"] = {
		line: ob1Number1,
		column: startColumn,
	};

	let end: DiagnosticLocation["end"] = {
		line: ob1Number1,
		column: endColumn,
	};

	if (target === "none") {
		start = undefined;
		end = undefined;
	}

	return {
		language: "shell",
		mtime: undefined,
		sourceText: code.trimRight(),
		filename: "argv",
		start,
		end,
	};
}
