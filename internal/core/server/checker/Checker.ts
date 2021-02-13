/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server, ServerRequest} from "@internal/core";
import {LINTABLE_EXTENSIONS} from "@internal/core/common/file-handlers";
import {Diagnostics, DiagnosticsProcessor} from "@internal/diagnostics";
import {EventSubscription} from "@internal/events";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyPath,
	MixedPathMap,
} from "@internal/path";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import DependencyGraph from "../dependencies/DependencyGraph";
import {
	ReporterProgress,
	ReporterProgressOptions,
} from "@internal/cli-reporter";
import DependencyNode from "../dependencies/DependencyNode";
import {
	LintCompilerOptions,
	LintCompilerOptionsDecisions,
	areAnalyzeDependencyResultsEqual,
} from "@internal/compiler";
import {StaticMarkup, markup} from "@internal/markup";
import {Dict, VoidCallback} from "@internal/typescript-helpers";
import {FileNotFound} from "@internal/fs";
import {WatchFilesEvent} from "../fs/glob";
import {WorkerIntegrationTimings} from "@internal/core/worker/types";
import {ExtendedMap} from "@internal/collections";
import {humanizeDuration} from "@internal/string-utils";

type CheckWatchChange = {
	path: AnyPath;
	diagnostics: Diagnostics;
};

export type LinterCompilerOptionsPerFile = Dict<Required<LintCompilerOptions>>;

export type CheckerOptions = {
	apply?: boolean;
	args?: string[];
	hasDecisions?: boolean;
	formatOnly?: boolean;
	globalDecisions?: LintCompilerOptionsDecisions;
	lintCompilerOptionsPerFile?: LinterCompilerOptionsPerFile;
	suppressionExplanation?: string;
};

type ProgressFactory = (opts: ReporterProgressOptions) => ReporterProgress;

type WatchEvents = {
	onRunStart: VoidCallback;
	createProgress: ProgressFactory;
	onRunEnd: (result: RunResult) => Promise<void> | void;
	onChange: (result: CheckWatchChange) => Promise<void> | void;
};

type RunResult = {
	runner: CheckRunner;
	evictedPaths: AbsoluteFilePathSet;
	savedPaths: AbsoluteFilePathSet;
	totalCount: number;
};

const FLUSH_CHANGES_TIMEOUT = 200;

class CheckRunner {
	constructor(
		checker: Checker,
		{
			graph,
			events,
		}: {
			events: WatchEvents;
			graph: DependencyGraph;
		},
	) {
		this.checker = checker;
		this.server = checker.request.server;
		this.graph = graph;
		this.request = checker.request;
		this.options = checker.options;
		this.events = events;
		this.hadDependencyValidationErrors = new AbsoluteFilePathMap();
		this.timingsByWorker = new ExtendedMap("timingsByWorker", () => new Map());

		this.pendingChanges = new MixedPathMap();
		this.flushChangesTimer = undefined;

		this.compilerProcessor = this.createDiagnosticsProcessor();
		this.dependencyProcessor = this.createDiagnosticsProcessor();
		this.processors = [this.compilerProcessor, this.dependencyProcessor];
	}

	public events: WatchEvents;

	private hadDependencyValidationErrors: AbsoluteFilePathMap<boolean>;
	private checker: Checker;
	private server: Server;
	private request: ServerRequest;
	private graph: DependencyGraph;
	private options: CheckerOptions;
	private timingsByWorker: ExtendedMap<number, WorkerIntegrationTimings>;

	private compilerProcessor: DiagnosticsProcessor;
	private dependencyProcessor: DiagnosticsProcessor;
	private processors: DiagnosticsProcessor[];
	private pendingChanges: MixedPathMap<{
		guaranteed: boolean;
	}>;
	private flushChangesTimer: undefined | NodeJS.Timeout;

