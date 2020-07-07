/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createAbsoluteFilePath,
} from "@romejs/path";
import {exists, readFileText, removeFile, writeFile} from "@romejs/fs";
import {TestServerRunnerOptions} from "../server/testing/types";
import TestWorkerRunner from "./TestWorkerRunner";
import {DiagnosticDescription, descriptions} from "@romejs/diagnostics";
import {createSnapshotParser} from "./SnapshotParser";
import {ErrorFrame} from "@romejs/v8";
import {Number0, Number1} from "@romejs/ob1";
import prettyFormat from "@romejs/pretty-format";
import {naturalCompare} from "@romejs/string-utils";
import {FilePathLocker} from "../common/utils/lockers";

function cleanHeading(key: string): string {
	if (key[0] === "`") {
		key = key.slice(1);
	}

	if (key[key.length - 1] === "`") {
		key = key.slice(0, -1);
	}

	return key.trim();
}

type SnapshotEntry = {
	testName: string;
	entryName: string;
	language: undefined | string;
	value: string;
	used: boolean;
};

type Snapshot = {
	existsOnDisk: boolean;
	used: boolean;
	raw: string;
	entries: Map<string, SnapshotEntry>;
};

export const SNAPSHOT_EXT = ".test.md";

function buildEntriesKey(testName: string, entryName: string): string {
	return `${testName}#${entryName}`;
}

export type InlineSnapshotUpdate = {
	line: Number1;
	column: Number0;
	snapshot: boolean | number | string | null;
};

export type InlineSnapshotUpdates = Array<InlineSnapshotUpdate>;

export type SnapshotCounts = {
	deleted: number;
	updated: number;
	created: number;
};

function stringOrPrettyFormat(value: unknown): string {
	if (typeof value === "string") {
		return value;
	} else {
		return prettyFormat(value);
	}
}

export default class SnapshotManager {
	constructor(runner: TestWorkerRunner, testPath: AbsoluteFilePath) {
		this.defaultSnapshotPath = testPath.getParent().append(
			`${testPath.getExtensionlessBasename()}${SNAPSHOT_EXT}`,
		);
		this.testPath = testPath;
		this.runner = runner;
		this.options = runner.options;
		this.snapshots = new AbsoluteFilePathMap();
		this.fileLocker = new FilePathLocker();
		this.inlineSnapshotsUpdates = [];
		this.snapshotCounts = {
			deleted: 0,
			updated: 0,
			created: 0,
		};
	}

	inlineSnapshotsUpdates: Array<InlineSnapshotUpdate>;
	testPath: AbsoluteFilePath;
	defaultSnapshotPath: AbsoluteFilePath;
	snapshots: AbsoluteFilePathMap<Snapshot>;
	fileLocker: FilePathLocker;
	runner: TestWorkerRunner;
	options: TestServerRunnerOptions;
	snapshotCounts: SnapshotCounts;

	normalizeSnapshotPath(filename: undefined | string): AbsoluteFilePath {
		if (filename === undefined) {
			return this.defaultSnapshotPath;
		}

		const path = createAbsoluteFilePath(filename);
		const ext = path.getExtensions();
		if (ext.endsWith(SNAPSHOT_EXT)) {
			return path;
		} else {
			return path.addExtension(SNAPSHOT_EXT);
		}
	}

	async init() {
		await this.loadSnapshot(this.defaultSnapshotPath);
	}

	async emitDiagnostic(metadata: DiagnosticDescription) {
		await this.runner.emitDiagnostic({
			description: metadata,
			location: {
				filename: this.defaultSnapshotPath.join(),
			},
		});
	}

