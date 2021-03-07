/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";
import {JSNumericLiteral} from "@internal/ast";
import {humanizeNumber} from "@internal/numbers";

export default function JSNumericLiteral(
	builder: Builder,
	node: JSNumericLiteral,
): Token {
	const {format, value} = node;

	switch (format) {
		case "binary":
			return `0b${value.toString(2)}`;

		case "octal":
			return `0o${value.toString(8)}`;

		case "hex":
			return `0x${value.toString(16)}`;

		case "scientific": {
			let str = value.toExponential();

			// The plus in the form of 5e+0 is redundant
			str = str.replace(/e\+/g, "e");

			return str;
		}
	}

	return humanizeNumber(value);
}
