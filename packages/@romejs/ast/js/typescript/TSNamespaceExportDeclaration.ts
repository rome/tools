/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSIdentifier, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type TSNamespaceExportDeclaration = JSNodeBase & {
	type: "TSNamespaceExportDeclaration";
	id: JSIdentifier;
};

export const tsNamespaceExportDeclaration = createBuilder<TSNamespaceExportDeclaration>(
	"TSNamespaceExportDeclaration",
	{bindingKeys: {}, visitorKeys: {id: true}},
);
