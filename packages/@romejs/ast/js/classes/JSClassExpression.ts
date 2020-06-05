/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBindingIdentifier, JSClassHead, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSClassExpression = JSNodeBase & {
	type: "JSClassExpression";
	id?: JSBindingIdentifier;
	meta: JSClassHead;
};

export const jsClassExpression = createBuilder<JSClassExpression>(
	"JSClassExpression",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			meta: true,
		},
	},
);
