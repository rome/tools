/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";
import {JSXText} from "@internal/ast";
import {escapeXHTMLEntities} from "@internal/html-parser";
import {cleanJSXText} from "@internal/js-ast-utils";

export default function JSXText(builder: Builder, node: JSXText): Token {
	return escapeXHTMLEntities(cleanJSXText(node.value));
}
