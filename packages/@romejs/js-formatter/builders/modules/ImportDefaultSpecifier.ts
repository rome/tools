/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {ImportDefaultSpecifier} from "@romejs/js-ast";

export default function ImportDefaultSpecifier(
	builder: Builder,
	node: ImportDefaultSpecifier,
): Token {
	return builder.tokenize(node.local.name, node);
}
