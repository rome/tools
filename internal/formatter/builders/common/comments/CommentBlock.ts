/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Token} from "@internal/formatter";

// Printing of comments is handled in internal/formatter/builders/comments.ts
export default function CommentBlock(): Token {
	throw new Error("Unexpected comment printing");
}
