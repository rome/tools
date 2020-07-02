/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstExportModuleKind,
	JSExportDefaultSpecifier,
	JSExportExternalSpecifier,
	JSExportNamespaceSpecifier,
	JSStringLiteral,
	NodeBaseWithComments,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type AnyExportExternalSpecifier =
	| JSExportNamespaceSpecifier
	| JSExportDefaultSpecifier
	| JSExportExternalSpecifier;

export type JSExportExternalDeclaration = NodeBaseWithComments & {
	type: "JSExportExternalDeclaration";
	defaultSpecifier?: JSExportDefaultSpecifier;
	namespaceSpecifier?: JSExportNamespaceSpecifier;
	namedSpecifiers: Array<JSExportExternalSpecifier>;
	source: JSStringLiteral;
	exportKind?: ConstExportModuleKind;
};

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
