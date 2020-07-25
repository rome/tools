/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, MOCK_PROGRAM, NodeBaseWithComments} from "@romefrontend/ast";
import {createDefaultProjectConfig} from "@romefrontend/project";
import {CompilerContext, Path, TransformVisitors} from "@romefrontend/compiler";
import {SourceLocation} from "@romefrontend/parser-core";

function removeProp<T extends {
	loc?: SourceLocation;
}>(obj: T): Omit<T, "loc"> {
	const {loc, ...locless} = obj;
	loc;
	return locless;
}

const removeLocTransform: TransformVisitors = [
	{
		name: "removeLocTransform",
		enter(path: Path) {
			const {node} = path;
			if (node.loc === undefined) {
				return node;
			} else {
				const newNode: NodeBaseWithComments = removeProp(node);

				// Also remove any `undefined` properties
				// rome-ignore lint/js/noExplicitAny
				const escaped: any = newNode;
				for (const key in newNode) {
					if (escaped[key] === undefined) {
						// rome-ignore lint/js/noDelete
						delete escaped[key];
					}
				}

				return (newNode as AnyNode);
			}
		},
	},
];

export function removeLoc(ast: AnyNode) {
	const context = new CompilerContext({
		ast: MOCK_PROGRAM,
		project: {
			directory: undefined,
			config: createDefaultProjectConfig(),
		},
	});
	return context.reduce(ast, removeLocTransform);
}
