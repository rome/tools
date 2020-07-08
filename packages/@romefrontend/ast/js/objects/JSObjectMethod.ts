/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBlockStatement,
	JSComputedPropertyKey,
	JSFunctionHead,
	JSStaticPropertyKey,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSObjectMethodKind = "get" | "set" | "method";

export type JSObjectMethod = NodeBaseWithComments & {
	key: JSComputedPropertyKey | JSStaticPropertyKey;
	type: "JSObjectMethod";
	kind: JSObjectMethodKind;
	head: JSFunctionHead;
	body: JSBlockStatement;
};

export const jsObjectMethod = createBuilder<JSObjectMethod>(
	"JSObjectMethod",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			head: true,
			body: true,
		},
	},
);
