/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSRoot, MOCK_PARENT} from "@romejs/ast";
import {CompilerContext} from "@romejs/compiler";
import {SCOPE_PRIVATE_PREFIX} from "../constants";
import evaluators from "./evaluators/index";
import * as GLOBALS from "./globals";
import {Binding} from "./bindings";
import {
	isValidIdentifierName,
	isVariableIdentifier,
} from "@romejs/js-ast-utils";
import Path from "../lib/Path";

Error.stackTraceLimit = Infinity;

type ScopeBindings = Map<string, Binding>;

export type ScopeKind =
	| "root"
	| "program"
	| "function"
	| "block"
	| "loop"
	| "type-generic"
	| "class";

export default class Scope {
	constructor(
		{
			kind,
			node,
			parentScope,
			rootScope,
		}: {
			kind: ScopeKind;
			node: undefined | AnyNode;
			parentScope: undefined | Scope;
			rootScope: undefined | RootScope;
		},
	) {
		this.parentScope = parentScope;
		this.rootScope = rootScope;
		this.node = node;
		this.kind = kind;
		this.bindings = new Map();
		this.hasHoistedVars = false;
		this.globals = new Set();

		this.childScopeCache = new WeakMap();
	}

	childScopeCache: WeakMap<AnyNode, Scope>;
	bindings: ScopeBindings;
	rootScope: undefined | RootScope;
	parentScope: undefined | Scope;
	globals: Set<string>;
	node: undefined | AnyNode;
	kind: ScopeKind;
	hasHoistedVars: boolean;

	setHoistedVars() {
		this.hasHoistedVars = true;
	}

	hasBindings(): boolean {
		return this.bindings.size > 0;
	}

	getOwnBindings(): ScopeBindings {
		return this.bindings;
	}

	getOwnBindingNames(): Array<string> {
		return Array.from(this.bindings.keys());
	}

	findScope(kind: ScopeKind): undefined | Scope {
		let scope: undefined | Scope = this;
		while (scope !== undefined) {
			if (scope.kind === kind) {
				return scope;
			} else {
				scope = scope.parentScope;
			}
		}
		return undefined;
	}

	getRootScope(): RootScope {
		const {rootScope} = this;
		if (rootScope === undefined) {
			throw new Error(`Expected rootScope`);
		}
		return rootScope;
	}

	enterEvaluate(
		node: undefined | AnyNode,
		parent: AnyNode = MOCK_PARENT,
		force: boolean = false,
	): Scope {
		if (node === undefined) {
			return this;
		}

		if (!force && node === this.node) {
			return this;
		}

		const cached = this.childScopeCache.get(node);
		if (cached !== undefined) {
			return cached;
		}

		const evaluator = evaluators.get(node.type);
		if (evaluator === undefined || evaluator.enter === undefined) {
			return this;
		}

		let scope = evaluator.enter(node, parent, this);
		this.childScopeCache.set(node, scope);

		return scope;
	}

	injectEvaluate(node: undefined | AnyNode, parent: AnyNode = MOCK_PARENT): void {
		if (node === undefined) {
			return;
		}

		const evaluator = evaluators.get(node.type);
		if (evaluator === undefined || evaluator.inject === undefined) {
			return;
		}

		evaluator.inject(node, parent, this);
	}

	fork(kind: ScopeKind, node: undefined | AnyNode): Scope {
		const rootScope = this.getRootScope();
		return new Scope({
			kind,
			node,
			parentScope: this,
			rootScope,
		});
	}

	// Debug utility for dumping scope information
	dump(): string {
		const lines = [];

		lines.push(`# Scope ${this.kind}`);

		if (this.globals.size > 0) {
			const filteredGlobals: Array<string> = [];
			for (const name of this.globals) {
				if (globalGlobals.includes(name)) {
					continue;
				}

				filteredGlobals.push(name);
			}

			if (filteredGlobals.length > 0) {
				lines.push("## Globals");

				for (const name of filteredGlobals) {
					lines.push(` * ${name}`);
				}
			}
		}

		if (this.bindings.size > 0) {
			lines.push("## Variables");
			for (const [name, binding] of this.bindings) {
				lines.push(` * ${binding.kind} ${name}`);
			}
		}

		return lines.join("\n");
	}

	dumpAncestry() {
		const lines = [];
		lines.push(this.dump());
		if (this.parentScope !== undefined) {
			lines.push(this.parentScope.dump());
		}
		return lines.join("\n");
	}

