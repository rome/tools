/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DIAGNOSTIC_CATEGORIES,
	Diagnostic,
	DiagnosticAdvice,
	createSingleDiagnosticsError,
	descriptions,
	getErrorStackAdvice,
} from "@internal/diagnostics";
import SnapshotManager from "./SnapshotManager";
import {TestServerRunnerOptions} from "../../server/testing/types";
import {Event} from "@internal/events";
import {getErrorStructure} from "@internal/errors";
import {
	normalizeLimitProperties,
	prettyFormatToString,
} from "@internal/pretty-format";
import {markup} from "@internal/markup";
import {
	ExpectedError,
	TestDiagnosticAdvice,
	TestHelper,
	TestSnapshotHelper,
	TestSnapshotOptions,
} from "@internal/virtual-rome/test";
import TestWorkerFile, {TestDetails} from "./TestWorkerFile";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import {DurationMeasurer} from "@internal/numbers";
import {stringDiffCompressed} from "@internal/string-diff";

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

function prettyFormatUntrusted(value: unknown): string {
	return prettyFormatToString(value, {accurate: true});
}

export type OnTimeout = (time: number) => void;

type SnapshotOptions = {
	entryName: string;
	expected: unknown;
	filename?: string;
	opts?: TestSnapshotOptions;
};

function normalizeUserAdvice(advice: UserAdviceItem[]): DiagnosticAdvice[] {
	return advice.map((item) => {
		if (typeof item === "function") {
			return normalizeUserAdviceItem(item());
		} else {
			return normalizeUserAdviceItem(item);
		}
	});
}

// Once we have a public test framework we should perform normalization here
function normalizeUserAdviceItem(item: TestDiagnosticAdvice): DiagnosticAdvice {
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

		case "inspect":
			return {
				type: "inspect",
				// rome-ignore lint/ts/noExplicitAny: can remove this when we add back diagnostics structure validation
				data: item.data as any,
			};

		default:
			return item;
	}
}

type UserAdviceItem = TestDiagnosticAdvice | (() => TestDiagnosticAdvice);

type DiagnosticAdviceFactory = () => DiagnosticAdvice[];

export default class TestAPI {
	constructor(
		file: TestWorkerFile,
		testDetails: TestDetails,
		onTimeout: OnTimeout,
	) {
		this.testDetails = testDetails;
		this.options = file.globalOptions;
		this.snapshotManager = file.snapshotManager;
		this.file = file;
		this.teardownEvent = new Event("TestAPI.teardown");
		this.startTime = new DurationMeasurer();
		this.snapshotCounter = 0;
		this.onTimeout = onTimeout;
		this.timeoutMax = 0;
		this.timeoutId = undefined;
		this.setTimeout(5_000);
		this.logAdvice = [];
		this.userAdvice = [];
	}

	public teardownEvent: Event<void, void>;

	private snapshotCounter: number;
	private file: TestWorkerFile;
	private startTime: DurationMeasurer;
	private options: TestServerRunnerOptions;

	private onTimeout: OnTimeout;
	private timeoutId: undefined | NodeJS.Timeout;
	private timeoutStart: undefined | DurationMeasurer;
	private timeoutMax: undefined | number;

	private userAdvice: UserAdviceItem[];
	private logAdvice: DiagnosticAdviceFactory[];
	private testDetails: TestDetails;
	private snapshotManager: SnapshotManager;

	public async emitDiagnostic(diag: Diagnostic): Promise<void> {
		return this.file.emitDiagnostic(
			{
				...diag,
				description: {
					...diag.description,
					advice: this.getAdvice(diag.description.advice),
				},
			},
			this.testDetails,
		);
	}

