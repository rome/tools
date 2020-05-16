/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type Node<Value> = {
	value: Value;
	lines: Array<Node<Value>>;
};

export default class Graph<Value> {
	constructor() {
		this.nodes = [];
		this.nodesByValue = new Map();
	}

	nodes: Array<Node<Value>>;
	nodesByValue: Map<Value, Node<Value>>;

	addNode(value: Value): void {
		if (this.find(value)) {
			return;
		}

		const node: Node<Value> = {lines: [], value};
		this.nodesByValue.set(value, node);
		this.nodes.push(node);
	}

	find(value: Value): undefined | Node<Value> {
		return this.nodesByValue.get(value);
	}

	hasConnections(value: Value) {
		const node = this.nodesByValue.get(value);
		return node !== undefined && (node?.lines).length > 0;
	}

	addLine(startValue: Value, endValue: Value) {
		const startNode = this.find(startValue);
		const endNode = this.find(endValue);

		if (!startNode || !endNode) {
			throw new Error('Both nodes need to exist');
		}

		startNode.lines.push(endNode);
	}
}
