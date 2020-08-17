import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import {TestWorkerFileResult} from "@internal/core/test-worker/TestWorkerFile";
import SnapshotManager, {
	InlineSnapshotUpdates,
	Snapshot,
} from "@internal/core/test-worker/SnapshotManager";
import {BundleResult, FileReference, ServerRequest} from "@internal/core";
import TestServer from "@internal/core/server/testing/TestServer";
import {DiagnosticLocation, descriptions} from "@internal/diagnostics";
import {removeFile} from "@internal/fs";

export default class TestServerFile {
	constructor(
		{ref, bundle, runner, request}: {
			ref: FileReference;
			bundle: BundleResult;
			runner: TestServer;
			request: ServerRequest;
		},
	) {
		this.request = request;
		this.runner = runner;
		this.path = ref.real;
		this.ref = ref;
		this.bundle = bundle;

		this.hasDiagnostics = false;
		this.pendingTests = new Set();
		this.snapshots = new AbsoluteFilePathMap();
		this.inlineSnapshotUpdates = [];
	}

	public bundle: BundleResult;
	public path: AbsoluteFilePath;
	public ref: FileReference;

	private hasDiagnostics: boolean;
	private pendingTests: Set<string>;
	private request: ServerRequest;
	private runner: TestServer;
	private inlineSnapshotUpdates: InlineSnapshotUpdates;
	private snapshots: AbsoluteFilePathMap<Snapshot>;

	public onDiagnostics() {
		this.hasDiagnostics = true;
	}

	public getPendingTests(): Set<string> {
		return this.pendingTests;
	}

	public removePendingTest(testName: string) {
		this.pendingTests.delete(testName);
	}

	public addPendingTest(testName: string) {
		this.pendingTests.add(testName);
	}

	public clearPendingTests() {
		this.pendingTests.clear();
	}

	public addResult(result: TestWorkerFileResult) {
		for (const [path, snapshot] of result.snapshots) {
			const existing = this.snapshots.get(path);
			if (existing === undefined) {
				this.snapshots.set(path, snapshot);
				continue;
			}

			const mergedEntries: Snapshot["entries"] = new Map(existing.entries);

			for (const [name, entry] of snapshot.entries) {
				const existing = mergedEntries.get(name);
				if (existing === undefined || entry.used) {
					mergedEntries.set(name, entry);
				}
			}

			const merged: Snapshot = {
				// This should be the same. Maybe validate?
				existsOnDisk: existing.existsOnDisk,
				raw: existing.raw,
				used: existing.used || snapshot.used,
				entries: mergedEntries,
			};
			this.snapshots.set(path, merged);
		}

		this.inlineSnapshotUpdates = [
			...this.inlineSnapshotUpdates,
			...result.inlineSnapshotUpdates,
		];
	}

	private async writeSnapshots() {
		const {snapshots, runner, request} = this;
		if (snapshots.size === 0) {
			return;
		}

		const {hasDiagnostics} = this;
		const {processor} = runner.printer;

		for (const [path, {used, existsOnDisk, raw, entries}] of this.snapshots) {
			const formatted = SnapshotManager.buildSnapshot({
				entries: Array.from(entries.values()),
				absolute: this.path,
				relative: this.ref.relative,
			});

			const location: DiagnosticLocation = {
				filename: path.join(),
			};

			if (runner.options.freezeSnapshots) {
				if (used) {
					if (formatted !== raw) {
						processor.addDiagnostic({
							description: descriptions.SNAPSHOTS.INCORRECT(raw, formatted),
							location,
						});
					}
				} else {
					processor.addDiagnostic({
						description: descriptions.SNAPSHOTS.REDUNDANT,
						location,
					});
				}
			} else {
				// Don't delete or write a snapshot if there are test failures as those failures may be hiding snapshot usages
				if (hasDiagnostics && !runner.options.updateSnapshots) {
					continue;
				}

				if (existsOnDisk && !used) {
					// If a snapshot wasn't used or is empty then delete it!
					await removeFile(path);
					runner.progress.deletedSnapshots++;
				} else if (used && formatted !== raw) {
					// Fresh snapshot!
					request.queueSaveFile(
						path,
						{
							type: "UNSAFE_WRITE",
							content: formatted,
						},
					);
					if (existsOnDisk) {
						runner.progress.updatedSnapshots++;
					} else {
						runner.progress.createdSnapshots++;
					}
				}
			}
		}
	}

	private async writeInlineSnapshots() {
		let {inlineSnapshotUpdates, path, ref, request, runner} = this;
		if (inlineSnapshotUpdates.length === 0) {
			return;
		}

		const filename = path.join();

		// Resolve source maps. These will originally be pointed to the compiled source.
		inlineSnapshotUpdates = inlineSnapshotUpdates.map((update) => {
			const resolved = runner.sourceMaps.assertApproxOriginalPositionFor(
				filename,
				update.line,
				update.column,
			);

			if (!resolved.found) {
				throw new Error("Could not find inline snapshot location in source map");
			}

			if (resolved.source !== filename && resolved.source !== ref.uid) {
				throw new Error(
					`Inline snapshot update resolved to ${resolved.source} when it should be ${filename}`,
				);
			}

			return {
				...update,
				line: resolved.line,
				column: resolved.column,
			};
		});

		const {diagnostics, file} = await request.requestWorkerUpdateInlineSnapshots(
			path,
			inlineSnapshotUpdates,
			{},
		);
		runner.printer.processor.addDiagnostics(diagnostics);
		if (file !== undefined) {
			request.queueSaveFile(path, file);
			runner.progress.updatedInlineSnapshots++;
		}
	}

	public async finish() {
		await this.writeSnapshots();
		await this.writeInlineSnapshots();
	}
}
