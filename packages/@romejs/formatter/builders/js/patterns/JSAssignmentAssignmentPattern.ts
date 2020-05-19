/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSAssignmentAssignmentPattern} from "@romejs/ast";
import {Builder, Token} from "@romejs/formatter";

import JSBindingAssignmentPattern from "./JSBindingAssignmentPattern";

export default function JSAssignmentAssignmentPattern(
	builder: Builder,
	node: JSAssignmentAssignmentPattern,
): Token {
	return JSBindingAssignmentPattern(builder, node);
}
