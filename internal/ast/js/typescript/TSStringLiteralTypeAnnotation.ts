/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSStringLiteralTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSStringLiteralTypeAnnotation";
	readonly value: string;
}

export const tsStringLiteralTypeAnnotation = createBuilder<TSStringLiteralTypeAnnotation>(
	"TSStringLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
