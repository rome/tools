/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, indent} from "@romejs/formatter";

import {JSXFragment} from "@romejs/ast";

export default function JSXFragment(builder: Builder, node: JSXFragment): Token {
	return concat([
		"<>",
		indent(concat(node.children.map((child) => builder.tokenize(child, node)))),
		"</>",
	]);
}
