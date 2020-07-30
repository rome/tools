/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSClassPrivateMethod} from "@internal/ast";
import {printMethod} from "../utils";

export default function JSClassPrivateMethod(
	builder: Builder,
	node: JSClassPrivateMethod,
): Token {
	const printed = printMethod(builder, node);
	if (node.meta.static === true) {
		return concat(["static", space, printed]);
	} else {
		return printed;
	}
}
