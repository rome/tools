/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyNodes, MOCK_PARENT} from "@internal/ast";
import {
	AnyVisitor,
	CompilerContext,
	ExitSignal,
	Scope,
	signals,
} from "@internal/compiler";
import {reduceNode, reduceSignal} from "../methods/reduce";
import {isRoot} from "@internal/ast-utils";
import {formatAST} from "@internal/formatter";
import {StaticMarkup, markup} from "@internal/markup";
import {DiagnosticDescription, equalCategoryNames} from "@internal/diagnostics";
import {ContextDiagnostic} from "./CompilerContext";
import {
	buildLintDecisionAdviceAction,
	buildLintDecisionGlobalString,
	buildLintDecisionString,
	deriveDecisionPositionKey,
} from "../lint/decisions";
import {inheritLoc} from "@internal/js-ast-utils";
import {stringDiffCompressed} from "@internal/string-diff";

// Can be used with referential equality to determine if paths are derivatives of each other
// Import for state retention which requires tracking ownership
export type CompilerPathToken = {
	type: "COMPILER_PATH_TOKEN";
};

export type CompilerPathOptions = {
	ancestryPaths?: CompilerPath[];
	nodeKey?: string;
	listKey?: number;
	parentScope?: Scope;
	scope?: Scope;
	noArrays?: boolean;
	noScopeCreation?: boolean;
	isMock?: boolean;
};

// Given a signal, calculate what the formatted code would be
function getFormattedCodeFromSignal(
	signal: ExitSignal,
	path: CompilerPath,
): string {
	switch (signal.type) {
		case "REMOVE":
			return "";

		case "REPLACE": {
			const {value} = signal;
			if (Array.isArray(value)) {
				return value.map((node) => {
					return formatAST(node).code;
				}).filter((str) => str !== "").join("\n");
			} else {
				return formatAST(value).code;
			}
		}

		case "PARENT": {
			for (const possiblePath of path.ancestryPaths) {
				if (possiblePath.node === signal.parent) {
					return getFormattedCodeFromSignal(signal.signal, path);
				}
			}

			// Will later be an error in reduce since this parent was not found
			return "";
		}

		case "RETAIN":
			return getFormattedCodeFromSignal(signals.replace(path.node), path);
	}
}

export default class CompilerPath {
	constructor(
		node: AnyNode,
		context: CompilerContext,
		opts: CompilerPathOptions,
		token?: CompilerPathToken,
	) {
		const ancestryPaths = opts.ancestryPaths || [];
		this.ancestryPaths = ancestryPaths;

		if (node === MOCK_PARENT) {
			this.parentPath = this;
		} else if (ancestryPaths.length === 0) {
			this.parentPath = new CompilerPath(
				MOCK_PARENT,
				context,
				{
					isMock: true,
				},
			);
		} else {
			this.parentPath = ancestryPaths[0];
		}

		this.node = node;
		this.parent = this.parentPath.node;
		this.context = context;

		const parentScope =
			opts.parentScope === undefined || isRoot(node)
				? context.rootScope
				: opts.parentScope;

		let scope = opts.scope;
		if (scope === undefined) {
			if (opts.noScopeCreation === true) {
				scope = parentScope;
			} else {
				scope = parentScope.enterEvaluate(node, this.parent);
			}
		}
		this.scope = scope;

		this.nodeKey = opts.nodeKey;
		this.listKey = opts.listKey;

		this.isMock = opts.isMock === true;
		this.opts = opts;
		this.token = token ?? {type: "COMPILER_PATH_TOKEN"};
	}

	public context: CompilerContext;
	public node: AnyNode;
	public parent: AnyNode;
	public scope: Scope;
	public ancestryPaths: CompilerPath[];
	public parentPath: CompilerPath;
	public token: CompilerPathToken;
	public opts: CompilerPathOptions;

	private isMock: boolean;
	private nodeKey: undefined | string;
	private listKey: undefined | number;

