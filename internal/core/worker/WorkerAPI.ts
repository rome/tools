/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference, Worker} from "@internal/core";
import {AnyNode, AnyRoot} from "@internal/ast";
import {
	Diagnostics,
	catchDiagnostics,
	descriptions,
} from "@internal/diagnostics";
import {
	CompileResult,
	CompilerContext,
	CompilerOptions,
	CompilerPath,
	TransformStageName,
	analyzeDependencies,
	compile,
	signals,
} from "@internal/compiler";
import {
	WorkerAnalyzeDependencyResult,
	WorkerCompileResult,
	WorkerCompilerOptions,
	WorkerFormatOptions,
	WorkerFormatResult,
	WorkerLintOptions,
	WorkerLintResult,
	WorkerParseOptions,
	WorkerUpdateInlineSnapshotResult,
} from "./types";
import Logger from "../common/utils/Logger";
import * as jsAnalysis from "@internal/js-analysis";
import {ModuleSignature} from "@internal/js-analysis";
import {
	AnalyzeDependencyResult,
	UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from "../common/types/analyzeDependencies";
import {
	InlineSnapshotUpdate,
	InlineSnapshotUpdates,
} from "./test/SnapshotManager";
import {formatAST} from "@internal/formatter";
import {getNodeReferenceParts, valueToNode} from "@internal/js-ast-utils";
import {markup} from "@internal/markup";
import {RecoverySaveFile} from "../server/fs/RecoveryStore";
import WorkerCache, {createCacheEntryLoader} from "./WorkerCache";
import {uncachedFormat, uncachedLint} from "./workerLint";

const analyzeDependenciesCacheLoader = createCacheEntryLoader<AnalyzeDependencyResult>(
	"analyzeDependencies",
	(consumer) => consumer.asAny(),
);

const moduleSignatureCacheLoader = createCacheEntryLoader<ModuleSignature>(
	"moduleSignature",
	(consumer) => consumer.asAny(),
);

const compileCacheLoader = createCacheEntryLoader<CompileResult>(
	"compile",
	(consumer) => consumer.asAny(),
);

const lintCacheLoader = createCacheEntryLoader<WorkerLintResult>(
	"lint",
	(consumer) => consumer.asAny(),
);

export default class WorkerAPI {
	constructor(worker: Worker, logger: Logger, cache: WorkerCache) {
		this.worker = worker;
		this.logger = logger;
		this.cache = cache;
	}

	private worker: Worker;
	private logger: Logger;
	private cache: WorkerCache;

	public interceptDiagnostics<T extends {
		diagnostics: Diagnostics;
	}>(
		val: T,
		{astModifiedFromSource}: {
			astModifiedFromSource: boolean;
		},
	): T {
		if (astModifiedFromSource) {
			const diagnostics = val.diagnostics.map((diag) => {
				return {
					...diag,
					metadata: {
						...diag.description,
						advice: [
							...diag.description.advice,
							{
								type: "log",
								category: "warn",
								text: "We manipulated this file before parsing it so the source locations are likely incorrect",
							},
						],
					},
				};
			});

			return {...val, diagnostics};
		} else {
			return val;
		}
	}

	public async moduleSignatureJS(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<ModuleSignature> {
		const cacheEntry = await this.cache.getEntry(
			ref,
			moduleSignatureCacheLoader,
			[parseOptions],
		);
		const cached = await cacheEntry.load();
		if (cached !== undefined) {
			return cached;
		}

		const {ast, project} = await this.worker.parse(ref, parseOptions);

		if (ast.type !== "JSRoot") {
			throw new Error(
				`Expected a JSRoot for moduleSignatureJS but got ${ast.type}`,
			);
		}

		this.logger.info(markup`Generating module signature: ${ref.real}`);

		const res = await jsAnalysis.getModuleSignature({
			ast,
			project,
			provider: await this.worker.getTypeCheckProvider(ref, {}, parseOptions),
		});

		cacheEntry.update(res);

		return res;
	}

	public async updateInlineSnapshots(
		ref: FileReference,
		updates: InlineSnapshotUpdates,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerUpdateInlineSnapshotResult> {
		let {ast, mtimeNs, project} = await this.worker.parse(ref, parseOptions);

		if (!project.config.format.enabled) {
			return {
				file: undefined,
				diagnostics: [
					// TODO not enabled
				],
			};
		}

		const appliedUpdatesToCallees: Set<AnyNode> = new Set();
		const pendingUpdates: Set<InlineSnapshotUpdate> = new Set(updates);
		const context = new CompilerContext({
			ast,
			ref,
		});
		ast = context.reduceRoot({
			name: "updateInlineSnapshots",
			enter(path: CompilerPath) {
				const {node} = path;
				if (node.type !== "JSCallExpression" || pendingUpdates.size === 0) {
					return signals.retain;
				}

				let matchedUpdate: undefined | InlineSnapshotUpdate;

				const {callee} = node;
				for (const {node} of getNodeReferenceParts(callee).parts) {
					const {loc} = node;
					if (loc === undefined) {
						continue;
					}

					for (const update of pendingUpdates) {
						if (
							loc.start.column.equal(update.column) &&
							loc.start.line.equal(update.line)
						) {
							matchedUpdate = update;
							break;
						}
					}

					if (matchedUpdate !== undefined) {
						break;
					}
				}

				if (matchedUpdate !== undefined) {
					if (appliedUpdatesToCallees.has(callee)) {
						context.addNodeDiagnostic(
							node,
							descriptions.SNAPSHOTS.INLINE_COLLISION,
						);
						return signals.retain;
					}

					pendingUpdates.delete(matchedUpdate);
					appliedUpdatesToCallees.add(callee);

					const args = node.arguments;
					if (args.length < 1) {
						context.addNodeDiagnostic(
							node,
							descriptions.SNAPSHOTS.INLINE_MISSING_RECEIVED,
						);
						return signals.retain;
					}

					return signals.replace({
						...node,
						arguments: [args[0], valueToNode(matchedUpdate.snapshot)],
					});
				}

				return signals.retain;
			},
		});

		const diags = context.diagnostics.getDiagnostics();

		if (pendingUpdates.size > 0 && diags.length === 0) {
			throw new Error(
				`${pendingUpdates.size} left over inline snapshots that were not updated ${JSON.stringify(
					Array.from(pendingUpdates),
				)}`,
			);
		}

		let file: undefined | RecoverySaveFile;

		if (diags.length === 0) {
			const formatted = formatAST(
				ast,
				{
					projectConfig: project.config,
				},
			).code;
			file = {
				type: "WRITE",
				content: formatted,
				mtimeNs,
			};
		}

		return {diagnostics: diags, file};
	}

	public async analyzeDependencies(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerAnalyzeDependencyResult> {
		const cacheEntry = await this.cache.getEntry(
			ref,
			analyzeDependenciesCacheLoader,
		);
		const cached = await cacheEntry.load();
		if (cached !== undefined) {
			return {
				integrity: await this.cache.getIntegrity(ref),
				value: cached,
				cached: true,
			};
		}

		const res = await this.uncachedAnalyzeDependencies(ref, parseOptions);

		cacheEntry.update(res);

		return {
			integrity: await this.cache.getIntegrity(ref),
			cached: false,
			value: res,
		};
	}

	private async uncachedAnalyzeDependencies(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<AnalyzeDependencyResult> {
		const project = this.worker.getProject(ref);
		this.logger.info(markup`Analyze dependencies: ${ref.real}`);

		const parseResult = await catchDiagnostics(async () =>
			this.worker.parse(ref, parseOptions)
		);

		if (parseResult.diagnostics !== undefined) {
			return {
				...UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
				diagnostics: parseResult.diagnostics,
			};
		}

		const {ast, sourceText, astModifiedFromSource} = parseResult.value;

		const analyzeResult = await catchDiagnostics(async () =>
			this.interceptDiagnostics(
				await analyzeDependencies({
					ref,
					ast,
					sourceText,
					project,
					options: {},
				}),
				{astModifiedFromSource},
			)
		);

		if (analyzeResult.diagnostics !== undefined) {
			return {
				...UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
				diagnostics: analyzeResult.diagnostics,
			};
		} else {
			return analyzeResult.value;
		}
	}

	private async workerCompilerOptionsToCompilerOptions(
		ref: FileReference,
		workerOptions: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<CompilerOptions> {
		const {bundle, ...options} = workerOptions;

		if (bundle === undefined) {
			return options;
		} else {
			return {
				...options,
				bundle: {
					...bundle,
					analyze: (await this.analyzeDependencies(ref, parseOptions)).value,
				},
			};
		}
	}

	public async compile(
		ref: FileReference,
		stage: TransformStageName,
		options: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerCompileResult> {
		// Check cache for this stage and options
		const cacheEntry = await this.cache.getEntry(
			ref,
			compileCacheLoader,
			[stage, options, parseOptions],
		);
		const cached = await cacheEntry.load();
		if (cached !== undefined) {
			// TODO check cacheDependencies
			return {
				integrity: await this.cache.getIntegrity(ref),
				value: cached,
				cached: true,
			};
		}

		const {ast, integrity, project, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);
		this.logger.info(markup`Compiling: ${ref.real}`);

		const compilerOptions = await this.workerCompilerOptionsToCompilerOptions(
			ref,
			options,
			parseOptions,
		);
		const res = await this.interceptDiagnostics(
			await compile({
				ref,
				ast,
				sourceText,
				options: compilerOptions,
				project,
				stage,
			}),
			{astModifiedFromSource},
		);

		// There's a race condition here between the file being opened and then rewritten
		cacheEntry.update(res);

		return {
			integrity,
			value: res,
			cached: false,
		};
	}

	public async parse(
		ref: FileReference,
		opts: WorkerParseOptions,
	): Promise<AnyRoot> {
		let {ast, astModifiedFromSource} = await this.worker.parse(
			ref,
			{
				...opts,
				sourceTypeJS: opts.sourceTypeJS,
				cache: false,
			},
		);

		return this.interceptDiagnostics(ast, {astModifiedFromSource});
	}

	public async format(
		ref: FileReference,
		options: WorkerFormatOptions,
		parseOptions: WorkerParseOptions,
	): Promise<undefined | WorkerFormatResult> {
		const res = await uncachedFormat({
			worker: this.worker,
			ref,
			options,
			parseOptions,
		});
		return res?.result;
	}

	public async lint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerLintResult> {
		const optionsWithoutModuleSigs = {
			...options,
			// TODO return just mtimes or something
			prefetchedModuleSignatures: undefined,
		};
		const cacheEntry = await this.cache.getEntry(
			ref,
			lintCacheLoader,
			[optionsWithoutModuleSigs, parseOptions],
		);
		const cached = await cacheEntry.load();
		if (cached !== undefined) {
			return cached;
		}

		const res = await uncachedLint({
			worker: this.worker,
			ref,
			options,
			parseOptions,
		});
		cacheEntry.update(res);
		return res;
	}
}
