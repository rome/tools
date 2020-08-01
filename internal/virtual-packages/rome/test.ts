/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AsyncVoidCallback, JSONPropertyValue, VoidCallback} from "./types";

export type ExpectedError = undefined | string | RegExp | Function;

export type TestSnapshotOptions = {
	filename?: string;
	language?: string;
};

// These diagnostics are subsets of the official diagnostics
// We can potentially normalize these and ensure backwards compatibility with the official diagnostics

export type TestDiagnosticLogCategory = "none" | "info" | "warn" | "error";

export type TestDiagnosticAdviceInspect = {
	type: "inspect";
	data: JSONPropertyValue;
};

export type TestDiagnosticAdviceList = {
	type: "list";
	list: Array<string>;
};

export type TestDiagnosticAdviceCode = {
	type: "code";
	sourceText: string;
};

export type TestDiagnosticAdviceLog = {
	type: "log";
	category: TestDiagnosticLogCategory;
	text: string;
};

export type TestDiagnosticAdviceItem =
	| TestDiagnosticAdviceInspect
	| TestDiagnosticAdviceCode
	| TestDiagnosticAdviceLog
	| TestDiagnosticAdviceList;

export interface TestHelper {
	addToAdvice(
		item: TestDiagnosticAdviceItem | (() => TestDiagnosticAdviceItem),
	): void;
	clearAdvice(): void;
	onTeardown(callback: AsyncVoidCallback): void;
	clearTimeout(): void;
	extendTimeout(time: number): void;
	setTimeout(time: number): void;
	checkTimeout(): void;
	truthy(value: unknown, message?: string): void;
	falsy(value: unknown, message?: string): void;
	true(value: unknown, message?: string): void;
	false(value: unknown, message?: string): void;
	is(received: unknown, expected: unknown, message?: string): void;
	not(received: unknown, expected: unknown, message?: string): void;
	looksLike(received: unknown, expected: unknown, message?: string): void;
	notLooksLike(received: unknown, expected: unknown, message?: string): void;
	throws(
		thrower: VoidCallback,
		expected?: ExpectedError,
		message?: string,
	): void;
	throwsAsync(
		thrower: AsyncVoidCallback,
		expected?: ExpectedError,
		message?: string,
	): Promise<void>;
	notThrows(nonThrower: VoidCallback, message?: string): void;
	notThrowsAsync(
		nonThrower: AsyncVoidCallback,
		message?: string,
	): Promise<void>;
	regex(contents: string, regex: RegExp, message?: string): void;
	notRegex(contents: string, regex: RegExp, message?: string): void;
	snapshot(
		expected: unknown,
		message?: string,
		opts?: TestSnapshotOptions,
	): string;
	inlineSnapshot(received: unknown, expected?: string | boolean | number): void;
	namedSnapshot(
		name: string,
		expected: unknown,
		message?: string,
		opts?: TestSnapshotOptions,
	): string;
}

export type TestName = string | Array<string>;

declare const __ROME__TEST_OPTIONS__: GlobalTestOptions;

export type GlobalTestOptions =
	| undefined
	| {
			dirname?: string;
			register?: (err: Error, opts: TestOptions, callback: TestCallback) => void;
		};

type NamelessTestOptions = {
	timeout?: number;
	only?: boolean;
};

export type TestCallback = (t: TestHelper) => void | undefined | Promise<void>;

export type TestOptions = NamelessTestOptions & {
	name: TestName;
};

type TestArg = TestName | NamelessTestOptions | TestCallback | undefined;

export const testOptions: NonNullable<GlobalTestOptions> =
	typeof __ROME__TEST_OPTIONS__ === "undefined" ? {} : __ROME__TEST_OPTIONS__;

function registerTest(
	callsiteError: Error,
	opts: TestOptions,
	callback: TestCallback,
) {
	const register = testOptions.register;

	if (typeof register !== "function") {
		throw new Error("Test harness does not exist");
	}

	register(callsiteError, opts, callback);
}

function isOptionsObject(arg: TestArg): arg is NamelessTestOptions {
	return typeof arg === "object" && arg != null && !Array.isArray(arg);
}

function splitArgs(
	args: TestRegisterFunctionArgs,
): {
	options: TestOptions;
	callback: TestCallback;
} {
	const name = args.shift();
	if (typeof name !== "string" && !Array.isArray(name)) {
		throw new Error("Expected test name to be a string or an array of strings");
	}

	const callback = args.pop();
	if (typeof callback !== "function") {
		throw new Error("Expected options callback");
	}

	const options = args.pop();
	if (options !== undefined && !isOptionsObject(options)) {
		throw new Error("Expected options object");
	}

	if (args.length > 0) {
		throw new Error("Expected to have exhausted test register arguments");
	}

	return {
		options: {
			...options,
			name,
		},
		callback,
	};
}

type TestRegisterFunctionArgs =
	| [TestName, TestCallback]
	| [TestName, NamelessTestOptions, TestCallback];

type TestRegisterFunction = (...args: TestRegisterFunctionArgs) => void;

export const test: TestRegisterFunction & {
	only: TestRegisterFunction;
} = function(...args: TestRegisterFunctionArgs) {
	const {options, callback} = splitArgs(args);
	registerTest(new Error(), options, callback);
};

test.only = function(...args: TestRegisterFunctionArgs) {
	const {options, callback} = splitArgs(args);
	registerTest(
		new Error(),
		{
			...options,
			only: true,
		},
		callback,
	);
};
