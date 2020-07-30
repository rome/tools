/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSPrivateName extends NodeBaseWithComments {
	readonly type: "JSPrivateName";
	readonly id: JSIdentifier;
}

export const jsPrivateName = createBuilder<JSPrivateName>(
	"JSPrivateName",
	{
		bindingKeys: {},
		visitorKeys: {
			id: true,
		},
	},
);
