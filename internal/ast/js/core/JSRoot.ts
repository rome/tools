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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSRoot extends NodeBaseWithComments,
RootBase {
	readonly type: "JSRoot";
	readonly directives: Array<JSDirective>;
	readonly body: Array<AnyJSStatement>;
	readonly interpreter: undefined | JSInterpreterDirective;
	readonly sourceType: ConstJSSourceType;
	readonly syntax: Array<ConstJSProgramSyntax>;
	readonly hasHoistedVars: boolean;
}

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
