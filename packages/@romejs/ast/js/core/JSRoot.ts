/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSStatement,
	ConstJSProgramSyntax,
	ConstJSSourceType,
	JSDirective,
	JSInterpreterDirective,
	NodeBaseWithComments,
	RootBase,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSRoot = NodeBaseWithComments &
	RootBase & {
		type: "JSRoot";
		directives: Array<JSDirective>;
		body: Array<AnyJSStatement>;
		interpreter: undefined | JSInterpreterDirective;
		sourceType: ConstJSSourceType;
		syntax: Array<ConstJSProgramSyntax>;
		hasHoistedVars: boolean;
	};

export const MOCK_PROGRAM: JSRoot = {
	type: "JSRoot",
	directives: [],
	body: [],
	filename: "unknown",
	mtime: undefined,
	interpreter: undefined,
	corrupt: false,
	sourceType: "module",
	diagnostics: [],
	comments: [],
	syntax: [],
	hasHoistedVars: false,
};

export const jsRoot = createBuilder<JSRoot>(
	"JSRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			interpreter: true,
			directives: true,
			body: true,
			comments: true,
		},
	},
);
