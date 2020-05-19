/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSExportExternalSpecifier} from "@romejs/ast";
import JSExportLocalSpecifier from "./JSExportLocalSpecifier";

export default function JSExportExternalSpecifier(
	builder: Builder,
	node: JSExportExternalSpecifier,
): Token {
	return JSExportLocalSpecifier(builder, node);
}
