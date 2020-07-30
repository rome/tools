/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface CommentLine extends NodeBaseWithComments {
	readonly type: "CommentLine";
	readonly value: string;
	readonly id: string;
}

export const jsCommentLine = createBuilder<CommentLine>(
	"CommentLine",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
