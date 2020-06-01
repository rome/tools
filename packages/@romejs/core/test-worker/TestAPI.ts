/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	createBlessedDiagnosticMessage,
	createSingleDiagnosticError,
	deriveDiagnosticFromErrorStructure,
	descriptions,
	getErrorStackAdvice,
} from "@romejs/diagnostics";
import SnapshotManager from "./SnapshotManager";
import {TestMasterRunnerOptions} from "../master/testing/types";
import {Event} from "@romejs/events";
import stringDiff from "@romejs/string-diff";
import {getErrorStructure} from "@romejs/v8";
import prettyFormat from "@romejs/pretty-format";
import {FileReference} from "../common/types/files";
import {escapeMarkup, markup} from "@romejs/string-markup";
import {
	AsyncFunc,
	ExpectedError,
	SyncThrower,
	TestDiagnosticAdviceItem,
	TestHelper,
	TestSnapshotOptions,
} from "@romejs-runtime/rome/test";

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

export default class TestAPI implements TestHelper {
	constructor(
		{
			testName,
			onTimeout,
			file,
			snapshotManager,
			options,
			emitDiagnostic,
		}: {
			emitDiagnostic: EmitDiagnostic;
			file: FileReference;
			testName: string;
			onTimeout: OnTimeout;
			snapshotManager: SnapshotManager;
			options: TestMasterRunnerOptions;
		},
	) {
		this.testName = testName;
		this.options = options;
		this.snapshotManager = snapshotManager;
		this.snapshotCounter = 0;
		this.file = file;
		this.teardownEvent = new Event({name: "TestAPI.teardown"});
		this.startTime = Date.now();
		this.onTimeout = onTimeout;
		this.emitDiagnostic = emitDiagnostic;
		this.timeoutMax = 0;
		this.timeoutId = undefined;
		this.setTimeout(5_000);
		this.advice = [];
	}

	startTime: number;
	options: TestMasterRunnerOptions;
	file: FileReference;
	emitDiagnostic: EmitDiagnostic;

	onTimeout: OnTimeout;
	timeoutId: undefined | NodeJS.Timeout;
	timeoutStart: undefined | number;
	timeoutMax: undefined | number;

	advice: DiagnosticAdvice;
	teardownEvent: Event<void, void>;
	testName: string;
	snapshotCounter: number;
	snapshotManager: SnapshotManager;

	buildMatchAdvice(
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
			expectedFormat = prettyFormat(expected);
			receivedFormat = prettyFormat(received);
		}

		const advice: DiagnosticAdvice = [];

		if (expectedFormat === receivedFormat) {
			// Better error message when both values are visually identical
			advice.push({
				type: "log",
				category: "info",
				text: `Both the received and expected values are visually identical`,
			});

			advice.push({
				type: "code",
				code: expectedFormat,
			});

			if (visualMethod !== undefined) {
				advice.push({
					type: "log",
					category: "info",
					text: `Try using t.${visualMethod} if you wanted a visual match`,
				});
			}
		} else {
			const bothSingleLine =
				!expectedFormat.match(/\n/g) && !receivedFormat.match(/\n/g);

			if (!bothSingleLine) {
				advice.push({
					type: "log",
					category: "info",
					text: `Expected to receive`,
				});

				advice.push({
					type: "code",
					code: expectedFormat,
				});

				advice.push({
					type: "log",
					category: "info",
					text: `But got`,
				});

				advice.push({
					type: "code",
					code: receivedFormat,
				});

				advice.push({
					type: "log",
					category: "info",
					text: "Diff",
				});
			}

			advice.push({
				type: "diff",
				diff: stringDiff(expectedFormat, receivedFormat),
				legend: {
					add: receivedAlias ? receivedAlias : "Received",
					delete: expectedAlias ? expectedAlias : "Expected",
				},
			});
		}

