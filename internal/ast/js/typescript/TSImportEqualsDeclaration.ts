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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSImportEqualsDeclaration extends NodeBaseWithComments {
	readonly type: "TSImportEqualsDeclaration";
	readonly id: JSBindingIdentifier;
	readonly moduleReference: AnyTSModuleReference;
	readonly isExport?: boolean;
}

export const tsImportEqualsDeclaration = createBuilder<TSImportEqualsDeclaration>(
	"TSImportEqualsDeclaration",
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {id: true, moduleReference: true},
	},
);
