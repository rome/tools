/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server, ServerRequest} from "@internal/core";
import {LINTABLE_EXTENSIONS} from "@internal/core/common/file-handlers";
import {
	DiagnosticSuppressions,
	Diagnostics,
	DiagnosticsProcessor,
} from "@internal/diagnostics";
import {EventSubscription} from "@internal/events";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyPath,
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
import {markup} from "@internal/markup";
import {Dict, VoidCallback} from "@internal/typescript-helpers";
import {FileNotFound} from "@internal/fs";
import {WatchFilesEvent} from "../fs/glob";
import {
	EMPTY_LINT_TIMINGS,
	WorkerLintTimings,
} from "@internal/core/worker/types";
import {ExtendedMap} from "@internal/collections";
import {humanizeDuration} from "@internal/string-utils";

type LintWatchChange =
	| {
			type: "absolute";
			path: AbsoluteFilePath;
			diagnostics: Diagnostics;
		}
	| {
			type: "unknown";
			path: undefined | AnyPath;
			diagnostics: Diagnostics;
		};

type LintWatchChanges = LintWatchChange[];

export type LinterCompilerOptionsPerFile = Dict<Required<LintCompilerOptions>>;

export type LinterOptions = {
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
	onChanges: (result: WatchResults) => Promise<void> | void;
};

type WatchResults = {
	runner: LintRunner;
	evictedPaths: AbsoluteFilePathSet;
	changes: LintWatchChanges;
	savedCount: number;
	totalCount: number;
};

function createDiagnosticsPrinter(
	runner: LintRunner,
	request: ServerRequest,
	processor: DiagnosticsProcessor,
	totalCount: number,
	savedCount: number,
): DiagnosticsPrinter {
	const printer = request.createDiagnosticsPrinter(processor);

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

		const timings = runner.getFinalTimings();
		if (timings.slowest.eslint > 0n) {
			const ms = Number(timings.slowest.eslint / 1000000n);
			reporter.warn(
				markup`Spent <emphasis>${humanizeDuration(
					ms,
					{longform: true, allowMilliseconds: true},
				)}</emphasis> running ESLint`,
			);
		}
	});

	return printer;
}

class LintRunner {
	constructor(
		linter: Linter,
		{
			graph,
			events,
		}: {
			events: WatchEvents;
			graph: DependencyGraph;
		},
	) {
		this.linter = linter;
		this.server = linter.request.server;
		this.graph = graph;
		this.request = linter.request;
		this.options = linter.options;
		this.events = events;
		this.compilerDiagnosticsCache = new AbsoluteFilePathMap();
		this.hadDependencyValidationErrors = new AbsoluteFilePathMap();
		this.timingsByWorker = new ExtendedMap(
			"timingsByWorker",
			() => EMPTY_LINT_TIMINGS,
		);
	}

	private hadDependencyValidationErrors: AbsoluteFilePathMap<boolean>;
	private compilerDiagnosticsCache: AbsoluteFilePathMap<{
		diagnostics: Diagnostics;
		suppressions: DiagnosticSuppressions;
	}>;
	private linter: Linter;
	private events: WatchEvents;
	private server: Server;
	private request: ServerRequest;
	private graph: DependencyGraph;
	private options: LinterOptions;
	private timingsByWorker: ExtendedMap<number, WorkerLintTimings>;

	public getFinalTimings(): {
		slowest: WorkerLintTimings;
		total: WorkerLintTimings;
	} {
		let slowest: WorkerLintTimings = EMPTY_LINT_TIMINGS;
		let total: WorkerLintTimings = EMPTY_LINT_TIMINGS;

		for (const timings of this.timingsByWorker.values()) {
			total.eslint += timings.eslint;
			total.prettier += timings.prettier;

			if (timings.eslint > slowest.eslint) {
				slowest = {
					...slowest,
					eslint: timings.eslint,
				};
			}

			if (timings.prettier > slowest.prettier) {
				slowest = {
					...slowest,
					prettier: timings.prettier,
				};
			}
		}

		return {slowest, total};
	}

	public hasCompilerDiagnostics(path: AbsoluteFilePath): boolean {
		return this.compilerDiagnosticsCache.has(path);
	}

	private clearCompilerDiagnosticsForPath(path: AbsoluteFilePath) {
		this.compilerDiagnosticsCache.set(path, {suppressions: [], diagnostics: []});
	}

