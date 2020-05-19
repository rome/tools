/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSNumericLiteral} from "@romejs/ast";
import {humanizeNumber} from "@romejs/string-utils";

export default function JSNumericLiteral(
	builder: Builder,
	node: JSNumericLiteral,
): Token {
	if (builder.options.format === "pretty") {
		if (node.format === undefined) {
			return humanizeNumber(node.value);
		} else {
			switch (node.format) {
				case "binary":
					return `0b${node.value.toString(2)}`;
				case "octal":
					return `0o${node.value.toString(8)}`;
				case "hex":
					return `0x${node.value.toString(16)}`;
			}
		}
	} else {
		return String(node.value);
	}
}
