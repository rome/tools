/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type TSStringLiteralTypeAnnotation = JSNodeBase & {
	type: "TSStringLiteralTypeAnnotation";
	value: string;
};

export const tsStringLiteralTypeAnnotation = createBuilder<TSStringLiteralTypeAnnotation>(
	"TSStringLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
