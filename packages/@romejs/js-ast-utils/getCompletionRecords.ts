/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	AnyStatement,
	BreakStatement,
	ContinueStatement,
	ReturnStatement,
	ThrowStatement,
} from '@romejs/js-ast';

type CompletionRecord = {
	type: 'COMPLETION';
	node: ReturnStatement | ContinueStatement | BreakStatement | ThrowStatement;
};

type InvalidRecord = {
	type: 'INVALID';
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
				type: 'INVALID',
				description: `empty ${key}`,
				node: parent,
			},
		];
	} else {
		return getCompletionRecords(node);
	}
}

function getLastCompletionRecordFromNodes(
	nodes: Array<AnyStatement>,
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
	if (node.type === 'BlockStatement') {
		const records = getLastCompletionRecordFromNodes(node.body);
		if (records !== undefined) {
			return records;
		}

		return [
			{
				type: 'INVALID',
				description: 'empty block',
				node,
			},
		];
	}

	if (node.type === 'SwitchStatement') {
		for (const caseNode of node.cases) {
			if (caseNode.test === undefined) {
				const records = getLastCompletionRecordFromNodes(caseNode.consequent);
				if (records === undefined) {
					return [
						{
							type: 'INVALID',
							description: 'default switch clause with no completions',
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
				type: 'INVALID',
				description: 'switch with no default clause',
				node,
			},
		];
	}

	if (node.type === 'IfStatement') {
		return [
			...getIfCompletionRecords(node.consequent, node, 'consequent'),
			...getIfCompletionRecords(node.alternate, node, 'alternate'),
		];
	}

	if (
		node.type === 'ReturnStatement' ||
		node.type === 'ContinueStatement' ||
		node.type === 'BreakStatement' ||
		node.type === 'ThrowStatement'
	) {
		return [
			{
				type: 'COMPLETION',
				node,
			},
		];
	}

	return undefined;
}

export default function getCompletionRecords(node: AnyNode): Records {
	const records = _getCompletionRecords(node);
	if (records === undefined) {
		return [
			{
				type: 'INVALID',
				description: 'invalid node',
				node,
			},
		];
	} else {
		return records;
	}
}
