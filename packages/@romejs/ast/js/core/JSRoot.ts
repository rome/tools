/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSComment,
	AnyJSStatement,
	ConstProgramSyntax,
	ConstSourceType,
	JSDirective,
	JSInterpreterDirective,
	NodeBaseWithComments,
} from "@romejs/ast";
import {Diagnostics} from "@romejs/diagnostics";
import {createBuilder} from "../../utils";

export type JSRoot = NodeBaseWithComments & {
	type: "JSRoot";
	directives: Array<JSDirective>;
	body: Array<AnyJSStatement>;
	filename: string;
	interpreter: undefined | JSInterpreterDirective;
	mtime: undefined | number;
	corrupt: boolean;
	sourceType: ConstSourceType;
	diagnostics: Diagnostics;
	comments: Array<AnyJSComment>;
	syntax: Array<ConstProgramSyntax>;
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
