/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticLocation} from "@romefrontend/diagnostics";
import {toKebabCase} from "@romefrontend/string-utils";
import {ConsumeSourceLocationRequestTarget} from "@romefrontend/consume";
import {
	Number0,
	ob1Coerce0,
	ob1Number0Neg1,
	ob1Number1,
} from "@romefrontend/ob1";
import {Dict, RequiredProps} from "@romefrontend/typescript-helpers";
import {FlagValue} from "./Parser";
import {AbsoluteFilePath} from "@romefrontend/path";

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
	prefix?: string;
};

export type SerializeCLITarget =
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
		}
	| {
			type: "none";
		}
	| {
			type: "command";
		}
	| {
			type: "program";
		}
	| {
			type: "cwd";
		};

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
		prefix = "$ ",
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

	// Only output cwd if it's the target
	if (cwd !== undefined && target.type === "cwd") {
		push(cwd.join(), true);
	}

	code += prefix;

	push(`${programName} `, target.type === "program");

	if (commandName !== undefined) {
		push(`${commandName} `, target.type === "command");
	}

	// Add args
	for (let i = 0; i < args.length; i++) {
		const arg = args[i];

		let isTarget = false;
		if (target.type === "arg" && i === target.key) {
			isTarget = true;
		}
		if (target.type === "arg-range" && target.from === i) {
			isTarget = true;
		}

		if (isTarget) {
			setStartColumn();
		}

		code += `${arg} `;

		let isEndTarget = isTarget;

		// We are the end target if we're within the from-to range or we're greater than from with no to
		if (
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

	// Add flags
	for (const key in flags) {
		const val = flags[key];

		// Ignore pointless default values
		if (val === defaultFlags[key]) {
			continue;
		}

		const values = Array.isArray(val) ? val : [val];

		const isTarget = target.type === "flag" && key === target.key;

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

	if (startColumn === ob1Number0Neg1 || endColumn === ob1Number0Neg1) {
		startColumn = ob1Coerce0(code.length - 1);
		endColumn = startColumn;
	}

	let start: DiagnosticLocation["start"] = {
		line: ob1Number1,
		column: startColumn,
		index: startColumn,
	};

	let end: DiagnosticLocation["end"] = {
		line: ob1Number1,
		column: endColumn,
		index: endColumn,
	};

	if (target.type === "none") {
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
