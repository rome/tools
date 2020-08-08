/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyNodes, MOCK_PARENT} from "@internal/ast";
import {
	AnyVisitors,
	CompilerContext,
	ExitSignal,
	Scope,
	signals,
} from "@internal/compiler";
import {reduceNode, reduceSignal} from "../methods/reduce";
import {isRoot} from "@internal/ast-utils";
import stringDiff from "@internal/string-diff";
import {formatAST} from "@internal/formatter";
import {StaticMarkup, markup} from "@internal/markup";
import {DiagnosticDescription} from "@internal/diagnostics";
import {ContextDiagnostic} from "./CompilerContext";
import {
	buildLintDecisionAdviceAction,
	buildLintDecisionGlobalString,
	buildLintDecisionString,
	deriveDecisionPositionKey,
} from "../lint/decisions";
import {AnyVisitor} from "../types";
import {inheritLoc} from "@internal/js-ast-utils";

// Can be used with referential equality to determine if paths are derivatives of each other
// Import for state retention which requires tracking ownership
export type PathToken = {
	type: "PATH_TOKEN";
};

export type PathOptions = {
	ancestryPaths?: Array<Path>;
	nodeKey?: string;
	listKey?: number;
	parentScope?: Scope;
	scope?: Scope;
	noArrays?: boolean;
	noScopeCreation?: boolean;
	isMock?: boolean;
};

// Given a signal, calculate what the formatted code would be
function getFormattedCodeFromSignal(signal: ExitSignal, path: Path): string {
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

export default class Path {
	constructor(
		node: AnyNode,
		context: CompilerContext,
		opts: PathOptions,
		token?: PathToken,
	) {
		const ancestryPaths = opts.ancestryPaths || [];
		this.ancestryPaths = ancestryPaths;

		if (node === MOCK_PARENT) {
			this.parentPath = this;
		} else if (ancestryPaths.length === 0) {
			this.parentPath = new Path(
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
		this.token = token ?? {type: "PATH_TOKEN"};
	}

	public context: CompilerContext;
	public node: AnyNode;
	public parent: AnyNode;
	public scope: Scope;
	public ancestryPaths: Array<Path>;
	public parentPath: Path;
	public token: PathToken;
	public opts: PathOptions;

	private isMock: boolean;
	private nodeKey: undefined | string;
	private listKey: undefined | number;

	public findAncestry(callback: (path: Path) => boolean): undefined | Path {
		for (const path of this.ancestryPaths) {
			if (callback(path)) {
				return path;
			}
		}
		return undefined;
	}

	public getChildPath(key: string): Path {
		// rome-ignore lint/ts/noExplicitAny
		const node = (this.node as any)[key];
		if (node === undefined) {
			throw new Error(
				`Attempted to get child path for ${key} but no such node existed`,
			);
		}

		return new Path(
			node,
			this.context,
			{
				parentScope: this.scope,
				ancestryPaths: this.ancestryPaths.concat([this]),
				nodeKey: key,
			},
		);
	}

	public getChildPaths(key: string): Array<Path> {
		// rome-ignore lint/ts/noExplicitAny
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
			return new Path(
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

	public getPathKeys(): Array<string> {
		const parts = [];

		let path: undefined | Path = this;
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

	public fork(newNode: AnyNode): Path {
		return new Path(newNode, this.context, this.getPathOptions(), this.token);
	}

	private getPathOptions(): PathOptions {
		return {
			...this.opts,
			parentScope: this.scope === undefined ? undefined : this.scope.parentScope,
		};
	}

	public traverse(name: string, callback: (path: Path) => void) {
		this.reduceNode({
			name,
			enter(path) {
				callback(path);
				return signals.retain;
			},
		});
	}

	public reduceNode(
		visitors: AnyVisitor | AnyVisitors,
		opts?: Partial<PathOptions>,
	): AnyNodes {
		return reduceNode(
			this.node,
			visitors,
			this.context,
			{...this.getPathOptions(), ...opts},
		);
	}

	public reduceSignal(
		visitors: AnyVisitor | AnyVisitors,
		opts?: Partial<PathOptions>,
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
			target?: AnyNode | Array<AnyNode>;
			fixed?: ExitSignal;
			suggestions?: Array<{
				description: StaticMarkup;
				title: StaticMarkup;
				fixed: ExitSignal;
			}>;
		},
		description: DiagnosticDescription,
		diag: ContextDiagnostic = {},
	): ExitSignal {
		const old = this.node;
		const {context} = this;
		const {fixed: defaultFixed, suggestions} = nodes;
		const target = nodes.target ?? old;

		const {category} = description;
		const advice = [...description.advice];
		const loc = context.getLoc(target);

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
				diff: stringDiff(
					getFormattedCodeFromSignal(signals.replace(old), this),
					getFormattedCodeFromSignal(defaultFixed, this),
				),
			});

			if (loc === undefined) {
				advice.push({
					type: "log",
					category: "error",
					text: markup`Unable to find target location`,
				});
			} else {
				advice.push(
					buildLintDecisionAdviceAction({
						filename: context.displayFilename,
						decision: buildLintDecisionString({
							action: "fix",
							filename: context.displayFilename,
							category,
							start: loc.start,
						}),
						shortcut: "f",
						noun: markup`Apply fix`,
						instruction: markup`To apply this fix run`,
					}),
				);

				advice.push(
					buildLintDecisionAdviceAction({
						extra: true,
						noun: markup`Apply fix for ALL files with this category`,
						instruction: markup`To apply fix for ALL files with this category run`,
						decision: buildLintDecisionGlobalString("fix", category),
					}),
				);
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
						decision.category === category &&
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
					diff: stringDiff(
						getFormattedCodeFromSignal(signals.replace(old), this),
						getFormattedCodeFromSignal(suggestion.fixed, this),
					),
				});

				advice.push({
					type: "log",
					category: "info",
					text: suggestion.description,
				});

				if (loc === undefined) {
					advice.push({
						type: "log",
						category: "error",
						text: markup`Unable to find target location`,
					});
				} else {
					advice.push(
						buildLintDecisionAdviceAction({
							noun: suggestions.length === 1
								? markup`Apply suggested fix`
								: markup`Apply suggested fix "${suggestion.title}"`,
							shortcut: String(num),
							instruction: markup`To apply this fix run`,
							filename: context.displayFilename,
							decision: buildLintDecisionString({
								filename: context.displayFilename,
								action: "fix",
								category,
								start: loc.start,
								id: index,
							}),
						}),
					);
				}

				index++;
			}
		}

		const {suppressed} = context.addLocDiagnostic(
			loc,
			{
				...description,
				advice,
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
		}

		return fixed;
	}
}