	public getAdvice(startAdvice: DiagnosticAdvice[] = []): DiagnosticAdvice[] {
		const {userAdvice, logAdvice} = this;
		if (userAdvice.length === 0 && logAdvice.length === 0) {
			return startAdvice;
		}

		const advice: DiagnosticAdvice[] = [...startAdvice];

		if (userAdvice.length > 0) {
			advice.push({
				type: "group",
				title: markup`User-specified test advice`,
				advice: normalizeUserAdvice(userAdvice),
			});
		}

		if (logAdvice.length > 0) {
			let flatLogAdvice: DiagnosticAdvice[] = [];
			for (const factory of logAdvice) {
				flatLogAdvice = [...flatLogAdvice, ...factory()];
			}
			advice.push({
				type: "group",
				title: markup`Console logs`,
				advice: flatLogAdvice,
			});
		}

		return advice;
	}

	private buildMatchAdvice(
		received: unknown,
		expected: unknown,
		{
			visualMethod,
			expectedAlias,
			receivedAlias,
			expectedFormat,
			receivedFormat,
		}: {
			visualMethod?: string;
			expectedAlias?: string;
			receivedAlias?: string;
			expectedFormat?: string;
			receivedFormat?: string;
		} = {},
	): DiagnosticAdvice[] {
		if (expectedFormat === undefined || receivedFormat === undefined) {
			if (typeof received === "string" && typeof expected === "string") {
				expectedFormat = expected;
				receivedFormat = received;
			} else {
				expectedFormat = prettyFormatUntrusted(expected);
				receivedFormat = prettyFormatUntrusted(received);
			}
		}

		const advice: DiagnosticAdvice[] = [];

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
				diff: stringDiffCompressed(expectedFormat, receivedFormat),
				legend: {
					add: receivedAlias ? receivedAlias : "Received",
					delete: expectedAlias ? expectedAlias : "Expected",
				},
			});
		}

		return advice;
	}

	public addToLogAdvice(item: DiagnosticAdviceFactory): void {
		this.logAdvice.push(item);
	}

	private bufferSnapshot(
		{
			entryName,
			filename,
			expected,
			opts = {},
		}: SnapshotOptions,
	): string {
		let language: undefined | string = opts.language;

		let formatted = this.snapshotManager.formatValue(expected);
		if (typeof expected !== "string") {
			language = "javascript";
		}

		const callError = getErrorStructure(new Error(), 2);

		this.onTeardown(async () => {
			// Get the current snapshot
			const existingSnapshot = await this.snapshotManager.get(
				this.testDetails.name,
				entryName,
				filename,
			);
			if (existingSnapshot === undefined) {
				if (this.options.freezeSnapshots) {
					await this.emitDiagnostic(
						this.file.deriveDiagnosticFromErrorStructure(
							callError,
							{
								description: descriptions.SNAPSHOTS.FROZEN,
							},
						),
					);
				} else {
					// No snapshot exists, let's save this one!
					this.snapshotManager.set({
						testName: this.testDetails.name,
						entryName,
						value: formatted,
						language,
						optionalFilename: filename,
					});
				}
				return;
			}

			// Compare the snapshots
			const snapshotPath = this.snapshotManager.normalizeSnapshotPath(filename);
			if (formatted !== existingSnapshot) {
				const advice: DiagnosticAdvice[] = this.buildMatchAdvice(
					formatted,
					existingSnapshot,
					{
						receivedAlias: "What the code gave us",
						expectedAlias: "Existing snapshot",
					},
				);

				let markupMessage;

				if (opts.message === undefined) {
					markupMessage = markup`Snapshot <emphasis>"${entryName}"</emphasis> at <emphasis>${snapshotPath}</emphasis> doesn't match`;
				} else {
					markupMessage = markup`${opts.message}`;

					advice.push({
						type: "log",
						category: "info",
						text: markup`Snapshot can be found at <emphasis>${snapshotPath}</emphasis>`,
					});
				}

				await this.emitDiagnostic(
					this.file.deriveDiagnosticFromErrorStructure(
						callError,
						{
							description: {
								category: DIAGNOSTIC_CATEGORIES["tests/snapshots/incorrect"],
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

	// We don't want to expose an internal instance to userland tests that could allow for private property escape
	public getUserSafeHelper(): TestHelper {
		return Object.freeze({
			addToAdvice: this.addToAdvice.bind(this),
			clearAdvice: this.clearAdvice.bind(this),
			onTeardown: this.onTeardown.bind(this),
			clearTimeout: this.clearTimeout.bind(this),
			extendTimeout: this.extendTimeout.bind(this),
			setTimeout: this.setTimeout.bind(this),
			checkTimeout: this.checkTimeout.bind(this),
			truthy: this.truthy.bind(this),
			falsy: this.falsy.bind(this),
			true: this.true.bind(this),
			false: this.false.bind(this),
			is: this.is.bind(this),
			not: this.not.bind(this),
			deepEquals: this.deepEquals.bind(this),
			notDeepEquals: this.notDeepEquals.bind(this),
			looksLike: this.looksLike.bind(this),
			notLooksLike: this.notLooksLike.bind(this),
			throws: this.throws.bind(this),
			throwsAsync: this.throwsAsync.bind(this),
			notThrows: this.notThrows.bind(this),
			notThrowsAsync: this.notThrowsAsync.bind(this),
			regex: this.regex.bind(this),
			notRegex: this.notRegex.bind(this),
			snapshot: this.snapshot.bind(this),
			inlineSnapshot: this.inlineSnapshot.bind(this),
			namedSnapshot: this.namedSnapshot.bind(this),
			customSnapshot: this.customSnapshot.bind(this),
		});
	}

	// We allow lazy construction of test advice when an error actually occurs
	private addToAdvice(item: UserAdviceItem): void {
		this.userAdvice.push(item);
	}

	private clearAdvice() {
		this.userAdvice = [];
	}

	private onTeardown(callback: AsyncVoidCallback): void {
		this.teardownEvent.subscribe(callback);
	}

	public clearTimeout(): void {
		if (this.timeoutId !== undefined) {
			clearTimeout(this.timeoutId);
		}

		this.timeoutMax = undefined;
		this.timeoutStart = undefined;
	}

	private extendTimeout(time: number): void {
		const {timeoutMax, timeoutStart} = this;
		if (timeoutMax === undefined || timeoutStart === undefined) {
			throw new Error("No timeout set");
		}

		const elapsed = timeoutStart.since().toMilliseconds();
		const newTime = timeoutMax - elapsed + time;
		this.setTimeout(newTime);
	}

	private setTimeout(time: number): void {
		this.clearTimeout();

		this.timeoutStart = new DurationMeasurer();
		this.timeoutMax = time;

		this.timeoutId = setTimeout(
			() => {
				this.onTimeout(time);
			},
			time,
		);
	}

	private checkTimeout(): void {
		const {startTime, timeoutMax} = this;
		if (timeoutMax === undefined) {
			return;
		}

		const delta = startTime.since().toMilliseconds();
		if (delta > timeoutMax) {
			throw new Error(`Test timeout - exceeded ${String(timeoutMax)}ms`);
		}
	}

	private fail(
		{message, methodName, advice = []}: {
			message: string;
			methodName: string;
			advice?: DiagnosticAdvice[];
		},
	): never {
		const diag = this.file.deriveDiagnosticFromErrorStructure(
			getErrorStructure(new Error(), 2),
			{
				description: {
					category: DIAGNOSTIC_CATEGORIES["tests/failure"],
					categoryValue: methodName,
					message: markup`${message}`,
					advice,
				},
			},
		);
		throw createSingleDiagnosticsError(diag);
	}

	private truthy(
		value: unknown,
		message: string = "Expected value to be truthy",
	): void {
		if (Boolean(value) === false) {
			this.fail({
				methodName: "truthy",
				message,
				advice: [
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
			});
		}
	}

	private falsy(
		value: unknown,
		message: string = "Expected value to be falsy",
	): void {
		if (Boolean(value) === true) {
			this.fail({
				message,
				methodName: "falsy",
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(value),
					},
				],
			});
		}
	}

	private true(
		value: boolean,
		message: string = "Expected value to be true",
	): void {
		if (!value) {
			this.fail({
				message,
				methodName: "truthy",
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(value),
					},
				],
			});
		}
	}

	private false(
		value: boolean,
		message: string = "Expected value to be false",
	): void {
		if (value) {
			this.fail({
				message,
				methodName: "false",
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Received`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(value),
					},
				],
			});
		}
	}

	private is<T extends unknown>(
		received: T,
		expected: T,
		message: string = "Expected values to match with Object.is() semantics",
	): void {
		if (Object.is(received, expected) !== true) {
			this.fail({
				message,
				methodName: "is",
				advice: this.buildMatchAdvice(
					received,
					expected,
					{
						visualMethod: "looksLike",
					},
				),
			});
		}
	}

	private not(
		received: unknown,
		expected: unknown,
		message: string = "Expected values to be different but were the same with Object.is() semantics",
	): void {
		if (Object.is(received, expected) === true) {
			this.fail({
				message,
				methodName: "not",
				advice: this.buildMatchAdvice(
					received,
					expected,
					{
						visualMethod: "notLooksLike",
					},
				),
			});
		}
	}

	private deepEquals<T extends unknown>(
		received: T,
		expected: T,
		message: string = "Expected values to match deeply",
	): void {
		const actualInspect = prettyFormatToString(received);
		const expectedInspect = prettyFormatToString(expected);

		if (actualInspect !== expectedInspect) {
			this.fail({
				methodName: "deepEquals",
				message,
				advice: this.buildMatchAdvice(received, expected),
			});
		}
	}

	private notDeepEquals(
		received: unknown,
		expected: unknown,
		message: string = "Expected values not to match deeply",
	): void {
		const actualInspect = prettyFormatToString(received);
		const expectedInspect = prettyFormatToString(expected);

		if (actualInspect === expectedInspect) {
			this.fail({
				methodName: "notDeepEquals",
				message,
				advice: this.buildMatchAdvice(received, expected),
			});
		}
	}

	private looksLike<T extends unknown>(
		received: T,
		expected: T,
		message: string = "Expected values to look-alike but they do not",
	): void {
		const receivedFormat = prettyFormatToString(
			received,
			{limitProperties: normalizeLimitProperties(expected)},
		);
		const expectedFormat = prettyFormatToString(expected);

		if (receivedFormat !== expectedFormat) {
			this.fail({
				methodName: "looksLike",
				message,
				advice: this.buildMatchAdvice(
					received,
					expected,
					{
						expectedFormat,
						receivedFormat,
					},
				),
			});
		}
	}

	private notLooksLike(
		received: unknown,
		expected: unknown,
		message: string = "Expected values to look different but they look-alike",
	): void {
		const receivedFormat = prettyFormatToString(
			received,
			{limitProperties: normalizeLimitProperties(expected)},
		);
		const expectedFormat = prettyFormatToString(expected);

		if (receivedFormat === expectedFormat) {
			this.fail({
				message,
				methodName: "notLooksLike",
				advice: this.buildMatchAdvice(
					received,
					expected,
					{
						expectedFormat,
						receivedFormat,
					},
				),
			});
		}
	}

	private throws(
		thrower: VoidCallback,
		expected?: ExpectedError,
		message: string = "Expected an error to be thrown but none were",
	): void {
		try {
			thrower();
		} catch (err) {
			if (matchExpectedError(err, expected)) {
				return undefined;
			} else {
				this.fail({
					message: `Expected an error to be thrown that matches ${formatExpectedError(
						expected,
					)} but got ${err.name}: ${JSON.stringify(err.message)}`,
					methodName: "throws",
					advice: getErrorStackAdvice(
						getErrorStructure(err),
						{
							title: markup`Incorrect error stack trace`,
						},
					),
				});
			}
		}

		this.fail({message, methodName: "throws"});
	}

	private async throwsAsync(
		thrower: AsyncVoidCallback,
		expected?: ExpectedError,
		message: string = "Expected an error to be thrown but none were",
	): Promise<void> {
		try {
			await thrower();
		} catch (err) {
			if (matchExpectedError(err, expected)) {
				return undefined;
			} else {
				this.fail({
					message: `Expected an error to be thrown that matches ${formatExpectedError(
						expected,
					)} but got ${err.name}: ${JSON.stringify(err.message)}`,
					methodName: "throws",
					advice: getErrorStackAdvice(
						getErrorStructure(err),
						{
							title: markup`Incorrect error stack trace`,
						},
					),
				});
			}
		}
		this.fail({message, methodName: "throws"});
	}

	private notThrows(
		nonThrower: VoidCallback,
		message: string = "Did not expect an error to be thrown but one was",
	): void {
		try {
			nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				{
					title: markup`Thrown error ${err.name}: ${JSON.stringify(err.message)}`,
				},
			);
			this.fail({message, methodName: "notThrows", advice});
		}
	}

	private async notThrowsAsync(
		nonThrower: AsyncVoidCallback,
		message: string = "Did not expect an error to be thrown but one was",
	): Promise<void> {
		try {
			await nonThrower();
		} catch (err) {
			const advice = getErrorStackAdvice(
				getErrorStructure(err),
				{
					title: markup`Thrown error ${err.name}: ${JSON.stringify(err.message)}`,
				},
			);
			this.fail({message, methodName: "notThrows", advice});
		}
	}

	private regex(
		contents: string,
		regex: RegExp,
		message: string = "Expected string to match regex",
	): void {
		if (!regex.test(contents)) {
			this.fail({
				message,
				methodName: "regex",
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Expected`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(contents),
					},
					{
						type: "log",
						category: "info",
						text: markup`to match pattern`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(regex.source),
					},
				],
			});
		}
	}

	private notRegex(
		contents: string,
		regex: RegExp,
		message: string = "Expected string not to match regex",
	): void {
		if (regex.test(contents)) {
			this.fail({
				message,
				methodName: "notRegex",
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Expected`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(contents),
					},
					{
						type: "log",
						category: "info",
						text: markup`to not match pattern`,
					},
					{
						type: "code",
						language: "unknown",
						sourceText: prettyFormatUntrusted(regex.source),
					},
				],
			});
		}
	}

	private inlineSnapshot(
		received: unknown,
		snapshot?: string | boolean | number,
	) {
		const callFrame = getErrorStructure(new Error()).frames[1];
		const callError = getErrorStructure(new Error(), 1);

		this.onTeardown(async () => {
			const res = this.snapshotManager.testInlineSnapshot(
				callFrame,
				received,
				snapshot,
			);

			if (res.status === "UPDATE" && this.options.freezeSnapshots) {
				await this.emitDiagnostic(
					this.file.deriveDiagnosticFromErrorStructure(
						callError,
						{
							description: descriptions.SNAPSHOTS.INLINE_FROZEN,
						},
					),
				);
			}

			if (res.status === "NO_MATCH") {
				await this.emitDiagnostic(
					this.file.deriveDiagnosticFromErrorStructure(
						callError,
						{
							description: {
								...descriptions.SNAPSHOTS.INLINE_BAD_MATCH,
								advice: this.buildMatchAdvice(
									res.receivedFormat,
									res.expectedFormat,
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

	private snapshot(expected: unknown, opts?: TestSnapshotOptions): string {
		const id = this.snapshotCounter++;
		return this.bufferSnapshot({
			entryName: String(id),
			expected,
			opts,
		});
	}

	private customSnapshot(
		filename: string,
		defaultOpts?: TestSnapshotOptions,
	): TestSnapshotHelper {
		return {
			snapshot: (expected, opts) => {
				const id = this.snapshotCounter++;
				return this.bufferSnapshot({
					entryName: String(id),
					expected,
					filename,
					opts: {
						...defaultOpts,
						...opts,
					},
				});
			},
			named: (entryName, expected, opts) => {
				return this.bufferSnapshot({
					entryName,
					expected,
					filename,
					opts: {
						...defaultOpts,
						...opts,
					},
				});
			},
		};
	}

	private namedSnapshot(
		entryName: string,
		expected: unknown,
		opts?: TestSnapshotOptions,
	): string {
		return this.bufferSnapshot({
			entryName,
			expected,
			opts,
		});
	}
}
