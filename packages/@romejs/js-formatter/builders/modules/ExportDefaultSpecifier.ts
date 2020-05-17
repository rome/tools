/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {ExportDefaultSpecifier} from "@romejs/js-ast";

export default function ExportDefaultSpecifier(
	builder: Builder,
	node: ExportDefaultSpecifier,
): Token {
	return builder.tokenize(node.exported, node);
}
