import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	RelativePath,
} from "@internal/path";
import {TestConsoleAdvice} from "@internal/core/worker/test/TestWorkerFile";
import SnapshotManager, {
	InlineSnapshotUpdates,
	SnapshotEntry,
} from "@internal/core/worker/test/SnapshotManager";
import {BundleResult, FileReference, ServerRequest} from "@internal/core";
import TestServer from "@internal/core/server/testing/TestServer";
import {
	DiagnosticAdvice,
	DiagnosticLocation,
	descriptions,
} from "@internal/diagnostics";
import {pretty} from "@internal/pretty-format";
import TestServerWorker from "./TestServerWorker";

type SnapshotWriteOptions = {
	path: AbsoluteFilePath;
	existsOnDisk: boolean;
	used: boolean;
	location: DiagnosticLocation;
	formatted: string;
};

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

		// Test file relative to the project
		this.relative = request.server.projectManager.assertProjectExisting(
			ref.real,
		).directory.relativeForce(ref.real);

		this.hasDiagnostics = false;
		this.finishedTests = 0;
		this.totalTests = 0;
		this.pendingTests = new Set();
		this.snapshots = new AbsoluteFilePathMap();
		this.diskSnapshotsToWorker = new AbsoluteFilePathMap();
		this.inlineSnapshotUpdates = [];
		this.workers = new Set();
	}

	public bundle: BundleResult;
	public path: AbsoluteFilePath;
	public ref: FileReference;

	private workers: Set<TestServerWorker>;
	private finishedTests: number;
	private totalTests: number;
	private relative: RelativePath;
	private hasDiagnostics: boolean;
	private pendingTests: Set<string>;
	private request: ServerRequest;
	private runner: TestServer;
	private inlineSnapshotUpdates: InlineSnapshotUpdates;
	private snapshots: AbsoluteFilePathMap<SnapshotEntry[]>;
	private diskSnapshotsToWorker: AbsoluteFilePathMap<TestServerWorker>;

	public onDiagnostics() {
		this.hasDiagnostics = true;
	}

	public getPendingTests(): Set<string> {
		return this.pendingTests;
	}

	public removePendingTest(testName: string, worker: TestServerWorker) {
		this.pendingTests.delete(testName);
		this.workers.add(worker);
	}

	public addPendingTest(testName: string) {
		this.totalTests++;
		this.pendingTests.add(testName);
	}

	public markCompletedTest(): boolean {
		this.finishedTests++;
		return this.finishedTests === this.totalTests;
	}

	public clearPendingTests() {
		this.totalTests = 0;
		this.pendingTests.clear();
	}

	public discoveredDiskSnapshot(
		snapshotPath: AbsoluteFilePath,
		worker: TestServerWorker,
	) {
		if (!this.snapshots.has(snapshotPath)) {
			this.snapshots.set(snapshotPath, []);
		}
		this.diskSnapshotsToWorker.set(snapshotPath, worker);
	}

	public addSnapshotEntry(snapshotPath: AbsoluteFilePath, entry: SnapshotEntry) {
		let entries = this.snapshots.get(snapshotPath);
		if (entries === undefined) {
			entries = [];
			this.snapshots.set(snapshotPath, entries);
		}
		entries.push(entry);
	}

	public addInlineSnapshotUpdates(updates: InlineSnapshotUpdates) {
		this.inlineSnapshotUpdates = [...this.inlineSnapshotUpdates, ...updates];
	}

	private async getRawSnapshot(snapshotPath: AbsoluteFilePath): Promise<string> {
		const worker = this.diskSnapshotsToWorker.assert(snapshotPath);
		return worker.bridge.events.testGetRawSnapshot.call({
			snapshotPath,
			path: this.path,
		});
	}

	private async writeSnapshots() {
		const {snapshots, runner} = this;
		if (snapshots.size === 0) {
			return;
		}

		// Could be hiding tests that use snapshots
		if (runner.hasFocusedTests()) {
			return;
		}

		for (const [path, usedEntries] of this.snapshots) {
			const existsOnDisk = this.diskSnapshotsToWorker.has(path);
			const used = usedEntries.length > 0;

			const formatted = SnapshotManager.buildSnapshot({
				entries: Array.from(usedEntries.values()),
				absolute: this.path,
				relative: this.relative,
			});

			const opts: SnapshotWriteOptions = {
				existsOnDisk,
				used,
				formatted,
				path,
				location: {
					path,
				},
			};

			if (runner.options.freezeSnapshots) {
				await this.writeFrozenSnapshot(opts);
			} else {
				await this.writeSnapshot(opts);
			}
		}

		// Clear the memory as snapshots can be huge
		this.snapshots.clear();

		// Perform actual writes
		await this.request.flushFiles();
	}

	private async writeSnapshot(
		{path, used, existsOnDisk, formatted}: SnapshotWriteOptions,
	): Promise<void> {
		const {runner, request} = this;

		// Don't delete or write a snapshot if there are test failures as those failures may be hiding snapshot usages
		if (this.hasDiagnostics && !runner.options.updateSnapshots) {
			return;
		}

		if (!used) {
			// If a snapshot wasn't used or is empty then delete it!
			if (existsOnDisk) {
				await path.removeFile();
				runner.progress.deletedSnapshots++;
			}
			return;
		}

		if (existsOnDisk) {
			const raw = await this.getRawSnapshot(path);
			if (raw === formatted) {
				// Already up to date
				return;
			}
		}

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

	private async writeFrozenSnapshot(
		{path, used, existsOnDisk, location, formatted}: SnapshotWriteOptions,
	): Promise<void> {
		const {processor} = this.runner.printer;

		if (used && existsOnDisk) {
			const raw = await this.getRawSnapshot(path);
			if (formatted !== raw) {
				processor.addDiagnostic({
					description: descriptions.SNAPSHOTS.INCORRECT(raw, formatted),
					location,
				});
			}
		}

		if (used && !existsOnDisk) {
			processor.addDiagnostic({
				description: descriptions.SNAPSHOTS.MISSING,
				location,
			});
		}

		if (!used && existsOnDisk) {
			processor.addDiagnostic({
				description: descriptions.SNAPSHOTS.REDUNDANT,
				location,
			});
		}
	}

	public async writeInlineSnapshots() {
		let {inlineSnapshotUpdates, path, ref, request, runner} = this;
		if (inlineSnapshotUpdates.length === 0) {
			return;
		}

		// Resolve source maps. These will originally be pointed to the compiled source.
		inlineSnapshotUpdates = inlineSnapshotUpdates.map((update) => {
			const resolved = runner.sourceMaps.assertApproxOriginalPositionFor(
				path,
				update.line,
				update.column,
			);

			if (!resolved.found) {
				throw new Error("Could not find inline snapshot location in source map");
			}

			if (!(resolved.source.equal(path) || resolved.source.equal(ref.uid))) {
				throw new Error(
					pretty`Inline snapshot update resolved to ${resolved.source} when it should be ${path}`,
				);
			}

			return {
				...update,
				line: resolved.line,
				column: resolved.column,
			};
		});

		// Clear snapshot updates
		this.inlineSnapshotUpdates = [];

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

	// NB: This could be further optimizing by having workers notify us if they have console advice
	public async emitConsoleAdvice(): Promise<void> {
		// We do not show console advice when there are no test failures
		if (!this.hasDiagnostics) {
			return;
		}

		// Fetch console advice from all workers who ran a test
		let allConsoleAdvice: TestConsoleAdvice = [];
		await Promise.all(
			Array.from(
				this.workers,
				async (worker) => {
					const consoleAdvice = await worker.bridge.events.testGetConsoleAdvice.call(
						this.path,
					);
					allConsoleAdvice = allConsoleAdvice.concat(consoleAdvice);
				},
			),
		);

		// Regain chronological order
		allConsoleAdvice = allConsoleAdvice.sort((a, b) => a[1] - b[1]);

		// Flatten
		let advice: DiagnosticAdvice = allConsoleAdvice.map(([advice]) => advice).flat();

		// Emit diagnostic if we had any console advice
		if (advice.length > 0) {
			this.runner.printer.processor.addDiagnostic({
				description: descriptions.TESTS.LOGS(advice),
				location: {
					path: this.path,
				},
			});
		}
	}

	public async teardown(): Promise<void> {
		await Promise.all([
			this.emitConsoleAdvice(),
			this.writeSnapshots(),
			this.writeInlineSnapshots(),
		]);
	}
}
