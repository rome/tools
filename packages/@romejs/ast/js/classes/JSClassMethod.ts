/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBlockStatement,
	JSClassPropertyMeta,
	JSFunctionHead,
	JSNodeBase,
} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {AnyJSObjectPropertyKey} from "../unions";

export type JSClassMethod = JSNodeBase & {
	type: "JSClassMethod";
	meta: JSClassPropertyMeta;
	key: AnyJSObjectPropertyKey;
	kind: JSClassMethodKind;
	head: JSFunctionHead;
	body: JSBlockStatement;
};

export type JSClassMethodKind = "constructor" | "method" | "get" | "set";

export const jsClassMethod = createBuilder<JSClassMethod>(
	"JSClassMethod",
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
