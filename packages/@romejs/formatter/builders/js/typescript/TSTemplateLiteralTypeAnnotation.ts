/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {TSTemplateLiteralTypeAnnotation} from "@romejs/ast";
import {escapeString} from "@romejs/string-escape";

export default function TSTemplateLiteralTypeAnnotation(
	builder: Builder,
	node: TSTemplateLiteralTypeAnnotation,
): Token {
	return escapeString(node.value, {quote: "`"});
}