	private createDiagnosticsProcessor(): DiagnosticsProcessor {
		const processor = this.checker.createDiagnosticsProcessor();

		processor.guaranteedDiagnosticsEvent.subscribe((diags) => {
			for (const diag of diags) {
				this.queueChanges(diag.location.path, true);
			}
		});

		processor.modifiedDiagnosticsForPathEvent.subscribe((path) => {
			this.queueChanges(path, false);
		});

		return processor;
	}

	public processIntegrationTimings(): {
		slowest: WorkerIntegrationTimings;
		total: WorkerIntegrationTimings;
	} {
		let slowest: WorkerIntegrationTimings = new Map();
		let total: WorkerIntegrationTimings = new Map();

		for (const timings of this.timingsByWorker.values()) {
			for (let [key, timing] of timings) {
				const existingTotal = total.get(key);

				if (existingTotal === undefined) {
					total.set(key, timing);
				} else {
					total.set(
						key,
						{
							...existingTotal,
							took: existingTotal.took + timing.took,
						},
					);
				}

				const existingSlowest = slowest.get(key);
				if (existingSlowest === undefined || existingSlowest.took > timing.took) {
					slowest.set(key, timing);
				}
			}
		}

		return {slowest, total};
	}

	private async runCompiler({paths}: WatchFilesEvent): Promise<void> {
		const {server} = this.request;
		const {
			lintCompilerOptionsPerFile = {},
			globalDecisions = [],
			hasDecisions,
		} = this.options;
		const shouldSave = this.checker.shouldSave();
		const applySafeFixes = !this.checker.shouldOnlyFormat();

		const queue = server.createWorkerQueue({
			callback: async ({path}) => {
				const filename = path.join();
				const progressId = progress.pushText(markup`${path}`);

				let compilerOptions = lintCompilerOptionsPerFile[filename];

				// If we have decisions then make sure it's declared on all files
				if (hasDecisions) {
					if (compilerOptions === undefined) {
						compilerOptions = {
							hasDecisions: true,
							globalDecisions,
							decisionsByPosition: {},
						};
					} else {
						compilerOptions = {
							...compilerOptions,
							hasDecisions: true,
							globalDecisions: [
								...(compilerOptions.globalDecisions || []),
								...globalDecisions,
							],
						};
					}
				}

				this.compilerProcessor.removePath(path);

				const res = await FileNotFound.allowMissing(
					path,
					() =>
						this.request.requestWorkerLint(
							path,
							{
								save: shouldSave,
								applySafeFixes,
								compilerOptions,
								suppressionExplanation: this.options.suppressionExplanation,
							},
						)
					,
				);

				if (res.missing) {
					return;
				}

				const {
					diagnostics,
					suppressions,
					save,
					timingsNs,
				} = res.value;
				this.compilerProcessor.addSuppressions(suppressions);
				this.compilerProcessor.addDiagnostics(diagnostics);
				if (save !== undefined) {
					this.request.queueSaveFile(path, save);
				}

				// Update timings
				const workerId = server.fileAllocator.getOwnerAssert(path).id;
				const workerTimings = this.timingsByWorker.assert(workerId);
				for (let [key, timing] of timingsNs) {
					const existing = workerTimings.get(key);
					if (existing === undefined) {
						workerTimings.set(key, timing);
					} else {
						workerTimings.set(
							key,
							{
								...existing,
								took: existing.took + timing.took,
							},
						);
					}
				}

				progress.popText(progressId);
				progress.tick();
			},
		});

		const progress = this.events.createProgress({title: markup`Linting`});
		progress.setTotal(paths.size);

		await queue.prepare(paths);

		for (const path of paths) {
			await FileNotFound.allowMissing(path, () => queue.pushPath(path));
		}

		await queue.spin();
		progress.end();
	}