	async loadSnapshot(path: AbsoluteFilePath): Promise<undefined | Snapshot> {
		if (!(await exists(path))) {
			return;
		}

		return this.fileLocker.wrapLock(
			path,
			async () => {
				const loadedSnapshot = this.snapshots.get(path);
				if (loadedSnapshot !== undefined) {
					return loadedSnapshot;
				}

				const content = await readFileText(path);
				const parser = createSnapshotParser({
					path,
					input: content,
				});

				const nodes = parser.parse();

				const snapshot: Snapshot = {
					existsOnDisk: true,
					used: false,
					raw: parser.input,
					entries: new Map(),
				};
				this.snapshots.set(path, snapshot);

				while (nodes.length > 0) {
					const node = nodes.shift()!;

					if (node.type === "Heading" && node.level === 1) {
						// Title
						continue;
					}

					if (node.type === "Heading" && node.level === 2) {
						const testName = cleanHeading(node.text);

						while (nodes.length > 0) {
							const node = nodes[0];

							if (node.type === "Heading" && node.level === 3) {
								nodes.shift();

								const entryName = cleanHeading(node.text);

								const codeBlock = nodes.shift();
								if (codeBlock === undefined || codeBlock.type !== "CodeBlock") {
									throw parser.unexpected({
										description: descriptions.SNAPSHOTS.EXPECTED_CODE_BLOCK_AFTER_HEADING,
										loc: node.loc,
									});
								}

								snapshot.entries.set(
									buildEntriesKey(testName, entryName),
									{
										testName,
										entryName,
										language: codeBlock.language,
										value: codeBlock.text,
										used: false,
									},
								);

								continue;
							}

							if (node.type === "CodeBlock") {
								nodes.shift();

								snapshot.entries.set(
									buildEntriesKey(testName, "0"),
									{
										testName,
										entryName: "0",
										language: node.language,
										value: node.text,
										used: false,
									},
								);
							}

							break;
						}

						continue;
					}
				}
				return snapshot;
			},
		);
	}

	buildSnapshot(entries: Iterable<SnapshotEntry>): Array<string> {
		// Build the snapshot
		let lines: Array<string> = [];

		function pushNewline() {
			if (lines[lines.length - 1] !== "") {
				lines.push("");
			}
		}

		lines.push(`# \`${this.testPath.getBasename()}\``);
		pushNewline();
		const relativeTestPath = this.runner.projectFolder.relative(this.testPath).join();
		lines.push(
			`**DO NOT MODIFY**. This file has been autogenerated. Run \`rome test ${relativeTestPath} --update-snapshots\` to update.`,
		);
		pushNewline();

		const testNameToEntries: Map<string, Map<string, SnapshotEntry>> = new Map();
		for (const entry of entries) {
			if (!entry.used && !this.runner.hasFocusedTests) {
				continue;
			}
			let entriesByTestName = testNameToEntries.get(entry.testName);
			if (entriesByTestName === undefined) {
				entriesByTestName = new Map();
				testNameToEntries.set(entry.testName, entriesByTestName);
			}
			entriesByTestName.set(entry.entryName, entry);
		}

		// Get test names and sort them so they are in a predictable
		const testNames = Array.from(testNameToEntries.keys()).sort();

		for (const testName of testNames) {
			const entries = testNameToEntries.get(testName)!;

			lines.push(`## \`${testName}\``);
			pushNewline();
			const entryNames = Array.from(entries.keys()).sort(naturalCompare);

			for (const snapshotName of entryNames) {
				const entry = entries.get(snapshotName)!;

				const {value} = entry;
				const language = entry.language === undefined ? "" : entry.language;

				// If the test only has one snapshot then omit the heading
				const skipHeading = snapshotName === "0" && entryNames.length === 1;
				if (!skipHeading) {
					lines.push(`### \`${snapshotName}\``);
				}

				pushNewline();
				lines.push("```" + language);
				// TODO escape triple backquotes
				lines.push(value);
				lines.push("```");
				pushNewline();
			}
		}
		return lines;
	}

