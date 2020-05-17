/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExpressionNode} from "./parse";

export default function stringify(node: ExpressionNode): string {
	// TODO parens
	switch (node.type) {
		case "Or":
			return `${stringify(node.left)} OR ${stringify(node.right)}`;

		case "And":
			return `${stringify(node.left)} AND ${stringify(node.right)}`;

		case "License": {
			let str = node.id;
			if (node.plus) {
				str += "+";
			}
			if (node.exception !== undefined) {
				str += ` WITH ${node.exception}`;
			}
			return str;
		}
	}
}
