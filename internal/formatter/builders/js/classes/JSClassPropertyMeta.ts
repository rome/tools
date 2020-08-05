/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSClassPropertyMeta} from "@internal/ast";

export default function JSClassPropertyMeta(
	builder: Builder,
	node: JSClassPropertyMeta,
): Token {
	const tokens: Array<Token> = [];

	if (builder.options.typeAnnotations) {
		if (node.accessibility) {
			tokens.push(node.accessibility, space);
		}

		if (node.readonly) {
			tokens.push("readonly", space);
		}

		if (node.abstract) {
			tokens.push("abstract", space);
		}
	}

	if (node.static) {
		tokens.push("static", space);
	}

	return concat(tokens);
}