	async save() {
		// If there's a focused test then we don't write or validate a snapshot
		if (this.runner.hasFocusedTests) {
			return;
		}

		const {hasDiagnostics} = this.runner;

		for (const [path, {used, existsOnDisk, raw, entries}] of this.snapshots) {
			const lines = this.buildSnapshot(entries.values());
			const formatted = lines.join("\n");

			if (this.options.freezeSnapshots) {
				if (used) {
					if (formatted !== raw) {
						await this.emitDiagnostic(
							descriptions.SNAPSHOTS.INCORRECT(raw, formatted),
						);
					}
				} else {
					await this.emitDiagnostic(descriptions.SNAPSHOTS.REDUNDANT);
				}
			} else {
				// Don't delete or write a snapshot if there are test failures as those failures may be hiding snapshot usages
				if (!hasDiagnostics) {
					continue;
				}

				if (existsOnDisk && !used) {
					// If a snapshot wasn't used or is empty then delete it!
					await removeFile(path);
					this.snapshotCounts.deleted++;
				} else if (used && formatted !== raw) {
					// Fresh snapshot!
					await writeFile(path, formatted);
					if (existsOnDisk) {
						this.snapshotCounts.updated++;
					} else {
						this.snapshotCounts.created++;
					}
				}
			}
		}
	}

	testInlineSnapshot(
		callFrame: ErrorFrame,
		received: unknown,
		expected?: InlineSnapshotUpdate["snapshot"],
	): {
		status: "MATCH" | "NO_MATCH" | "UPDATE";
	} {
		let receivedFormat = stringOrPrettyFormat(received);
		let expectedFormat = stringOrPrettyFormat(expected);

		// Matches, no need to do anything
		if (receivedFormat === expectedFormat) {
			return {status: "MATCH"};
		}

		const shouldSave = this.options.updateSnapshots || expected === undefined;
		if (shouldSave) {
			const {lineNumber, columnNumber} = callFrame;
			if (lineNumber === undefined || columnNumber === undefined) {
				throw new Error("Call frame has no line or column");
			}

			if (!this.options.freezeSnapshots) {
				let snapshot: InlineSnapshotUpdate["snapshot"] = receivedFormat;
				if (
					typeof received === "string" ||
					typeof received === "number" ||
					typeof received === "boolean" ||
					received === null
				) {
					snapshot = received;
				}

				this.inlineSnapshotsUpdates.push({
					line: lineNumber,
					column: columnNumber,
					snapshot,
				});
			}

			return {status: "UPDATE"};
		}

		return {status: "NO_MATCH"};
	}

	async get(
		testName: string,
		entryName: string,
		optionalFilename: undefined | string,
	): Promise<undefined | string> {
		const snapshotPath = this.normalizeSnapshotPath(optionalFilename);
		let snapshot = this.snapshots.get(snapshotPath);

		if (snapshot === undefined) {
			snapshot = await this.loadSnapshot(snapshotPath);
		}

		if (snapshot === undefined) {
			return undefined;
		}

		snapshot.used = true;

		// If we're force updating, pretend that there was no entry
		if (this.options.updateSnapshots) {
			return undefined;
		}

		const entry = snapshot.entries.get(buildEntriesKey(testName, entryName));
		if (entry === undefined) {
			return undefined;
		} else {
			entry.used = true;
			return entry.value;
		}
	}

	set(
		{
			testName,
			entryName,
			value,
			language,
			optionalFilename,
		}: {
			testName: string;
			entryName: string;
			value: string;
			language: undefined | string;
			optionalFilename: undefined | string;
		},
	) {
		const snapshotPath = this.normalizeSnapshotPath(optionalFilename);
		let snapshot = this.snapshots.get(snapshotPath);
		if (snapshot === undefined) {
			snapshot = {
				raw: "",
				existsOnDisk: false,
				used: true,
				entries: new Map(),
			};
			this.snapshots.set(snapshotPath, snapshot);
		}

		snapshot.entries.set(
			buildEntriesKey(testName, entryName),
			{
				testName,
				entryName,
				language,
				value,
				used: true,
			},
		);
	}
}
