/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {printMethod} from "../utils";
import {JSObjectMethod} from "@romefrontend/ast";

export default function JSObjectMethod(
	builder: Builder,
	node: JSObjectMethod,
): Token {
	return printMethod(builder, node);
}
