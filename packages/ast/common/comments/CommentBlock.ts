/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface CommentBlock extends NodeBaseWithComments {
	type: "CommentBlock";
	value: string;
	id: string;
}

export const jsCommentBlock = createBuilder<CommentBlock>(
	"CommentBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
