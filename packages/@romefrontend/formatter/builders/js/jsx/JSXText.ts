/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";
import {JSXText} from "@romefrontend/ast";
import {escapeXHTMLEntities} from "@romefrontend/html-parser";

export default function JSXText(builder: Builder, node: JSXText): Token {
	return escapeXHTMLEntities(node.value);
}
