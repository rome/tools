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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface TSDeclareMethod extends NodeBaseWithComments {
	type: "TSDeclareMethod";
	meta: JSClassPropertyMeta;
	kind: JSClassMethodKind;
	key: AnyJSObjectPropertyKey;
	head: JSFunctionHead;
	body?: void;
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
