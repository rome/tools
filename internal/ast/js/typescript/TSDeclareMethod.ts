/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSObjectPropertyKey,
	JSClassMethodKind,
	JSClassPropertyMeta,
	JSFunctionHead,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSDeclareMethod extends NodeBaseWithComments {
	readonly type: "TSDeclareMethod";
	readonly meta: JSClassPropertyMeta;
	readonly kind: JSClassMethodKind;
	readonly key: AnyJSObjectPropertyKey;
	readonly head: JSFunctionHead;
	readonly body?: void;
}

export const tsDeclareMethod = createBuilder<TSDeclareMethod>(
	"TSDeclareMethod",
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
			key: true,
			head: true,
		},
	},
);
