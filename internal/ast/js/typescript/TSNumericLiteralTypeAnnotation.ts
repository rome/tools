/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNumericLiteral, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSNumericLiteralTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSNumericLiteralTypeAnnotation";
	readonly value: number;
	readonly format?: JSNumericLiteral["format"];
}

export const tsNumericLiteralTypeAnnotation = createBuilder<TSNumericLiteralTypeAnnotation>(
	"TSNumericLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
