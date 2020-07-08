/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {TSTemplateLiteralTypeAnnotation} from "@romefrontend/ast";
import {escapeJSString} from "@romefrontend/string-escape";

export default function TSTemplateLiteralTypeAnnotation(
	builder: Builder,
	node: TSTemplateLiteralTypeAnnotation,
): Token {
	return escapeJSString(node.value, {quote: "`"});
}
