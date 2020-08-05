/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {assertSingleNode, inheritLoc} from "@internal/js-ast-utils";
import {NodeBase} from "@internal/parser-core";
import {AnyNode, AnyNodes} from "./index";
import {NodeBaseWithComments} from "./base";

export const bindingKeys: Map<string, Array<string>> = new Map();
export const visitorKeys: Map<string, Array<string>> = new Map();
export const nodeNames: Set<string> = new Set();

type JustNodeKeysProp<K, V> = V extends
	| NodeBase
	| Array<NodeBase>
	| Array<undefined | NodeBase>
	? K
	: never;

type JustNodeKeys<T> = ExcludeCoreNodeKeys<
	{[K in keyof T]: JustNodeKeysProp<K, NonNullable<T[K]>>}[keyof T]
>;

type ExcludeCoreNodeKeys<T> = Exclude<T, keyof NodeBaseWithComments>;

type VisitorKeys<T> = {[K in JustNodeKeys<T>]: true};

type BindingKeys<T> = {[K in JustNodeKeys<T>]?: true};

interface CreateBuilderOptions<Node> {
	bindingKeys: BindingKeys<Node>;
	visitorKeys: VisitorKeys<Node>;
}

function declareBuilder<Node>(type: string, opts: CreateBuilderOptions<Node>) {
	nodeNames.add(type);

	if (opts.visitorKeys !== undefined) {
		visitorKeys.set(type, Object.keys(opts.visitorKeys));
	}

	if (opts.bindingKeys !== undefined) {
		bindingKeys.set(type, Object.keys(opts.bindingKeys));
	}
}

// TODO only allow this method to be called on a node with only one required property
export function createQuickBuilder<
	Node extends AnyNode,
	QuickKey extends keyof Node
>(
	type: Node["type"],
	quickKey: QuickKey,
	opts: CreateBuilderOptions<Node>,
): QuickBuilder<Node, Node[QuickKey]> {
	declareBuilder(type, opts);
	return new QuickBuilder(type, quickKey);
}

export function createBuilder<Node extends AnyNode>(
	type: string,
	opts: CreateBuilderOptions<Node>,
): Builder<Node> {
	declareBuilder(type, opts);
	return new Builder(type);
}

class Builder<Node extends AnyNode> {
	constructor(type: string) {
		this.type = type;
	}

	private type: string;

	public create(opts: Omit<Node, "type">, inheritNode?: AnyNode): Node {
		// @ts-ignore
		return Object.freeze({
			loc: inheritNode === undefined ? undefined : inheritLoc(inheritNode),
			...opts,
			type: this.type,
		});
	}

	public assert(res: undefined | AnyNodes): Node {
		if (res === undefined) {
			throw new Error(`Expected ${this.type} Node but got undefined`);
		}

		const node = assertSingleNode(res);

		if (node.type !== this.type) {
			throw new Error(`Expected ${this.type} Node but got ${node.type}`);
		}

		// @ts-ignore
		return node;
	}
}

class QuickBuilder<Node extends AnyNode, Arg> extends Builder<Node> {
	constructor(type: string, quickKey: keyof Node) {
		super(type);
		this.quickKey = quickKey;
	}

	private quickKey: keyof Node;

	public quick(
		arg: Arg,
		opts?: Partial<Omit<Node, "type">>,
		inheritNode?: Node,
	): Node {
		const node = ({
			...opts,
			[this.quickKey]: arg,
		} as Node);

		return this.create(node, inheritNode);
	}
}
