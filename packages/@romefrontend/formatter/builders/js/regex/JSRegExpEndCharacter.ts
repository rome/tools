/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Token} from "@romefrontend/formatter";
import {JSRegExpEndCharacter} from "@romefrontend/ast";

export default function JSRegExpEndCharacter(): Token {
	return "$";
}