	getOwnBinding(name: string): undefined | Binding {
		return this.bindings.get(name);
	}

	getBindingFromPath(path: Path): undefined | Binding {
		const {node} = path;
		if (isVariableIdentifier(node)) {
			// TODO we can do some isInTypeAnnotation magic to get the proper "type" binding
			return this.getBinding(node.name);
		} else {
			return undefined;
		}
	}

	getBinding(name: string): undefined | Binding {
		const binding = this.bindings.get(name);
		if (binding !== undefined) {
			return binding;
		}

		const {parentScope} = this;
		if (parentScope !== undefined) {
			return parentScope.getBinding(name);
		}

		return undefined;
	}

	getBindingAssert(name: string): Binding {
		const binding = this.getBinding(name);
		if (binding === undefined) {
			this.dump();
			throw new Error(`Expected ${name} binding`);
		}
		return binding;
	}

	addBinding(binding: Binding): Binding {
		this.bindings.set(binding.name, binding);
		return binding;
	}

	hasBinding(name: string): boolean {
		return this.getBinding(name) !== undefined;
	}

	generateUid(name?: string): string {
		return this.getRootScope().generateUid(name);
	}

	addGlobal(name: string) {
		this.globals.add(name);
	}

	isGlobal(name: string): boolean {
		if (this.globals.has(name)) {
			return true;
		}

		if (this.parentScope !== undefined) {
			return this.parentScope.isGlobal(name);
		}

		return false;
	}
}

const GLOBAL_COMMENT_START = /^([\s+]|)global /;
const GLOBAL_COMMENT_COLON = /:(.*?)$/;

// lol global globals
const globalGlobals = [
	...GLOBALS.builtin,
	...GLOBALS.es5,
	...GLOBALS.es2015,
	...GLOBALS.es2017,
	...GLOBALS.browser,
	...GLOBALS.worker,
	...GLOBALS.node,
	...GLOBALS.commonjs,
	...GLOBALS.serviceworker,
];

export class RootScope extends Scope {
	constructor(context: CompilerContext, ast: JSRoot) {
		super({
			kind: "root",
			parentScope: undefined,
			rootScope: undefined,
			node: undefined,
		});
		this.uids = new Set();
		this.context = context;

		this.globals = new Set([
			...globalGlobals,
			...context.project.config.lint.globals,
			...this.parseGlobalComments(ast),
		]);
	}

	context: CompilerContext;
	uids: Set<string>;

	parseGlobalComments(ast: JSRoot): Array<string> {
		const globals: Array<string> = [];

		for (const {value} of ast.comments) {
			// Check if comment starts with "global ", ignoring any leading whitespace
			if (!GLOBAL_COMMENT_START.test(value)) {
				continue;
			}

			// Remove prefix
			const clean = value.replace(GLOBAL_COMMENT_START, "");

			// Split by commas, supports comments like "foo, bar"
			const parts = clean.split(",");

			for (const part of parts) {
				let name = part.trim();

				// Support comments like "foo: true" amd "bar: false"
				if (GLOBAL_COMMENT_COLON.test(name)) {
					const match = part.match(GLOBAL_COMMENT_COLON);
					if (match == null) {
						throw new Error(
							"Used RegExp.test already so know this will always match",
						);
					}

					// Remove everything after the colon
					name = name.replace(GLOBAL_COMMENT_COLON, "");

					const value = match[1].trim();

					// Other tools would flag these as unavailable and remove them from the master set
					// We don't do that, we might want to later though?
					// Also, we should maybe validate the value to only true/false
					if (value === "false") {
						break;
					}
				}

				globals.push(name);
			}
		}

		return globals;
	}

	getRootScope(): RootScope {
		return this;
	}

	generateUid(name?: string): string {
		const prefixed = `${SCOPE_PRIVATE_PREFIX}${name === undefined ? "" : name}`;

		// Check for invalid names
		if (name !== undefined && !isValidIdentifierName(name)) {
			throw new Error(`${name} is not a valid identifier name`);
		}

		// TODO find some way to remove the possibility of user bindings colliding with our private prefix
		let counter = 0;

		while (true) {
			const suffix = counter === 0 ? "" : String(counter);
			const name = prefixed + suffix;

			if (this.uids.has(name)) {
				counter++;
			} else {
				this.uids.add(name);
				return name;
			}
		}

		throw new Error("Unreachable");
	}
}
