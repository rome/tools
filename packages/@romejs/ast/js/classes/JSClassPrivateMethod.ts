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
	JSNodeBase,
	JSPrivateName,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSClassPrivateMethod = JSNodeBase & {
	type: "JSClassPrivateMethod";
	kind: JSClassMethodKind;
	key: JSPrivateName;
	head: JSFunctionHead;
	body: JSBlockStatement;
	meta: JSClassPropertyMeta;
};

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
