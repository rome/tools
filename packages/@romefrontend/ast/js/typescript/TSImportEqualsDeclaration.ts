/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSModuleReference,
	JSBindingIdentifier,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSImportEqualsDeclaration = NodeBaseWithComments & {
	type: "TSImportEqualsDeclaration";
	id: JSBindingIdentifier;
	moduleReference: AnyTSModuleReference;
	isExport?: boolean;
};

export const tsImportEqualsDeclaration = createBuilder<TSImportEqualsDeclaration>(
	"TSImportEqualsDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {id: true, moduleReference: true},
	},
);
