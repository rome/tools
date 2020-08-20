/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticAdviceItem,
	createSingleDiagnosticError,
	deriveDiagnosticFromErrorStructure,
	descriptions,
	getErrorStackAdvice,
} from "@internal/diagnostics";
import SnapshotManager from "./SnapshotManager";
import {TestServerRunnerOptions} from "../server/testing/types";
import {Event} from "@internal/events";
import stringDiff from "@internal/string-diff";
import {getErrorStructure} from "@internal/v8";
import {prettyFormatToString} from "@internal/pretty-format";
import {markup} from "@internal/markup";
import {
	ExpectedError,
	TestDiagnosticAdviceItem,
	TestHelper,
	TestSnapshotOptions,
} from "@internal/virtual-rome/test";
import {cleanFrames} from "./TestWorkerFile";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import {AbsoluteFilePath} from "@internal/path";

function formatExpectedError(expected: ExpectedError): string {
	if (typeof expected === "string") {
		return JSON.stringify(expected);
	}

	if (expected instanceof RegExp) {
		return String(expected);
	}

	if (typeof expected === "function") {
		return expected.name;
	}

	return "unknown";
}

function matchExpectedError(error: Error, expected: ExpectedError): boolean {
	if (expected === undefined) {
		return true;
	}

	if (typeof expected === "string") {
		return error.message.includes(expected);
	}

	if (expected instanceof RegExp) {
		return expected.test(error.message);
	}

	if (typeof expected === "function") {
		return error instanceof expected;
	}

	return false;
}

export type OnTimeout = (time: number) => void;

type SnapshotOptions = {
	entryName: string;
	expected: unknown;
	message?: string;
	opts?: TestSnapshotOptions;
};

type EmitDiagnostic = (diag: Diagnostic) => Promise<void>;

function normalizeUserAdvice(advice: Array<UserAdviceItem>): DiagnosticAdvice {
	return advice.map((item) => {
		if (typeof item === "function") {
			return normalizeUserAdviceItem(item());
		} else {
			return normalizeUserAdviceItem(item);
		}
	});
}

// Once we have a public test framework we should perform normalization here
function normalizeUserAdviceItem(
	item: TestDiagnosticAdviceItem,
): DiagnosticAdviceItem {
	switch (item.type) {
		case "code":
			return {
				...item,
				language: "unknown",
			};

		case "log":
			return {
				...item,
				text: markup`${item.text}`,
			};

		case "list":
			return {
				...item,
				list: item.list.map((item) => markup`${item}`),
			};

		default:
			return item;
	}
}

type UserAdviceItem =
	| TestDiagnosticAdviceItem
	| (() => TestDiagnosticAdviceItem);

export default class TestAPI implements TestHelper {
	constructor(
		{
			testName,
			onTimeout,
			path,
			snapshotManager,
			options,
			emitDiagnostic,
		}: {
			emitDiagnostic: EmitDiagnostic;
			path: AbsoluteFilePath;
			testName: string;
			onTimeout: OnTimeout;
			snapshotManager: SnapshotManager;
			options: TestServerRunnerOptions;
		},
	) {
		this.testName = testName;
		this.options = options;
		this.snapshotManager = snapshotManager;
		this.snapshotCounter = 0;
		this.path = path;
		this.teardownEvent = new Event({name: "TestAPI.teardown"});
		this.startTime = Date.now();
		this.onTimeout = onTimeout;
		this.emitDiagnostic = emitDiagnostic;
		this.timeoutMax = 0;
		this.timeoutId = undefined;
		this.setTimeout(5_000);
		this.advice = [];
	}

	public teardownEvent: Event<void, void>;

	private startTime: number;
	private options: TestServerRunnerOptions;
	private path: AbsoluteFilePath;
	private emitDiagnostic: EmitDiagnostic;

	private onTimeout: OnTimeout;
	private timeoutId: undefined | NodeJS.Timeout;
	private timeoutStart: undefined | number;
	private timeoutMax: undefined | number;

	private advice: Array<UserAdviceItem>;
	private testName: string;
	private snapshotCounter: number;
	private snapshotManager: SnapshotManager;

