/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {TSNumericLiteralTypeAnnotation} from "@romejs/js-ast";

export default function TSNumericLiteralTypeAnnotation(
	builder: Builder,
	node: TSNumericLiteralTypeAnnotation,
): Token {
	return String(node.value);
}
