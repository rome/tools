/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, NumericLiteral} from "../index";
import {createBuilder} from "../utils";

export type TSNumericLiteralTypeAnnotation = JSNodeBase & {
	type: "TSNumericLiteralTypeAnnotation";
	value: number;
	format?: NumericLiteral["format"];
};

export const tsNumericLiteralTypeAnnotation = createBuilder<TSNumericLiteralTypeAnnotation>(
	"TSNumericLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
