/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSExportModuleKind,
	JSExportDefaultSpecifier,
	JSExportExternalSpecifier,
	JSExportNamespaceSpecifier,
	JSStringLiteral,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type AnyExportExternalSpecifier =
	| JSExportNamespaceSpecifier
	| JSExportDefaultSpecifier
	| JSExportExternalSpecifier;

export interface JSExportExternalDeclaration extends NodeBaseWithComments {
	readonly type: "JSExportExternalDeclaration";
	readonly defaultSpecifier?: JSExportDefaultSpecifier;
	readonly namespaceSpecifier?: JSExportNamespaceSpecifier;
	readonly namedSpecifiers: Array<JSExportExternalSpecifier>;
	readonly source: JSStringLiteral;
	readonly exportKind?: ConstJSExportModuleKind;
}

export const jsExportExternalDeclaration = createBuilder<JSExportExternalDeclaration>(
	"JSExportExternalDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			defaultSpecifier: true,
			namespaceSpecifier: true,
			namedSpecifiers: true,
			source: true,
		},
	},
);
