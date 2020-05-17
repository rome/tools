/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Token, concat, hardline} from "../../tokens";
import { Builder } from "@romejs/js-formatter";
import { InterpreterDirective } from "@romejs/js-ast";

export default function InterpreterDirective(
  builder: Builder,
  node: InterpreterDirective
): Token {
	return concat([`#!${node.value}`, hardline, hardline]);
}