	private async seedGraph(
		{progressText, paths}: {
			progressText: StaticMarkup;
			paths: AbsoluteFilePathSet;
		},
	) {
		const filteredPaths: AbsoluteFilePath[] = [];
		for (const path of paths) {
			const project = this.server.projectManager.assertProjectExisting(path);
			if (project.config.check.dependencies) {
				filteredPaths.push(path);
				this.dependencyProcessor.removePath(path);

				// Take compiler suppressions for path if they exist
				const suppressions = this.compilerProcessor.getSuppressionsForPath(path);
				if (suppressions !== undefined) {
					this.compilerProcessor.addSuppressions(suppressions);
				}
			}
		}
		if (filteredPaths.length === 0) {
			return;
		}

		const progress = this.events.createProgress({
			title: progressText,
		});
		await this.graph.seed({
			allowFileNotFound: true,
			paths: filteredPaths,
			diagnosticsProcessor: this.dependencyProcessor,
			validate: false,
			analyzeProgress: progress,
		});
		progress.end();
	}

	private async runGraph(event: WatchFilesEvent): Promise<void> {
		const {graph} = this;
		const evictedPaths = event.paths;

		// Get all the current dependency nodes for the evicted files, and invalidate their nodes
		const oldEvictedNodes: AbsoluteFilePathMap<DependencyNode> = new AbsoluteFilePathMap();
		for (const path of evictedPaths) {
			const node = graph.maybeGetNode(path);
			if (node !== undefined) {
				oldEvictedNodes.set(path, node);
				graph.deleteNode(path);
			}
		}

		// Refresh only the evicted paths
		await this.seedGraph({
			paths: evictedPaths,
			progressText: event.initial
				? markup`Analyzing files`
				: markup`Analyzing changed files`,
		});

		// Maintain a list of all the dependencies we revalidated
		const validatedDependencyPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		// Maintain a list of all the dependents that need to be revalidated
		const validatedDependencyPathDependents: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		// Build a list of dependents to recheck
		for (const path of evictedPaths) {
			const newNode = graph.maybeGetNode(path);
			if (newNode === undefined) {
				continue;
			}

			validatedDependencyPaths.add(path);

			// Get the previous node and see if the exports have actually changed
			const oldNode = oldEvictedNodes.get(path);
			const sameShape =
				oldNode !== undefined &&
				areAnalyzeDependencyResultsEqual(
					oldNode.analyze.value,
					newNode.analyze.value,
				);

			for (const depNode of newNode.getDependents()) {
				// If the old node has the same shape as the new one, only revalidate the dependent if it had dependency errors
				if (
					sameShape &&
					this.hadDependencyValidationErrors.get(depNode.path) === false
				) {
					continue;
				}

				validatedDependencyPaths.add(depNode.path);
				validatedDependencyPathDependents.add(depNode.path);
			}
		}

		// Revalidate dependents
		if (validatedDependencyPathDependents.size > 0) {
			await this.seedGraph({
				progressText: markup`Analyzing dependents`,
				paths: validatedDependencyPaths,
			});
		}

		// Validate connections
		for (const path of validatedDependencyPaths) {
			graph.validate(graph.getNode(path), this.dependencyProcessor);
			this.hadDependencyValidationErrors.set(
				path,
				this.dependencyProcessor.hasDiagnosticsForPath(path),
			);
		}
	}

	public getDiagnosticsForPath(path: AnyPath, guaranteed: boolean): Diagnostics {
		const processor = new DiagnosticsProcessor();

		for (const subprocessor of this.processors) {
			const diagnostics = subprocessor.getDiagnosticsForPath(path, false);
			if (diagnostics !== undefined) {
				processor.addDiagnostics(
					guaranteed ? diagnostics.guaranteed : diagnostics.complete,
				);
				processor.addSuppressions(diagnostics.suppressions);
			}
		}

		return processor.getDiagnostics();
	}

	private flushChanges() {
		if (this.flushChangesTimer !== undefined) {
			clearTimeout(this.flushChangesTimer);
			this.flushChangesTimer = undefined;
		}

		const {pendingChanges} = this;
		this.pendingChanges = new MixedPathMap();

		for (const [path, {guaranteed}] of pendingChanges) {
			this.events.onChange({
				path,
				diagnostics: this.getDiagnosticsForPath(path, guaranteed),
			});
		}
	}

