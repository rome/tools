/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBlockStatement,
	JSClassMethodKind,
	JSClassPropertyMeta,
	JSFunctionHead,
	JSPrivateName,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSClassPrivateMethod extends NodeBaseWithComments {
	readonly type: "JSClassPrivateMethod";
	readonly kind: JSClassMethodKind;
	readonly key: JSPrivateName;
	readonly head: JSFunctionHead;
	readonly body: JSBlockStatement;
	readonly meta: JSClassPropertyMeta;
}

export const jsClassPrivateMethod = createBuilder<JSClassPrivateMethod>(
	"JSClassPrivateMethod",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			meta: true,
			head: true,
			body: true,
		},
	},
);
