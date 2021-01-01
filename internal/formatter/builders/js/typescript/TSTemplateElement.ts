/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";
import {AnyTSPrimary, tsTemplateLiteralTypeAnnotation, TSTemplateElement} from "@internal/ast";

export default function TSTemplateElement(
	builder: Builder,
	node: TSTemplateElement,
	parent: AnyTSPrimary,
): Token {
	parent = tsTemplateLiteralTypeAnnotation.assert(parent);

	const isFirst = parent.quasis[0] === node;
	const isLast = parent.quasis[parent.quasis.length - 1] === node;

	const value = (isFirst ? "`" : "}") + node.raw + (isLast ? "`" : "${");
	return value;
}
