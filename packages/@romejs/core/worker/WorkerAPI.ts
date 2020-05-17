/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference, Worker} from "@romejs/core";
import {AnyNode, Program} from "@romejs/js-ast";
import {Diagnostics, catchDiagnostics, descriptions} from "@romejs/diagnostics";
import {
	CompileResult,
	CompilerContext,
	CompilerOptions,
	Path,
	TransformStageName,
	compile,
} from "@romejs/js-compiler";
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
import {formatJS} from "@romejs/js-formatter";
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

	interceptAndAddGeneratedToDiagnostics<T extends {
		diagnostics: Diagnostics;
	}>(val: T, generated: boolean): T {
		if (generated) {
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
								text: "This diagnostic was generated on a file that has been converted to JavaScript. The source locations are most likely incorrect",
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
		const {ast, project} = await this.worker.parseJS(ref, parseOptions);

		this.logger.info(`Generating export types:`, ref.real);

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
		let {ast, sourceText} = await this.worker.parseJS(ref, parseOptions);

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
					if (node.type !== "CallExpression" || pendingUpdates.size === 0) {
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
							if (loc.start.column === update.column && loc.start.line === update.line) {
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
							context.addNodeDiagnostic(node, descriptions.SNAPSHOTS.INLINE_COLLISION);
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
			throw new Error("Left over inline snapshots that were not updated");
		}

		if (diags.length === 0) {
			const formatted = formatJS(ast, {sourceText}).code;
			await this.worker.writeFile(ref.real, formatted);
		}

		return diags;
	}

	async analyzeDependencies(
		ref: FileReference,
		parseOptions: WorkerParseOptions,
	): Promise<AnalyzeDependencyResult> {
		const project = this.worker.getProject(ref.project);
		const {handler} = getFileHandlerAssert(ref.real, project.config);
		this.logger.info(`Analyze dependencies:`, ref.real);

		const {analyzeDependencies} = handler;
		if (analyzeDependencies === undefined) {
			return UNKNOWN_ANALYZE_DEPENDENCIES_RESULT;
		}

		return await analyzeDependencies({
			file: ref,
			project,
			worker: this.worker,
			parseOptions,
		});
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

	async compileJS(
		ref: FileReference,
		stage: TransformStageName,
		options: WorkerCompilerOptions,
		parseOptions: WorkerParseOptions,
	): Promise<CompileResult> {
		const {ast, project, sourceText, generated} = await this.worker.parseJS(
			ref,
			parseOptions,
		);
		this.logger.info(`Compiling:`, ref.real);

		const compilerOptions = await this.workerCompilerOptionsToCompilerOptions(
			ref,
			options,
			parseOptions,
		);
		return this.interceptAndAddGeneratedToDiagnostics(
			await compile({
				ref,
				ast,
				sourceText,
				options: compilerOptions,
				project,
				stage,
			}),
			generated,
		);
	}

	async parseJS(ref: FileReference, opts: WorkerParseOptions): Promise<Program> {
		let {ast, generated} = await this.worker.parseJS(
			ref,
			{
				...opts,
				sourceType: opts.sourceType,
				cache: false,
			},
		);

		return this.interceptAndAddGeneratedToDiagnostics(ast, generated);
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
		this.logger.info(`Formatting:`, ref.real);

		const {handler} = getFileHandlerAssert(ref.real, project.config);
		const {format} = handler;
		if (format === undefined) {
			return;
		}

		const res = await format({
			file: ref,
			project,
			worker: this.worker,
			parseOptions,
		});

		return res;
	}

	async lint(
		ref: FileReference,
		options: WorkerLintOptions,
		parseOptions: WorkerParseOptions,
	): Promise<WorkerLintResult> {
		const project = this.worker.getProject(ref.project);
		this.logger.info(`Linting:`, ref.real);

		// Get the extension handler
		const {handler} = getFileHandlerAssert(ref.real, project.config);

		const {lint} = handler;
		if (lint === undefined && handler.format === undefined) {
			return {
				saved: false,
				diagnostics: [],
				suppressions: [],
			};
		}

		// Catch any diagnostics, in the case of syntax errors etc
		const res = await catchDiagnostics(
			() => {
				if (lint === undefined) {
					return this._format(ref, parseOptions);
				} else {
					return lint({
						file: ref,
						project,
						worker: this.worker,
						options,
						parseOptions,
					});
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
				saved: false,
				suppressions: [],
				diagnostics: res.diagnostics,
			};
		}

		// `format` could have return undefined
		if (res.value === undefined) {
			return {
				saved: false,
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
			// Save the file and evict it from the cache
			await this.worker.writeFile(ref.real, formatted);

			// Relint this file without fixing it, we do this to prevent false positive error messages
			return {
				...(await this.lint(ref, {...options, save: false}, parseOptions)),
				saved: true,
			};
		}

		// If there's no pending fix then no need for diagnostics
		if (!needsSave) {
			return {
				saved: false,
				diagnostics,
				suppressions,
			};
		}

		// Add pending autofix diagnostic
		return {
			saved: false,
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
}
