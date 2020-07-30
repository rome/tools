/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Token, concat} from "@internal/formatter";

export default function JSDebuggerStatement(): Token {
	return concat(["debugger", ";"]);
}