	public findAncestry(
		callback: (path: CompilerPath) => boolean,
	): undefined | CompilerPath {
		for (const path of this.ancestryPaths) {
			if (callback(path)) {
				return path;
			}
		}
		return undefined;
	}

	public getChildPath(key: string): CompilerPath {
		// rome-ignore lint/ts/noExplicitAny: future cleanup
		const node = (this.node as any)[key];
		if (node === undefined) {
			throw new Error(
				`Attempted to get child path for ${key} but no such node existed`,
			);
		}

		return new CompilerPath(
			node,
			this.context,
			{
				parentScope: this.scope,
				ancestryPaths: [this, ...this.ancestryPaths],
				nodeKey: key,
			},
		);
	}

	public getChildPaths(key: string): CompilerPath[] {
		// rome-ignore lint/ts/noExplicitAny: future cleanup
		const nodes = (this.node as any)[key];

		if (nodes === undefined) {
			throw new Error(
				`Attempted to get child paths for ${key} but no such node existed`,
			);
		}

		if (!Array.isArray(nodes)) {
			throw new Error(`Expected child nodes for ${key} to be an array`);
		}

		const ancestryPaths = this.ancestryPaths.concat([this]);

		return nodes.map((node: AnyNode, i: number) => {
			return new CompilerPath(
				node,
				this.context,
				{
					parentScope: this.scope,
					ancestryPaths,
					listKey: i,
					nodeKey: key,
				},
			);
		});
	}

	public getPathKeys(): string[] {
		const parts = [];

		let path: undefined | CompilerPath = this;
		while (path !== undefined && !path.isMock) {
			if (path.listKey !== undefined) {
				parts.push(String(path.listKey));
			}
			if (path.nodeKey !== undefined) {
				parts.push(path.nodeKey);
			}
			path = path.parentPath;
		}

		return parts.reverse();
	}

	public fork(newNode: AnyNode): CompilerPath {
		return new CompilerPath(
			newNode,
			this.context,
			this.getPathOptions(),
			this.token,
		);
	}

	private getPathOptions(): CompilerPathOptions {
		return {
			...this.opts,
			parentScope: this.scope === undefined ? undefined : this.scope.parentScope,
		};
	}

	public traverse(name: string, callback: (path: CompilerPath) => void) {
		this.reduceNode({
			name,
			enter(path) {
				callback(path);
				return signals.retain;
			},
		});
	}

	public reduceNode(
		visitors: AnyVisitor | AnyVisitor[],
		opts?: Partial<CompilerPathOptions>,
	): AnyNodes {
		return reduceNode(
			this.node,
			visitors,
			this.context,
			{...this.getPathOptions(), ...opts},
		);
	}

	public reduceSignal(
		visitors: AnyVisitor | AnyVisitor[],
		opts?: Partial<CompilerPathOptions>,
	): ExitSignal {
		return reduceSignal(
			this.node,
			visitors,
			this.context,
			{...this.getPathOptions(), ...opts},
		);
	}

