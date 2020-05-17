/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Intrinsics from "./Intrinsics";
import Evaluator from "./Evaluator";
import T from "./types/T";
import Hub from "./Hub";
import {AnyNode} from "@romejs/js-ast";
import StringLiteralT from "./types/StringLiteralT";
import UnknownT from "./types/UnknownT";
import GetPropT from "./types/GetPropT";
import UnionT from "./types/UnionT";
import OpenT from "./types/OpenT";
import {Class} from "@romejs/typescript-helpers";

type BindingStatus = "declared" | "initialized";

type Binding = {
	type: T;
	status: BindingStatus;
};

export type ScopeOptions = {
	evaluator?: Evaluator;
	parentScope?: Scope;
};

export class Scope {
	constructor(opts: ScopeOptions) {
		let {evaluator, parentScope} = opts;
		if (evaluator === undefined && parentScope !== undefined) {
			evaluator = parentScope.evaluator;
		}

		if (evaluator === undefined) {
			throw new Error("No evaluator was passed or inferred");
		}

		this.intrinsics = evaluator.intrinsics;
		this.evaluator = evaluator;
		this.hub = evaluator.hub;
		this.parentScope = parentScope;

		this.bindings = new Map();
	}

	hub: Hub;
	intrinsics: Intrinsics;
	evaluator: Evaluator;
	parentScope: undefined | Scope;
	bindings: Map<string, Binding>;

	getBinding(name: string): undefined | T {
		let scope: undefined | Scope = this;
		while (scope) {
			const binding = scope.bindings.get(name);
			if (binding) {
				return binding.type;
			}
			scope = scope.parentScope;
		}
		return undefined;
	}

	getBindingAssert(name: string): T {
		const binding = this.getBinding(name);
		if (binding === undefined) {
			throw new Error(`Expected binding ${name}`);
		}
		return binding;
	}

	query(paths: Array<string>): T {
		let initial = this.getBinding(paths[0]);
		if (initial === undefined) {
			throw new Error(
				`Expected "${paths[0]}" binding, found ${JSON.stringify(
					this.getBindingNames(),
				)} ${this.evaluator.filename}`,
			);
		}

		//invariant(initial !== undefined, `Expected "${paths[0]}" binding`);
		for (let i = 1; i < paths.length; i++) {
			initial = new GetPropT(
				this,
				undefined,
				initial,
				new StringLiteralT(this, undefined, paths[i]),
			);
		}

		return initial;
	}

	declareBinding(name: string, originNode: AnyNode) {
		if (name === undefined) {
			throw new Error("Expected name");
		}
		this.bindings.set(
			name,
			{
				type: new OpenT(this, originNode),
				status: "declared",
			},
		);
	}

	addBinding(name: string, type: T) {
		if (name === undefined) {
			throw new Error("Expected name");
		}

		const existingBinding = this.bindings.get(name);
		if (existingBinding !== undefined && existingBinding.status === "declared") {
			if (!(existingBinding.type instanceof OpenT)) {
				throw new Error("expected OpenT");
			}

			existingBinding.type.shouldMatch(type);
		}

		this.bindings.set(
			name,
			{
				type,
				status: "initialized",
			},
		);
	}

	getBindingNames(): Array<string> {
		const names: Set<string> = new Set(
			this.parentScope ? this.parentScope.getBindingNames() : [],
		);

		for (const [name] of this.bindings) {
			names.add(name);
		}

		return Array.from(names);
	}

	getOwnBindingNames(): Array<string> {
		return Array.from(this.bindings.keys());
	}

	createUnion(types: Array<T>, originNode?: AnyNode): T {
		if (types.length === 0) {
			return new UnknownT(this, originNode);
		} else if (types.length === 1) {
			return types[0];
		} else {
			return new UnionT(this, originNode, types);
		}
	}

	fork() {
		return new Scope({evaluator: this.evaluator, parentScope: this});
	}

	find<S extends Scope>(klass: Class<S>): S {
		const scope = this.findOptional(klass);
		if (scope === undefined) {
			throw new Error("Failed to find class");
		} else {
			return scope;
		}
	}

	findOptional<S extends Scope>(klass: Class<S>): undefined | S {
		let scope: undefined | Scope = this;

		do {
			if (scope instanceof klass) {
				return scope;
			}

			scope = scope.parentScope;
		} while (scope !== undefined);

		return undefined;
	}

	refine(): Scope {
		return new RefineScope({evaluator: this.evaluator, parentScope: this});
	}

	evaluate(node: undefined | AnyNode): T {
		return this.evaluator.evaluate(node, this);
	}
}

//#
export class RefineScope extends Scope {}

//#
type ClassScopeMeta = {
	instance: OpenT;
	static: OpenT;
};

export class ClassScope extends Scope {
	constructor(opts: ScopeOptions, meta: ClassScopeMeta) {
		super(opts);
		this.meta = meta;
	}

	meta: ClassScopeMeta;
}

//#
export class ThisScope extends Scope {
	constructor(opts: ScopeOptions, context: T) {
		super(opts);
		this.context = context;
	}

	context: T;
}

//#
type FunctionScopeMeta = {
	thisContext: T;
	returnType: OpenT;
};

export class FunctionScope extends ThisScope {
	constructor(opts: ScopeOptions, meta: FunctionScopeMeta) {
		super(opts, meta.thisContext);
		this.meta = meta;
	}

	meta: FunctionScopeMeta;
}
