/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {AnyNode, TemplateElement, templateLiteral} from "@romejs/js-ast";
import {Token} from "../../tokens";

export default function TemplateElement(
	builder: Builder,
	node: TemplateElement,
	parent: AnyNode,
): Token {
	parent = templateLiteral.assert(parent);

	const isFirst = parent.quasis[0] === node;
	const isLast = parent.quasis[parent.quasis.length - 1] === node;

	const value = (isFirst ? "`" : "}") + node.raw + (isLast ? "`" : "${");
	return value;
}
