/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CommentBlock extends NodeBaseWithComments {
	readonly type: "CommentBlock";
	readonly value: string;
	readonly id: string;
}

export const jsCommentBlock = createBuilder<CommentBlock>(
	"CommentBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
