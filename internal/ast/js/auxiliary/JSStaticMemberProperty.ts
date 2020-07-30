/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, JSPrivateName, NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSStaticMemberProperty extends NodeBaseWithComments {
	readonly type: "JSStaticMemberProperty";
	readonly value: JSIdentifier | JSPrivateName;
	readonly optional?: boolean;
}

export const jsStaticMemberProperty = createQuickBuilder<
	JSStaticMemberProperty,
	"value"
>(
	"JSStaticMemberProperty",
	"value",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
