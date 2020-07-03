/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference, Worker} from "@romejs/core";
import {AnyNode, AnyRoot} from "@romejs/ast";
import {Diagnostics, catchDiagnostics, descriptions} from "@romejs/diagnostics";
import {
	CompileResult,
	CompilerContext,
	CompilerOptions,
	Path,
	TransformStageName,
	analyzeDependencies,
	compile,
	lint,
} from "@romejs/compiler";
import {
	WorkerCompilerOptions,
	WorkerFormatResult,
	WorkerLintOptions,
	WorkerLintResult,
	WorkerParseOptions,
} from "../common/bridges/WorkerBridge";
import Logger from "../common/utils/Logger";
import * as jsAnalysis from "@romejs/js-analysis";
import {ExtensionLintResult} from "../common/file-handlers/types";
import {getFileHandlerAssert} from "../common/file-handlers/index";
import {
	AnalyzeDependencyResult,
	UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
} from "../common/types/analyzeDependencies";
import {
	InlineSnapshotUpdate,
	InlineSnapshotUpdates,
} from "../test-worker/SnapshotManager";
import {formatAST} from "@romejs/formatter";
import {getNodeReferenceParts, valueToNode} from "@romejs/js-ast-utils";

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

	worker: Worker;
	logger: Logger;

	interceptDiagnostics<T extends {
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

	async moduleSignatureJS(ref: FileReference, parseOptions: WorkerParseOptions) {
		const {ast, project} = await this.worker.parse(ref, parseOptions);

		if (ast.type !== "JSRoot") {
			throw new Error(
				`Expected a JSRoot for moduleSignatureJS but got ${ast.type}`,
			);
		}

		this.logger.info(`Generating module signature:`, ref.real.toMarkup());

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

	async updateInlineSnapshots(
		ref: FileReference,
		updates: InlineSnapshotUpdates,
		parseOptions: WorkerParseOptions,
	): Promise<Diagnostics> {
		let {ast, sourceText} = await this.worker.parse(ref, parseOptions);

		const appliedUpdatesToCallees: Set<AnyNode> = new Set();
		const pendingUpdates: Set<InlineSnapshotUpdate> = new Set(updates);
		const context = new CompilerContext({
			ast,
			ref,
		});
		ast = context.reduceRoot(
			ast,
			{
				name: "updateInlineSnapshots",
				enter(path: Path): AnyNode {
					const {node} = path;
					if (node.type !== "JSCallExpression" || pendingUpdates.size === 0) {
						return node;
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
							return node;
						}

						pendingUpdates.delete(matchedUpdate);
						appliedUpdatesToCallees.add(callee);

						const args = node.arguments;
						if (args.length < 1) {
							context.addNodeDiagnostic(
								node,
								descriptions.SNAPSHOTS.INLINE_MISSING_RECEIVED,
							);
							return node;
						}

						return {
							...node,
							arguments: [args[0], valueToNode(matchedUpdate.snapshot)],
						};
					}

					return node;
				},
			},
		);

		const diags = context.diagnostics.getDiagnostics();

		if (pendingUpdates.size > 0 && diags.length === 0) {
			throw new Error(
				`${pendingUpdates.size} left over inline snapshots that were not updated ${JSON.stringify(
					Array.from(pendingUpdates),
				)}`,
			);
		}

		if (diags.length === 0) {
			const formatted = formatAST(ast, {sourceText}).code;
			await this.worker.writeFile(ref.real, formatted);
		}

		return diags;
	}

	async analyzeDependencies(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<AnalyzeDependencyResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(`Analyze dependencies:`, ref.real.toMarkup());

		const {ast, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);

		const {value, diagnostics} = await catchDiagnostics(async () =>
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

		if (diagnostics !== undefined) {
			return {...UNKNOWN_ANALYZE_DEPENDENCIES_RESULT, diagnostics};
		}

		if (value === undefined) {
			return UNKNOWN_ANALYZE_DEPENDENCIES_RESULT;
		}

		return value;
	}

	async workerCompilerOptionsToCompilerOptions(
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

	async compile(
		ref: FileReference,
		stage: TransformStageName,
		options: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<CompileResult> {
		const {ast, project, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);
		this.logger.info(`Compiling:`, ref.real.toMarkup());

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

	async parse(ref: FileReference, opts: WorkerParseOptions): Promise<AnyRoot> {
		let {ast, astModifiedFromSource} = await this.worker.parse(
			ref,
			{
				...opts,
				sourceType: opts.sourceType,
				cache: false,
			},
		);

		return this.interceptDiagnostics(ast, {astModifiedFromSource});
	}

	async format(
		ref: FileReference,
		opts: WorkerParseOptions,
	): Promise<undefined | WorkerFormatResult> {
		const res = await this._format(ref, opts);
		if (res === undefined) {
			return undefined;
		} else {
			return {
				formatted: normalizeFormattedLineEndings(res.sourceText, res.formatted),
				original: res.sourceText,
				diagnostics: res.diagnostics,
			};
		}
	}

	async _format(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<undefined | ExtensionLintResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(`Formatting:`, ref.real.toMarkup());

		const {handler} = getFileHandlerAssert(ref.real, project.config);

		if (!handler.canFormat) {
			return;
		}

		const {customFormat} = handler;
		if (customFormat !== undefined) {
			return await customFormat({
				file: ref,
				project,
				worker: this.worker,
				parseOptions,
			});
		}

		const {ast, sourceText, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);

		const out = formatAST(
			ast,
			{
				sourceText,
			},
		);

		return this.interceptDiagnostics(
			{
				formatted: out.code,
				sourceText,
				suppressions: [],
				diagnostics: ast.diagnostics,
			},
			{astModifiedFromSource},
		);
	}

	async lint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerLintResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(`Linting:`, ref.real.toMarkup());

		// Get the extension handler
		const {handler} = getFileHandlerAssert(ref.real, project.config);

		if (!handler.canLint && !handler.canFormat) {
			return {
				save: undefined,
				diagnostics: [],
				suppressions: [],
			};
		}

		// Catch any diagnostics, in the case of syntax errors etc
		const res = await catchDiagnostics(
			() => {
				if (handler.canLint) {
					return this.compilerLint(ref, options, parseOptions);
				} else {
					return this._format(ref, parseOptions);
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
		}: ExtensionLintResult = res.value;

		const formatted = normalizeFormattedLineEndings(
			sourceText,
			res.value.formatted,
		);

		// If the file has pending fixes
		const needsSave = formatted !== sourceText;

		// Autofix if necessary
		if (options.save && needsSave) {
			return {
				save: formatted,
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
					fixable: true,
					location: {
						filename: ref.uid,
					},
					description: descriptions.LINT.PENDING_FIXES(
						ref.relative.join(),
						sourceText,
						formatted,
					),
				},
			],
		};
	}

	async compilerLint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<ExtensionLintResult> {
		const {ast, sourceText, project, astModifiedFromSource} = await this.worker.parse(
			ref,
			parseOptions,
		);

		// Run the compiler in lint-mode which is where all the rules are actually ran
		const res = await lint({
			applyRecommendedFixes: options.applyRecommendedFixes,
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
		let typeCheckingEnabled = project.config.typeCheck.enabled === true;
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
			},
			{astModifiedFromSource},
		);
	}
}
