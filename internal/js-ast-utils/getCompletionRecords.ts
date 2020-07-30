/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSStatement,
	AnyNode,
	JSBreakStatement,
	JSContinueStatement,
	JSReturnStatement,
	JSThrowStatement,
} from "@internal/ast";

type CompletionRecord = {
	type: "COMPLETION";
	node:
		| JSReturnStatement
		| JSContinueStatement
		| JSBreakStatement
		| JSThrowStatement;
};

type InvalidRecord = {
	type: "INVALID";
	description: string;
	node: AnyNode;
};

type Records = Array<CompletionRecord | InvalidRecord>;

function getIfCompletionRecords(
	node: undefined | AnyNode,
	parent: AnyNode,
	key: string,
): Records {
	if (node === undefined) {
		return [
			{
				type: "INVALID",
				description: `empty ${key}`,
				node: parent,
			},
		];
	} else {
		return getCompletionRecords(node);
	}
}

function getLastCompletionRecordFromNodes(
	nodes: Array<AnyJSStatement>,
): undefined | Records {
	// Get the last node to produce records
	for (let i = nodes.length - 1; i >= 0; i--) {
		const node = nodes[i];
		const records = _getCompletionRecords(node);
		if (records !== undefined) {
			return records;
		}
	}
	return undefined;
}

function _getCompletionRecords(node: AnyNode): undefined | Records {
	if (node.type === "JSBlockStatement") {
		const records = getLastCompletionRecordFromNodes(node.body);
		if (records !== undefined) {
			return records;
		}

		return [
			{
				type: "INVALID",
				description: "empty block",
				node,
			},
		];
	}

	if (node.type === "JSSwitchStatement") {
		for (const caseNode of node.cases) {
			if (caseNode.test === undefined) {
				const records = getLastCompletionRecordFromNodes(caseNode.consequent);
				if (records === undefined) {
					return [
						{
							type: "INVALID",
							description: "default switch clause with no completions",
							node: caseNode,
						},
					];
				} else {
					return records;
				}
			}
		}

		return [
			{
				type: "INVALID",
				description: "switch with no default clause",
				node,
			},
		];
	}

	if (node.type === "JSIfStatement") {
		return [
			...getIfCompletionRecords(node.consequent, node, "consequent"),
			...getIfCompletionRecords(node.alternate, node, "alternate"),
		];
	}

	if (
		node.type === "JSReturnStatement" ||
		node.type === "JSContinueStatement" ||
		node.type === "JSBreakStatement" ||
		node.type === "JSThrowStatement"
	) {
		return [
			{
				type: "COMPLETION",
				node,
			},
		];
	}

	return undefined;
}

export function getCompletionRecords(node: AnyNode): Records {
	const records = _getCompletionRecords(node);
	if (records === undefined) {
		return [
			{
				type: "INVALID",
				description: "invalid node",
				node,
			},
		];
	} else {
		return records;
	}
}
