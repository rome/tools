/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Scope from "./Scope";
import {AnyNode, ConstJSImportModuleKind} from "@internal/ast";

type BindingOpts = {
	scope: Scope;
	node: AnyNode;
	name: string;
	kind?: BindingKind;
};

export type BindingKind =
	| "let"
	| "const"
	| "var"
	| "type"
	| "class"
	| "import"
	| "arguments"
	| "function"
	| "catch"
	| "parameter";

export class Binding {
	constructor(opts: BindingOpts, defaultKind: BindingKind) {
		this.isExported = false;
		this.scope = opts.scope;
		this.name = opts.name;
		this.node = opts.node;
		this.kind = opts.kind ?? defaultKind;
	}

	public name: string;
	public kind: BindingKind;
	public scope: Scope;
	public node: AnyNode;
	public isExported: boolean;

	public setExported(isExported: boolean) {
		this.isExported = isExported;
	}
}

export type ImportBindingMeta =
	| {
			type: "name";
			imported: string;
			source: string;
			kind: ConstJSImportModuleKind;
		}
	| {
			type: "namespace";
			source: string;
			kind: ConstJSImportModuleKind;
		};

export class ConstBinding extends Binding {
	constructor(
		opts: BindingOpts,
		value: undefined | AnyNode,
		defaultKind: BindingKind = "const",
	) {
		super(opts, defaultKind);
		this.value = value;
	}

	public value: undefined | AnyNode;
}

export class LetBinding extends Binding {
	constructor(opts: BindingOpts) {
		super(opts, "let");
	}
}

export class VarBinding extends Binding {
	constructor(opts: BindingOpts) {
		super(opts, "var");
	}
}

export class ImportBinding extends Binding {
	constructor(opts: BindingOpts, meta: ImportBindingMeta) {
		super(opts, "import");
		this.meta = meta;
	}

	public meta: ImportBindingMeta;
}

export class ArgumentsBinding extends Binding {
	constructor(opts: BindingOpts) {
		super(opts, "arguments");
	}
}

export class FunctionBinding extends Binding {
	constructor(opts: BindingOpts) {
		super(opts, "function");
	}
}

export type TypeBindingKind =
	| "function"
	| "class"
	| "interface"
	| "alias"
	| "enum"
	| "parameter"
	| "mapped type";

export class TypeBinding extends ConstBinding {
	constructor(
		opts: BindingOpts,
		valueNode: undefined | AnyNode,
		kind: TypeBindingKind,
	) {
		super(opts, valueNode, "type");
		this.typeKind = kind;
	}

	public typeKind: TypeBindingKind;
}

export class ClassBinding extends Binding {
	constructor(opts: BindingOpts) {
		super(opts, "class");
	}
}
