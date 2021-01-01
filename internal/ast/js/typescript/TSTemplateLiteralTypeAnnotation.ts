/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments, TSTemplateElement, AnyTSPrimary} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTemplateLiteralTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSTemplateLiteralTypeAnnotation";
	readonly quasis: TSTemplateElement[];
	readonly expressions: AnyTSPrimary[];
}

export const tsTemplateLiteralTypeAnnotation = createBuilder<TSTemplateLiteralTypeAnnotation>(
	"TSTemplateLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			quasis: true,
			expressions: true,
		},
	},
);