	private async runLint(
		{paths}: WatchFilesEvent,
		processor: DiagnosticsProcessor,
	): Promise<void> {
		const {server} = this.request;
		const {
			lintCompilerOptionsPerFile = {},
			globalDecisions = [],
			hasDecisions,
		} = this.options;
		const shouldSave = this.linter.shouldSave();
		const applySafeFixes = !this.linter.shouldOnlyFormat();

		const queue = server.createWorkerQueue({
			callback: async ({path}) => {
				const filename = path.join();
				const progressId = progress.pushText(
					markup`<filelink target="${filename}" />`,
				);

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
				processor.addSuppressions(suppressions);
				processor.addDiagnostics(diagnostics);
				this.compilerDiagnosticsCache.set(path, {suppressions, diagnostics});
				if (save !== undefined) {
					this.request.queueSaveFile(path, save);
				}

				const workerId = server.fileAllocator.getOwnerAssert(path).id;
				const workerTimings = this.timingsByWorker.assert(workerId);
				this.timingsByWorker.set(
					workerId,
					{
						eslint: workerTimings.eslint + timingsNs.eslint,
						prettier: workerTimings.prettier + timingsNs.prettier,
					},
				);

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

	private async runGraph(
		event: WatchFilesEvent,
		processor: DiagnosticsProcessor,
	): Promise<AbsoluteFilePathSet> {
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
		const progress = this.events.createProgress({
			title: event.initial
				? markup`Analyzing files`
				: markup`Analyzing changed files`,
		});
		await graph.seed({
			allowFileNotFound: true,
			paths: Array.from(evictedPaths),
			diagnosticsProcessor: processor,
			validate: false,
			analyzeProgress: progress,
		});
		progress.end();

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
			const progress = this.events.createProgress({
				title: markup`Analyzing dependents`,
			});

			await graph.seed({
				paths: Array.from(validatedDependencyPaths),
				diagnosticsProcessor: processor,
				validate: false,
				analyzeProgress: progress,
			});

			progress.end();
		}

		// Validate connections
		for (const path of validatedDependencyPaths) {
			const hasValidationErrors = graph.validate(graph.getNode(path), processor);
			this.hadDependencyValidationErrors.set(path, hasValidationErrors);
		}

		return validatedDependencyPaths;
	}

	private computeChanges(
		{paths: evictedPaths}: WatchFilesEvent,
		processor: DiagnosticsProcessor,
		validatedDependencyPaths: AbsoluteFilePathSet,
	): LintWatchChanges {
		const {server} = this;
		const changes: LintWatchChanges = [];

		const updatedPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet([
			...validatedDependencyPaths,
		]);

		// Deleted paths wont show up in validatedDependencyPaths so we need to readd them
		for (const path of evictedPaths) {
			if (!server.memoryFs.exists(path)) {
				updatedPaths.add(path);
				this.clearCompilerDiagnosticsForPath(path);
			}
		}

		const diagnosticsByPath = processor.getDiagnosticsByPath();

		// In case we pushed on any diagnostics that aren't from the input paths, try to resolve them
		const includedPathsInDiagnostics = server.projectManager.categorizePaths(
			diagnosticsByPath.map.keys(),
		);
		for (const path of includedPathsInDiagnostics.absolutes) {
			updatedPaths.add(path);
		}

		// validatedDependencyPaths can include paths that weren't changed, but needed to be recomputed
		// as they were dependents of one of the files that was
		// In that case we need to push their previous compiler diagnostics
		for (const path of validatedDependencyPaths) {
			if (!evictedPaths.has(path)) {
				const compilerDiagnostics = this.compilerDiagnosticsCache.get(path);
				if (compilerDiagnostics !== undefined) {
					processor.addSuppressions(compilerDiagnostics.suppressions);
					processor.addDiagnostics(compilerDiagnostics.diagnostics);
				}
			}
		}

		// We can't just use getDiagnosticFilenames as we need to produce empty arrays for removed diagnostics
		for (const path of updatedPaths) {
			// Could have been a UID that we turned into an absolute path so turn it back
			const diagnostics = [
				...(diagnosticsByPath.map.get(path) || []),
				...(diagnosticsByPath.map.get(
					this.server.projectManager.getUid(path, true),
				) || []),
			];

			changes.push({
				type: "absolute",
				path,
				diagnostics,
			});
		}

		// We can produce diagnostics that don't actually point at a file. For LSP we will just throw these away,
		// otherwise inside of Rome we can display them.
		// These filenames may be relative or undefined
		for (const path of includedPathsInDiagnostics.unknowns) {
			changes.push({
				type: "unknown",
				path,
				diagnostics: diagnosticsByPath.map.get(path) || [],
			});
		}

		if (includedPathsInDiagnostics.hasUndefined) {
			changes.push({
				type: "unknown",
				path: undefined,
				diagnostics: diagnosticsByPath.pathless,
			});
		}

		return changes;
	}

	public async run(
		event: WatchFilesEvent,
		processor: DiagnosticsProcessor,
	): Promise<WatchResults> {
		this.events.onRunStart();

		// Run compiler lint
		await this.runLint(event, processor);

		// Update dependency graph
		const validatedDependencyPaths = await this.runGraph(event, processor);

		// Computed diagnostic changes
		const changes = await this.computeChanges(
			event,
			processor,
			validatedDependencyPaths,
		);

		// Flush saved files
		const savedCount = await this.request.flushFiles();

		return {
			evictedPaths: event.paths,
			changes,
			savedCount,
			totalCount: event.paths.size,
			runner: this,
		};
	}
}

export default class Linter {
	constructor(req: ServerRequest, opts: LinterOptions) {
		this.request = req;
		this.options = opts;
	}

	public request: ServerRequest;
	public options: LinterOptions;

	public shouldOnlyFormat(): boolean {
		const {formatOnly} = this.options;
		const {review} = this.request.query.requestFlags;
		return formatOnly || review;
	}

	public shouldSave(): boolean {
		const {apply, hasDecisions} = this.options;
		return apply || hasDecisions || this.shouldOnlyFormat();
	}

	private createDiagnosticsProcessor(): DiagnosticsProcessor {
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

	public async watch(events: WatchEvents): Promise<EventSubscription> {
		const graph = new DependencyGraph(
			this.request,
			this.request.getResolverOptionsFromFlags(),
			{shallow: true},
		);

		const runner = new LintRunner(this, {events, graph});

		const globber = await this.request.glob({
			args: this.options.args,
			noun: "lint",
			verb: "linting",
			configCategory: "lint",
			extensions: LINTABLE_EXTENSIONS,
			disabledDiagnosticCategory: "lint/disabled",
		});

		return globber.watch(async (event) => {
			const processor = this.createDiagnosticsProcessor();
			const result = await runner.run(event, processor);
			await events.onChanges(result);
		});
	}

	public async runWatch() {
		const {request} = this;
		const {reporter} = request;

		const diagnosticsByFilename: Map<undefined | string, Diagnostics> = new Map();

		await this.watch({
			onRunStart: () => {
				reporter.clearScreen();
			},
			createProgress: (opts) => {
				return reporter.progress(opts);
			},
			onChanges: async ({evictedPaths, changes, totalCount, savedCount, runner}) => {
				const printer = createDiagnosticsPrinter(
					runner,
					request,
					this.createDiagnosticsProcessor(),
					totalCount,
					savedCount,
				);

				// Update our diagnostics with the changes
				for (const {path, diagnostics} of changes) {
					const filename = path === undefined ? undefined : path.join();
					if (diagnostics.length === 0) {
						diagnosticsByFilename.delete(filename);
					} else {
						diagnosticsByFilename.set(filename, diagnostics);
					}
				}

				// Print all diagnostics
				for (const diagnostics of diagnosticsByFilename.values()) {
					printer.processor.addDiagnostics(diagnostics);
				}

				reporter.clearScreen();
				await printer.print();
				await printer.footer();
			},
		});

		await request.endEvent.wait();
	}

	public async runSingle(): Promise<{
		printer: DiagnosticsPrinter;
		savedCount: number;
	}> {
		const {request} = this;
		const {reporter} = request;
		const diagnosticsByFilename: Map<undefined | string, Diagnostics> = new Map();

		let savedCount = 0;
		let paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
		let runner: undefined | LintRunner;

		const watchEvent = await this.watch({
			onRunStart: () => {},
			createProgress: (opts) => {
				return reporter.progress(opts);
			},
			onChanges: (res) => {
				// Update counts
				savedCount += res.savedCount;
				paths = new AbsoluteFilePathSet([...paths, ...res.evictedPaths]);
				runner = res.runner;

				// Update our diagnostics with the changes
				for (const {path, diagnostics} of res.changes) {
					const filename = path === undefined ? undefined : path.join();
					if (diagnostics.length === 0) {
						diagnosticsByFilename.delete(filename);
					} else {
						diagnosticsByFilename.set(filename, diagnostics);
					}
				}
			},
		});

		await watchEvent.unsubscribe();

		const printer = createDiagnosticsPrinter(
			runner!,
			request,
			this.createDiagnosticsProcessor(),
			paths.size,
			savedCount,
		);

		for (const diagnostics of diagnosticsByFilename.values()) {
			printer.processor.addDiagnostics(diagnostics);
		}

		return {printer, savedCount};
	}

	public async throwSingle() {
		const {printer} = await this.runSingle();
		throw printer;
	}
}
