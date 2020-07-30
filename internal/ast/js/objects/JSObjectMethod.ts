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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type JSObjectMethodKind = "get" | "set" | "method";

export interface JSObjectMethod extends NodeBaseWithComments {
	readonly key: JSComputedPropertyKey | JSStaticPropertyKey;
	readonly type: "JSObjectMethod";
	readonly kind: JSObjectMethodKind;
	readonly head: JSFunctionHead;
	readonly body: JSBlockStatement;
}

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