	private queueChanges(path: AnyPath, guaranteed: boolean) {
		const existing = this.pendingChanges.get(path);
		if (existing !== undefined && !existing.guaranteed) {
			return;
		}

		this.pendingChanges.set(path, {guaranteed});

		if (this.flushChangesTimer === undefined) {
			this.flushChangesTimer = setTimeout(
				() => this.flushChanges(),
				FLUSH_CHANGES_TIMEOUT,
			);
		}
	}

	public async run(event: WatchFilesEvent): Promise<void> {
		this.events.onRunStart();

		// Remove deleted paths
		for (const path of event.paths) {
			if (!this.server.memoryFs.exists(path)) {
				this.compilerProcessor.removePath(path);
				this.dependencyProcessor.removePath(path);
			}
		}

		// Run compiler lint
		await this.runCompiler(event);

		// Update dependency graph
		await this.runGraph(event);

		// Flush saved files
		const savedPaths = await this.request.flushFiles();

		// Queue complete diagnostics if they are different than guaranteed
		for (const processor of this.processors) {
			for (const path of processor.getPaths()) {
				const diagnostics = processor.getDiagnosticsForPath(path);
				if (diagnostics !== undefined) {
					if (diagnostics.complete.length !== diagnostics.guaranteed.length) {
						this.queueChanges(path, false);
					}
				}
			}
		}

		this.flushChanges();

		await this.events.onRunEnd({
			evictedPaths: event.paths,
			savedPaths,
			totalCount: event.paths.size,
			runner: this,
		});
	}
}

export default class Checker {
	constructor(req: ServerRequest, opts: CheckerOptions) {
		this.request = req;
		this.options = opts;
	}

	public request: ServerRequest;
	public options: CheckerOptions;

	public shouldOnlyFormat(): boolean {
		const {formatOnly} = this.options;
		const {review} = this.request.query.requestFlags;
		return formatOnly || review;
	}

	public shouldSave(): boolean {
		const {apply, hasDecisions} = this.options;
		return apply || hasDecisions || this.shouldOnlyFormat();
	}

	public createDiagnosticsProcessor(): DiagnosticsProcessor {
		const processor = this.request.createDiagnosticsProcessor({
			origins: [
				{
					category: "lint",
					message: "Dispatched",
				},
			],
		});

		processor.addAllowedUnusedSuppressionPrefix("bundler");

		return processor;
	}

	private createDiagnosticsPrinter(
		{runner, getStats, streaming}: {
			streaming: boolean;
			runner: CheckRunner;
			getStats: () => {
				totalCount: number;
				savedCount: number;
			};
		},
	): DiagnosticsPrinter {
		const {request} = this;
		const processor = this.createDiagnosticsProcessor();
		const printer = request.createDiagnosticsPrinter(processor, {streaming});

		printer.onFooterPrint(async (reporter, isError) => {
			if (isError) {
				let hasPendingFixes = false;

				for (const {tags} of processor.getDiagnostics()) {
					if (tags?.fixable) {
						hasPendingFixes = true;
					}
				}

				if (hasPendingFixes) {
					reporter.info(
						markup`Fixes available. To apply safe fixes and formatting run`,
					);
					reporter.command("rome check --apply");
					reporter.info(markup`To choose fix suggestions run`);
					reporter.command("rome check --review");
					reporter.br();
				}
			}

			const {savedCount, totalCount} = getStats();

			if (savedCount > 0) {
				reporter.success(
					markup`<number emphasis>${String(savedCount)}</number> <grammarNumber plural="files" singular="file">${String(
						savedCount,
					)}</grammarNumber> updated`,
				);
				reporter.info(
					markup`You can revert these changes with the <code>rome recover pop</code> command`,
				);
				reporter.br();
			}

			if (!isError) {
				if (totalCount === 0) {
					reporter.warn(markup`No files linted`);
				} else {
					reporter.info(
						markup`<number emphasis>${String(totalCount)}</number> <grammarNumber plural="files" singular="file">${String(
							totalCount,
						)}</grammarNumber> linted`,
					);
				}
			}

			const timings = runner.processIntegrationTimings();
			for (const timing of timings.total.values()) {
				if (timing.took > 0n) {
					const ms = Number(timing.took / 1000000n);
					reporter.warn(
						markup`Spent <emphasis>${humanizeDuration(
							ms,
							{longform: true, allowMilliseconds: true},
						)}</emphasis> running ${timing.displayName}`,
					);
				}
			}
		});

		return printer;
	}

