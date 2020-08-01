/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	AnyNodes,
	MOCK_PROGRAM,
	NodeBaseWithComments,
} from "@internal/ast";
import {createDefaultProjectConfig} from "@internal/project";
import {AnyVisitors, CompilerContext, Path, signals} from "@internal/compiler";
import {SourceLocation} from "@internal/parser-core";

function removeProp<T extends {
	loc?: SourceLocation;
}>(obj: T): Omit<T, "loc"> {
	const {loc, ...locless} = obj;
	loc;
	return locless;
}

const removeLocTransform: AnyVisitors = [
	{
		name: "removeLocTransform",
		enter(path: Path) {
			const {node} = path;

			if (node.loc === undefined) {
				return signals.retain;
			} else {
				const newNode: NodeBaseWithComments = removeProp(node);

				// Also remove any `undefined` properties
				// rome-ignore lint/ts/noExplicitAny
				const escaped: any = newNode;
				for (const key in newNode) {
					if (escaped[key] === undefined) {
						// rome-ignore lint/js/noDelete
						delete escaped[key];
					}
				}

				return signals.replace((newNode as AnyNode));
			}
		},
	},
];

export function removeLoc(ast: AnyNode): AnyNodes {
	const context = new CompilerContext({
		ast: MOCK_PROGRAM,
		project: {
			directory: undefined,
			config: createDefaultProjectConfig(),
		},
	});
	return context.reduce(ast, removeLocTransform);
}