		return advice;
	}

	addToAdvice(item: TestDiagnosticAdviceItem): void {
		this.advice.push(item);
	}

	clearAdvice() {
		this.advice = [];
	}

	onTeardown(callback: AsyncFunc): void {
		this.teardownEvent.subscribe(callback);
	}

	clearTimeout(): void {
		if (this.timeoutId !== undefined) {
			clearTimeout(this.timeoutId);
		}

		this.timeoutMax = undefined;
		this.timeoutStart = undefined;
	}

	extendTimeout(time: number): void {
		const {timeoutMax, timeoutStart} = this;
		if (timeoutMax === undefined || timeoutStart === undefined) {
			throw new Error("No timeout set");
		}

		const elapsed = Date.now() - timeoutStart;
		const newTime = timeoutMax - elapsed + time;
		this.setTimeout(newTime);
	}

	setTimeout(time: number): void {
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

	checkTimeout(): void {
		const {startTime, timeoutMax} = this;
		if (timeoutMax === undefined) {
			return;
		}

		const delta = Date.now() - startTime;
		if (delta > timeoutMax) {
			throw new Error(`Test timeout - exceeded ${String(timeoutMax)}ms`);
		}
	}

	fail(
		message: string = "Test failure triggered by t.fail()",
		advice: DiagnosticAdvice = [],
		framesToShift: number = 0,
	): never {
		const diag = deriveDiagnosticFromErrorStructure(
			getErrorStructure(new Error(), framesToShift + 1),
			{
				description: {
					category: "tests/failure",
					message: createBlessedDiagnosticMessage(message),
					advice,
				},
			},
		);
		throw createSingleDiagnosticError(diag);
	}

	truthy(value: unknown, message: string = "Expected value to be truthy"): void {
		if (Boolean(value) === false) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Received`,
					},
					{
						type: "code",
						code: prettyFormat(value),
					},
				],
				1,
			);
		}
	}

	falsy(value: unknown, message: string = "Expected value to be falsy"): void {
		if (Boolean(value) === true) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Received`,
					},
					{
						type: "code",
						code: prettyFormat(value),
					},
				],
				1,
			);
		}
	}

	true(value: unknown, message: string = "Expected value to be true"): void {
		if (value !== true) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Received`,
					},
					{
						type: "code",
						code: prettyFormat(value),
					},
				],
				1,
			);
		}
	}

	false(value: unknown, message: string = "Expected value to be false"): void {
		if (value !== false) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Received`,
					},
					{
						type: "code",
						code: prettyFormat(value),
					},
				],
				1,
			);
		}
	}

	is(
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

	not(
		received: unknown,
		expected: unknown,
		message: string = "t.not() failed, using !Object.is() semantics",
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

	looksLike(
		received: unknown,
		expected: unknown,
		message: string = "t.looksLike() failed, using prettyFormat semantics",
	): void {
		const actualInspect = prettyFormat(received);
		const expectedInspect = prettyFormat(expected);

		if (actualInspect !== expectedInspect) {
			this.fail(message, this.buildMatchAdvice(received, expected), 1);
		}
	}

	notLooksLike(
		received: unknown,
		expected: unknown,
		message: string = "t.notLooksLike() failed, using !prettyFormat semantics",
	): void {
		const actualInspect = prettyFormat(received);
		const expectedInspect = prettyFormat(expected);

		if (actualInspect === expectedInspect) {
			this.fail(message, this.buildMatchAdvice(received, expected), 1);
		}
	}

	throws(
		thrower: SyncThrower,
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
						"Incorrect error stack trace",
					),
					1,
				);
			}
		}

		this.fail(message, undefined, 1);
	}

	async throwsAsync(
		thrower: AsyncFunc,
		expected?: ExpectedError,
		message: string = "t.throws() failed, callback did not throw an error",
	): Promise<void> {
		try {
			await thrower();
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
						"Incorrect error stack trace",
					),
					1,
				);
			}
		}
		this.fail(message, undefined, 1);
	}

	notThrows(
		nonThrower: SyncThrower,
		message: string = "t.notThrows() failed, callback threw an error",
	): void {
		try {
			nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				`t.notThrows did not expect an error to be thrown but got ${err.name}: ${JSON.stringify(
					err.message,
				)}`,
			);
			this.fail(message, advice, 1);
		}
	}

	async notThrowsAsync(
		nonThrower: AsyncFunc,
		message: string = "t.notThrowsAsync failed, callback threw an error",
	): Promise<void> {
		try {
			await nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				`t.notThrowsAsync did not expect an error to be thrown but got ${err.name}: ${JSON.stringify(
					err.message,
				)}`,
			);
			this.fail(message, advice, 1);
		}
	}

	regex(
		contents: string,
		regex: RegExp,
		message: string = "t.regex failed, using RegExp.test semantics",
	): void {
		if (!regex.test(contents)) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Expected`,
					},
					{
						type: "code",
						code: prettyFormat(contents),
					},
					{
						type: "log",
						category: "info",
						text: `to match pattern`,
					},
					{
						type: "code",
						code: prettyFormat(regex.source),
					},
				],
				1,
			);
		}
	}

	notRegex(
		contents: string,
		regex: RegExp,
		message: string = "t.regex failed, using RegExp.test semantics",
	): void {
		if (regex.test(contents)) {
			this.fail(
				message,
				[
					{
						type: "log",
						category: "info",
						text: `Expected`,
					},
					{
						type: "code",
						code: prettyFormat(contents),
					},
					{
						type: "log",
						category: "info",
						text: `to not match pattern`,
					},
					{
						type: "code",
						code: prettyFormat(regex.source),
					},
				],
				1,
			);
		}
	}

	inlineSnapshot(received: unknown, snapshot?: string | boolean | number) {
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
						},
					),
				);
			}

			if (status === "NO_MATCH") {
				await this.emitDiagnostic(
					deriveDiagnosticFromErrorStructure(
						callError,
						{
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

	snapshot(
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

	namedSnapshot(
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

	bufferSnapshot(
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
			language = "javascript";
			formatted = prettyFormat(expected);
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
			if (formatted !== existingSnapshot) {
				const advice: DiagnosticAdvice = this.buildMatchAdvice(
					formatted,
					existingSnapshot,
					{
						receivedAlias: "What the code gave us",
						expectedAlias: "Existing snapshot",
					},
				);

				if (message === undefined) {
					message = `Snapshot ${escapeMarkup(entryName)} at ${this.snapshotManager.defaultSnapshotPath.toMarkup({
						emphasis: true,
					})} doesn't match`;
				} else {
					message = escapeMarkup(message);

					advice.push({
						type: "log",
						category: "info",
						text: `Snapshot can be found at ${this.snapshotManager.defaultSnapshotPath.toMarkup({
							emphasis: true,
						})}`,
					});
				}

				advice.push({
					type: "log",
					category: "info",
					text: markup`Run <command>rome test <filelink target="${this.file.uid}" /> --update-snapshots</command> to update this snapshot`,
				});

				await this.emitDiagnostic(
					deriveDiagnosticFromErrorStructure(
						callError,
						{
							description: {
								category: "tests/snapshots/incorrect",
								message: createBlessedDiagnosticMessage(message),
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