	public createRunner(events: WatchEvents): CheckRunner {
		const graph = new DependencyGraph(
			this.request,
			this.request.getResolverOptionsFromFlags(),
			{shallow: true},
		);

		return new CheckRunner(this, {events, graph});
	}

	public async watch(runner: CheckRunner): Promise<EventSubscription> {
		const globber = await this.request.glob({
			args: this.options.args,
			noun: "lint",
			verb: "linting",
			configCategory: "lint",
			extensions: LINTABLE_EXTENSIONS,
		});

		return globber.watch(async (event) => {
			await runner.run(event);
		});
	}

	public async runWatch() {
		const {request} = this;
		const {reporter} = request;

		const diagnosticsByPath: MixedPathMap<Diagnostics> = new MixedPathMap();

		const runner = this.createRunner({
			onRunStart: () => {
				reporter.clearScreen();
			},
			createProgress: (opts) => {
				return reporter.progress(opts);
			},
			onChange: ({path, diagnostics}) => {
				if (diagnostics.length === 0) {
					diagnosticsByPath.delete(path);
				} else {
					diagnosticsByPath.set(path, diagnostics);
				}
			},
			onRunEnd: async ({totalCount, savedPaths, runner}) => {
				const printer = this.createDiagnosticsPrinter({
					streaming: true,
					runner,
					getStats: () => ({
						totalCount,
						savedCount: savedPaths.size,
					}),
				});

				// Print all diagnostics
				for (const diagnostics of diagnosticsByPath.values()) {
					printer.processor.addDiagnostics(diagnostics);
				}

				reporter.clearScreen();
				await printer.print();
			},
		});
		await this.watch(runner);
		await request.endEvent.wait();
	}

	public async runSingle(): Promise<{
		printer: DiagnosticsPrinter;
		savedCount: number;
	}> {
		const {request} = this;
		const {reporter} = request;

		const diagnosticsByPath: MixedPathMap<Diagnostics> = new MixedPathMap();

		let savedPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
		let paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		// If we aren't saving then we can print diagnostics as we receive them.
		// When we are saving, we may have diagnostics that eventually get cleared once we fix or format a file
		const streaming = !this.shouldSave();

		const runner = this.createRunner({
			onRunStart: () => {},
			createProgress: (opts) => {
				return reporter.progress(opts);
			},
			onChange: ({path, diagnostics}) => {
				if (streaming) {
					printer.processor.addDiagnostics(diagnostics);
				} else {
					if (diagnostics.length === 0) {
						diagnosticsByPath.delete(path);
					} else {
						diagnosticsByPath.set(path, diagnostics);
					}
				}
			},
			onRunEnd: (res) => {
				// Update counts
				savedPaths.addSet(res.savedPaths);
				paths = new AbsoluteFilePathSet([...paths, ...res.evictedPaths]);
			},
		});

		const printer = this.createDiagnosticsPrinter({
			streaming,
			runner,
			getStats: () => ({
				totalCount: paths.size,
				savedCount: savedPaths.size,
			}),
		});

		const watchEvent = await this.watch(runner);
		await watchEvent.unsubscribe();

		if (!streaming) {
			for (const diagnostics of diagnosticsByPath.values()) {
				printer.processor.addDiagnostics(diagnostics);
			}
		}

		return {printer, savedCount: savedPaths.size};
	}

	public async throwSingle() {
		const {printer} = await this.runSingle();
		throw printer;
	}
}
