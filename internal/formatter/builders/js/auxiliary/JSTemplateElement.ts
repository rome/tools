/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";
import {AnyNode, JSTemplateElement, jsTemplateLiteral} from "@internal/ast";

export default function JSTemplateElement(
	builder: Builder,
	node: JSTemplateElement,
	parent: AnyNode,
): Token {
	parent = jsTemplateLiteral.assert(parent);

	const isFirst = parent.quasis[0] === node;
	const isLast = parent.quasis[parent.quasis.length - 1] === node;

	const value = (isFirst ? "`" : "}") + node.raw + (isLast ? "`" : "${");
	return value;
}
