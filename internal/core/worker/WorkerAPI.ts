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
	Path,
	TransformStageName,
	analyzeDependencies,
	compile,
	lint,
	signals,
} from "@internal/compiler";
import {
	WorkerCompilerOptions,
	WorkerFormatOptions,
	WorkerFormatResult,
	WorkerLintOptions,
	WorkerLintResult,
	WorkerParseOptions,
	WorkerUpdateInlineSnapshotResult,
} from "../common/bridges/WorkerBridge";
import Logger from "../common/utils/Logger";
import * as jsAnalysis from "@internal/js-analysis";
import {ExtensionLintResult} from "../common/file-handlers/types";
import {getFileHandlerFromPathAssert} from "../common/file-handlers/index";
import {
	AnalyzeDependencyResult,
	UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from "../common/types/analyzeDependencies";
import {
	InlineSnapshotUpdate,
	InlineSnapshotUpdates,
} from "../test-worker/SnapshotManager";
import {formatAST} from "@internal/formatter";
import {getNodeReferenceParts, valueToNode} from "@internal/js-ast-utils";
import {markup} from "@internal/markup";
import {RecoverySaveFile} from "../server/fs/RecoveryStore";

// Some Windows git repos will automatically convert Unix line endings to Windows
// This retains the line endings for the formatted code if they were present in the source
function normalizeFormattedLineEndings(
	sourceText: string,
	formatted: string,
): string {
	if (sourceText.includes("\r")) {
		return formatted.replace(/\n/g, "\r\n");
	} else {
		return formatted;
	}
}

export default class WorkerAPI {
	constructor(worker: Worker) {
		this.worker = worker;
		this.logger = worker.logger;
	}

	private worker: Worker;
	private logger: Logger;

	private interceptDiagnostics<T extends {
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
	) {
		const {ast, project} = await this.worker.parse(ref, parseOptions);

		if (ast.type !== "JSRoot") {
			throw new Error(
				`Expected a JSRoot for moduleSignatureJS but got ${ast.type}`,
			);
		}

		this.logger.info(markup`Generating module signature: ${ref.real}`);

		return await jsAnalysis.getModuleSignature({
			ast,
			project,
			provider: await this.worker.getTypeCheckProvider(
				ref.project,
				{},
				parseOptions,
			),
		});
	}

	public async updateInlineSnapshots(
		ref: FileReference,
		updates: InlineSnapshotUpdates,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerUpdateInlineSnapshotResult> {
		let {ast, mtime, project} = await this.worker.parse(ref, parseOptions);

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
			enter(path: Path) {
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
							loc.start.column === update.column &&
							loc.start.line === update.line
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
				mtime,
			};
		}

		return {diagnostics: diags, file};
	}

	public async analyzeDependencies(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<AnalyzeDependencyResult> {
		const project = this.worker.getProject(ref.project);
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
		}

		return analyzeResult.value;
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
					analyze: await this.analyzeDependencies(ref, parseOptions),
				},
			};
		}
	}

	public async compile(
		ref: FileReference,
		stage: TransformStageName,
		options: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<CompileResult> {
		const {ast, project, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);
		this.logger.info(markup`Compiling: ${ref.real}`);

		const compilerOptions = await this.workerCompilerOptionsToCompilerOptions(
			ref,
			options,
			parseOptions,
		);
		return this.interceptDiagnostics(
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
		formatOptions: WorkerFormatOptions,
		parseOptions: WorkerParseOptions,
	): Promise<undefined | WorkerFormatResult> {
		const res = await this._format(ref, formatOptions, parseOptions);
		if (res === undefined) {
			return undefined;
		} else {
			return {
				formatted: normalizeFormattedLineEndings(res.sourceText, res.formatted),
				original: res.sourceText,
				diagnostics: res.diagnostics,
				suppressions: res.suppressions,
			};
		}
	}

	private async _format(
		ref: FileReference,
		formatOptions: WorkerFormatOptions,
		parseOptions: WorkerParseOptions,
	): Promise<undefined | ExtensionLintResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(markup`Formatting: ${ref.real}`);

		const {handler} = getFileHandlerFromPathAssert(ref.real, project.config);

		if (
			!formatOptions.forceFormat &&
			(!handler.capabilities.format || !project.config.format.enabled)
		) {
			return;
		}

		const {customFormat} = handler;
		if (customFormat !== undefined) {
			return await customFormat({
				mtime: await this.worker.getMtime(ref.real),
				file: ref,
				project,
				worker: this.worker,
				parseOptions,
			});
		}

		const {ast, mtime, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);

		const out = formatAST(
			ast,
			{
				...formatOptions,
				projectConfig: project.config,
			},
		);

		return this.interceptDiagnostics(
			{
				mtime,
				formatted: out.code,
				sourceText,
				suppressions: [],
				diagnostics: ast.diagnostics,
			},
			{astModifiedFromSource},
		);
	}

	public async lint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerLintResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(markup`Linting: ${ref.real}`);

		// Get the extension handler
		const {handler} = getFileHandlerFromPathAssert(ref.real, project.config);

		if (!handler.capabilities.lint && !handler.capabilities.format) {
			return {
				save: undefined,
				diagnostics: [],
				suppressions: [],
			};
		}

		// Catch any diagnostics, in the case of syntax errors etc
		const res = await catchDiagnostics(
			() => {
				if (handler.capabilities.lint) {
					return this.compilerLint(ref, options, parseOptions);
				} else {
					return this._format(ref, {}, parseOptions);
				}
			},
			{
				category: "lint",
				message: "Caught by WorkerAPI.lint",
			},
		);

		// These are fatal diagnostics
		if (res.diagnostics !== undefined) {
			return {
				save: undefined,
				suppressions: [],
				diagnostics: res.diagnostics,
			};
		}

		// `format` could have return undefined
		if (res.value === undefined) {
			return {
				save: undefined,
				diagnostics: [],
				suppressions: [],
			};
		}

		// These are normal diagnostics returned from the linter
		const {
			sourceText,
			diagnostics,
			suppressions,
			mtime,
		}: ExtensionLintResult = res.value;

		const formatted = normalizeFormattedLineEndings(
			sourceText,
			res.value.formatted,
		);

		// If the file has pending fixes
		const needsSave = project.config.format.enabled && formatted !== sourceText;

		// Autofix if necessary
		if (options.save && needsSave) {
			return {
				save: {
					type: "WRITE",
					mtime,
					content: formatted,
				},
				diagnostics,
				suppressions,
			};
		}

		// If there's no pending fix then no need for diagnostics
		if (!needsSave) {
			return {
				save: undefined,
				diagnostics,
				suppressions,
			};
		}

		// Add pending autofix diagnostic
		return {
			save: undefined,
			suppressions,
			diagnostics: [
				...diagnostics,
				{
					tags: {fixable: true},
					location: {
						filename: ref.uid,
					},
					description: descriptions.LINT.PENDING_FIXES(
						ref.relative.join(),
						handler.language,
						sourceText,
						formatted,
					),
				},
			],
		};
	}

	private async compilerLint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<ExtensionLintResult> {
		const {ast, mtime, sourceText, project, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);

		// Run the compiler in lint-mode which is where all the rules are actually ran
		const res = await lint({
			applySafeFixes: options.applySafeFixes,
			suppressionExplanation: options.suppressionExplanation,
			ref,
			options: {
				lint: options.compilerOptions,
			},
			ast,
			project,
			sourceText,
		});

		// Extract lint diagnostics
		let {diagnostics} = res;

		// Only enable typechecking if enabled in .romeconfig
		let typeCheckingEnabled = project.config.typeCheck.enabled;
		if (project.config.typeCheck.libs.has(ref.real)) {
			// don't typecheck lib files
			typeCheckingEnabled = false;
		}

		// Run type checking if necessary
		if (typeCheckingEnabled && ast.type === "JSRoot") {
			const typeCheckProvider = await this.worker.getTypeCheckProvider(
				ref.project,
				options.prefetchedModuleSignatures,
				parseOptions,
			);
			const typeDiagnostics = await jsAnalysis.check({
				ast,
				provider: typeCheckProvider,
				project,
			});
			diagnostics = [...diagnostics, ...typeDiagnostics];
		}

		return this.interceptDiagnostics(
			{
				suppressions: res.suppressions,
				diagnostics,
				sourceText,
				formatted: res.src,
				mtime,
			},
			{astModifiedFromSource},
		);
	}
}