	public addFixableDiagnostic(
		nodes: {
			target?: AnyNode | (AnyNode[]);
			fixed?: ExitSignal;
			suggestions?: {
				description: StaticMarkup;
				title: StaticMarkup;
				fixed: ExitSignal;
			}[];
		},
		description: DiagnosticDescription,
		diag: ContextDiagnostic = {},
	): ExitSignal {
		const old = this.node;
		const {context} = this;
		const {fixed: defaultFixed, suggestions} = nodes;
		const target = nodes.target ?? old;
		const {category, categoryValue} = description;
		const advice = [...description.advice];
		const verboseAdvice = [...(description.verboseAdvice ?? [])];
		const loc = context.getLoc(target);
		const canFormat = this.context.project.config.format.enabled;

		let fixed: undefined | ExitSignal = defaultFixed;

		if (nodes.target !== undefined) {
			// NB: The diff is going to refer to the old but diagnostic will be pointing to a different
			// location. Probably ok since you can mentally infer from context but we could add a log if it's
			// confusing.
		}

		// Add recommended fix
		if (defaultFixed !== undefined) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Safe fix`,
			});

			advice.push({
				type: "diff",
				language: context.language,
				diff: stringDiffCompressed(
					getFormattedCodeFromSignal(signals.replace(old), this),
					getFormattedCodeFromSignal(defaultFixed, this),
				),
			});

			if (canFormat) {
				if (loc === undefined) {
					advice.push({
						type: "log",
						category: "error",
						text: markup`Unable to find target location`,
					});
				} else {
					verboseAdvice.push(
						buildLintDecisionAdviceAction({
							path: context.displayPath,
							decision: buildLintDecisionString({
								action: "fix",
								path: context.displayPath,
								category,
								categoryValue,
								start: loc.start,
							}),
							shortcut: "f",
							description: markup`Apply this fix`,
						}),
					);

					verboseAdvice.push(
						buildLintDecisionAdviceAction({
							secondary: true,
							description: markup`Apply fix for ALL files with this category`,
							decision: buildLintDecisionGlobalString(
								"fix",
								category,
								undefined,
							),
						}),
					);
				}
			}
		}

		if (suggestions !== undefined) {
			// If we have lint decisions then find the fix that corresponds with this suggestion
			if (context.hasLintDecisions()) {
				const decisions = context.getLintDecisions(
					deriveDecisionPositionKey("fix", loc),
				);
				for (const decision of decisions) {
					if (
						equalCategoryNames(decision.category, category) &&
						(decision.categoryValue === undefined ||
						decision.categoryValue === categoryValue) &&
						decision.action === "fix" &&
						decision.id !== undefined
					) {
						const suggestion = suggestions[decision.id];
						if (suggestion !== undefined) {
							fixed = suggestion.fixed;
						}
					}
				}
			}

			// Add advice suggestions
			let index = 0;
			for (const suggestion of suggestions) {
				const num = index + 1;

				const titlePrefix =
					suggestions.length === 1 ? "Suggested fix" : `Suggested fix #${num}`;
				advice.push({
					type: "log",
					category: "none",
					text: markup`<emphasis>${titlePrefix}:</emphasis> ${suggestion.title}`,
				});

				advice.push({
					type: "diff",
					language: context.language,
					diff: stringDiffCompressed(
						getFormattedCodeFromSignal(signals.replace(old), this),
						getFormattedCodeFromSignal(suggestion.fixed, this),
					),
				});

				advice.push({
					type: "log",
					category: "info",
					text: suggestion.description,
				});

				if (canFormat) {
					if (loc === undefined) {
						advice.push({
							type: "log",
							category: "error",
							text: markup`Unable to find target location`,
						});
					} else {
						verboseAdvice.push(
							buildLintDecisionAdviceAction({
								description: suggestions.length === 1
									? markup`Apply suggested fix`
									: markup`Apply suggested fix "${suggestion.title}"`,
								shortcut: String(num),
								path: context.displayPath,
								decision: buildLintDecisionString({
									path: context.displayPath,
									action: "fix",
									category,
									categoryValue,
									start: loc.start,
									id: index,
								}),
							}),
						);
					}
				}

				index++;
			}
		}

		const {suppressed} = context.addLocDiagnostic(
			loc,
			{
				...description,
				advice,
				verboseAdvice,
			},
			{
				...diag,
				tags: {
					...diag.tags,
					fixable: true,
				},
			},
		);

		if (suppressed || fixed === undefined) {
			return signals.replace(old);
		}

		// Inherit old location if necessary
		if (fixed.type === "REPLACE") {
			let value = fixed.value;
			if (Array.isArray(value)) {
				// For arrays the first one gets the loc
				const [first, ...rest] = value;
				if (first.loc === undefined) {
					value = [
						{
							...first,
							loc: inheritLoc(old),
						},
						...rest,
					];
				}
			} else {
				value = {
					...value,
					loc: inheritLoc(old),
				};
			}
			fixed = {...fixed, value};
		}

		return fixed;
	}
}