	public getAdvice(startAdvice: DiagnosticAdvice = []): DiagnosticAdvice {
		const {advice} = this;

		if (advice.length === 0) {
			return startAdvice;
		}

		const userAdvice = normalizeUserAdvice(advice);

		return [
			...startAdvice,
			{
				type: "group",
				title: markup`User-specified test advice`,
				advice: userAdvice,
			},
		];
	}

	private buildMatchAdvice(
		received: unknown,
		expected: unknown,
		{
			visualMethod,
			expectedAlias,
			receivedAlias,
		}: {
			visualMethod?: string;
			expectedAlias?: string;
			receivedAlias?: string;
		} = {},
	): DiagnosticAdvice {
		let expectedFormat;
		let receivedFormat;
		if (typeof received === "string" && typeof expected === "string") {
			expectedFormat = expected;
			receivedFormat = received;
		} else {
			expectedFormat = prettyFormatToString(expected);
			receivedFormat = prettyFormatToString(received);
		}

		const advice: DiagnosticAdvice = [];

		if (expectedFormat === receivedFormat) {
			// Better error message when both values are visually identical
			advice.push({
				type: "log",
				category: "info",
				text: markup`Both the received and expected values are visually identical`,
			});

			advice.push({
				type: "code",
				language: "unknown",
				sourceText: expectedFormat,
			});

			if (visualMethod !== undefined) {
				advice.push({
					type: "log",
					category: "info",
					text: markup`Try using t.${visualMethod} if you wanted a visual match`,
				});
			}
		} else {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Comparison`,
			});

			advice.push({
				type: "diff",
				language: "unknown",
				diff: stringDiff(expectedFormat, receivedFormat),
				legend: {
					add: receivedAlias ? receivedAlias : "Received",
					delete: expectedAlias ? expectedAlias : "Expected",
				},
			});
		}

		return advice;
	}

	// We allow lazy construction of test advice when an error actually occurs
	public addToAdvice(item: UserAdviceItem): void {
		this.advice.push(item);
	}

	public clearAdvice() {
		this.advice = [];
	}

	public onTeardown(callback: AsyncVoidCallback): void {
		this.teardownEvent.subscribe(callback);
	}

	public clearTimeout(): void {
		if (this.timeoutId !== undefined) {
			clearTimeout(this.timeoutId);
		}

		this.timeoutMax = undefined;
		this.timeoutStart = undefined;
	}

	public extendTimeout(time: number): void {
		const {timeoutMax, timeoutStart} = this;
		if (timeoutMax === undefined || timeoutStart === undefined) {
			throw new Error("No timeout set");
		}

		const elapsed = Date.now() - timeoutStart;
		const newTime = timeoutMax - elapsed + time;
		this.setTimeout(newTime);
	}

	public setTimeout(time: number): void {
		this.clearTimeout();

		this.timeoutStart = Date.now();
		this.timeoutMax = time;

		this.timeoutId = setTimeout(
			() => {
				this.onTimeout(time);
			},
			time,
		);
	}

	public checkTimeout(): void {
		const {startTime, timeoutMax} = this;
		if (timeoutMax === undefined) {
			return;
		}

		const delta = Date.now() - startTime;
		if (delta > timeoutMax) {
			throw new Error(`Test timeout - exceeded ${String(timeoutMax)}ms`);
		}
	}

	public fail(
		message: string = "Test failure triggered by t.fail()",
		advice: DiagnosticAdvice = [],
		framesToShift: number = 0,
	): never {
		const diag = deriveDiagnosticFromErrorStructure(
			getErrorStructure(new Error(), framesToShift + 1),
			{
				description: {
					category: "tests/failure",
					message: markup`${message}`,
					advice,
				},
			},
		);
		throw createSingleDiagnosticError(diag);
	}

	public truthy(
		value: unknown,
		message: string = "Expected value to be truthy",
	): void {
		if (Boolean(value) === false) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(value),
					},
				],
				1,
			);
		}
	}

	public falsy(
		value: unknown,
		message: string = "Expected value to be falsy",
	): void {
		if (Boolean(value) === true) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(value),
					},
				],
				1,
			);
		}
	}

	public true(
		value: unknown,
		message: string = "Expected value to be true",
	): void {
		if (value !== true) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(value),
					},
				],
				1,
			);
		}
	}

	public false(
		value: unknown,
		message: string = "Expected value to be false",
	): void {
		if (value !== false) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(value),
					},
				],
				1,
			);
		}
	}

	public is(
		received: unknown,
		expected: unknown,
		message: string = "t.is() failed, using Object.is semantics",
	): void {
		if (Object.is(received, expected) !== true) {
			this.fail(
				message,
				this.buildMatchAdvice(
					received,
					expected,
					{
						visualMethod: "looksLike",
					},
				),
				1,
			);
		}
	}

	public not(
		received: unknown,
		expected: unknown,
		message: string = "t.not() failed, using !Object.is semantics",
	): void {
		if (Object.is(received, expected) === true) {
			this.fail(
				message,
				this.buildMatchAdvice(
					received,
					expected,
					{
						visualMethod: "notLooksLike",
					},
				),
				1,
			);
		}
	}

	public looksLike(
		received: unknown,
		expected: unknown,
		message: string = "t.looksLike() failed, using prettyFormat semantics",
	): void {
		const actualInspect = prettyFormatToString(received);
		const expectedInspect = prettyFormatToString(expected);

		if (actualInspect !== expectedInspect) {
			this.fail(message, this.buildMatchAdvice(received, expected), 1);
		}
	}

	public notLooksLike(
		received: unknown,
		expected: unknown,
		message: string = "t.notLooksLike() failed, using !prettyFormat semantics",
	): void {
		const actualInspect = prettyFormatToString(received);
		const expectedInspect = prettyFormatToString(expected);

		if (actualInspect === expectedInspect) {
			this.fail(message, this.buildMatchAdvice(received, expected), 1);
		}
	}

	public throws(
		thrower: VoidCallback,
		expected?: ExpectedError,
		message: string = "t.throws() failed, callback did not throw an error",
	): void {
		try {
			thrower();
		} catch (err) {
			if (matchExpectedError(err, expected)) {
				return undefined;
			} else {
				this.fail(
					`t.throws() expected an error to be thrown that matches ${formatExpectedError(
						expected,
					)} but got ${err.name}: ${JSON.stringify(err.message)}`,
					getErrorStackAdvice(
						getErrorStructure(err),
						{
							title: markup`Incorrect error stack trace`,
						},
					),
					1,
				);
			}
		}

		this.fail(message, undefined, 1);
	}

	public async throwsAsync(
		thrower: AsyncVoidCallback,
		expected?: ExpectedError,
		message: string = "t.throwsAsync() failed, callback did not throw an error",
	): Promise<void> {
		try {
			await thrower();
		} catch (err) {
			if (matchExpectedError(err, expected)) {
				return undefined;
			} else {
				this.fail(
					`t.throwsAsync() expected an error to be thrown that matches ${formatExpectedError(
						expected,
					)} but got ${err.name}: ${JSON.stringify(err.message)}`,
					getErrorStackAdvice(
						getErrorStructure(err),
						{
							title: markup`Incorrect error stack trace`,
						},
					),
					1,
				);
			}
		}
		this.fail(message, undefined, 1);
	}

	public notThrows(
		nonThrower: VoidCallback,
		message: string = "t.notThrows() failed, callback threw an error",
	): void {
		try {
			nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				{
					title: markup`t.notThrows() did not expect an error to be thrown but got ${err.name}: ${JSON.stringify(
						err.message,
					)}`,
				},
			);
			this.fail(message, advice, 1);
		}
	}

	public async notThrowsAsync(
		nonThrower: AsyncVoidCallback,
		message: string = "t.notThrowsAsync() failed, callback threw an error",
	): Promise<void> {
		try {
			await nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				{
					title: markup`t.notThrowsAsync() did not expect an error to be thrown but got ${err.name}: ${JSON.stringify(
						err.message,
					)}`,
				},
			);
			this.fail(message, advice, 1);
		}
	}

	public regex(
		contents: string,
		regex: RegExp,
		message: string = "t.regex() failed, using RegExp.test semantics",
	): void {
		if (!regex.test(contents)) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Expected`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(contents),
					},
					{
						type: "log",
						category: "info",
						text: markup`to match pattern`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(regex.source),
					},
				],
				1,
			);
		}
	}

	public notRegex(
		contents: string,
		regex: RegExp,
		message: string = "t.notRegex() failed, using !RegExp.test semantics",
	): void {
		if (regex.test(contents)) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: markup`Expected`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(contents),
					},
					{
						type: "log",
						category: "info",
						text: markup`to not match pattern`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatToString(regex.source),
					},
				],
				1,
			);
		}
	}

	public inlineSnapshot(received: unknown, snapshot?: string | boolean | number) {
		const callFrame = getErrorStructure(new Error()).frames[1];
		const callError = getErrorStructure(new Error(), 1);

		this.onTeardown(async () => {
			const {status} = this.snapshotManager.testInlineSnapshot(
				callFrame,
				received,
				snapshot,
			);

			if (status === "UPDATE" && this.options.freezeSnapshots) {
				await this.emitDiagnostic(
					deriveDiagnosticFromErrorStructure(
						callError,
						{
							description: descriptions.SNAPSHOTS.INLINE_FROZEN,
							cleanFrames,
						},
					),
				);
			}

			if (status === "NO_MATCH") {
				await this.emitDiagnostic(
					deriveDiagnosticFromErrorStructure(
						callError,
						{
							cleanFrames,
							description: {
								...descriptions.SNAPSHOTS.INLINE_BAD_MATCH,
								advice: this.buildMatchAdvice(
									received,
									snapshot,
									{
										receivedAlias: "What the code gave us",
										expectedAlias: "Existing inline snapshot",
									},
								),
							},
						},
					),
				);
			}
		});
	}

	public snapshot(
		expected: unknown,
		message?: string,
		opts?: TestSnapshotOptions,
	): string {
		const id = this.snapshotCounter++;
		return this.bufferSnapshot({
			entryName: String(id),
			expected,
			message,
			opts,
		});
	}

	public namedSnapshot(
		entryName: string,
		expected: unknown,
		message?: string,
		opts?: TestSnapshotOptions,
	): string {
		return this.bufferSnapshot({
			entryName,
			expected,
			message,
			opts,
		});
	}

	private bufferSnapshot(
		{
			entryName,
			message,
			expected,
			opts = {},
		}: SnapshotOptions,
	): string {
		let language: undefined | string = opts.language;

		let formatted = "";
		if (typeof expected === "string") {
			formatted = expected;
		} else {
			// Close enough syntax highlighting to pretty-format
			language = "javascript";
			formatted = prettyFormatToString(expected);
		}

		const callError = getErrorStructure(new Error(), 2);

		this.onTeardown(async () => {
			// Get the current snapshot
			const existingSnapshot = await this.snapshotManager.get(
				this.testName,
				entryName,
				opts.filename,
			);
			if (existingSnapshot === undefined) {
				if (this.options.freezeSnapshots) {
					await this.emitDiagnostic(
						deriveDiagnosticFromErrorStructure(
							callError,
							{
								cleanFrames,
								description: descriptions.SNAPSHOTS.FROZEN,
							},
						),
					);
				} else {
					// No snapshot exists, let's save this one!
					this.snapshotManager.set({
						testName: this.testName,
						entryName,
						value: formatted,
						language,
						optionalFilename: opts.filename,
					});
				}
				return;
			}

			// Compare the snapshots
			const snapshotPath = this.snapshotManager.normalizeSnapshotPath(
				opts.filename,
			);
			if (formatted !== existingSnapshot) {
				const advice: DiagnosticAdvice = this.buildMatchAdvice(
					formatted,
					existingSnapshot,
					{
						receivedAlias: "What the code gave us",
						expectedAlias: "Existing snapshot",
					},
				);

				let markupMessage;

				if (message === undefined) {
					markupMessage = markup`Snapshot <emphasis>"${entryName}"</emphasis> at <emphasis>${snapshotPath}</emphasis> doesn't match`;
				} else {
					markupMessage = markup`${message}`;

					advice.push({
						type: "log",
						category: "info",
						text: markup`Snapshot can be found at <emphasis>${snapshotPath}</emphasis>`,
					});
				}

				advice.push({
					type: "log",
					category: "info",
					text: markup`Run <code>rome test <filelink target="${this.path.join()}" /> --update-snapshots</code> to update this snapshot`,
				});

				await this.emitDiagnostic(
					deriveDiagnosticFromErrorStructure(
						callError,
						{
							cleanFrames,
							description: {
								category: "tests/snapshots/incorrect",
								message: markupMessage,
								advice,
							},
						},
					),
				);
			}
		});

		return entryName;
	}
}
