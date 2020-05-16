/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyExpression,
	AnyPrimaryType,
	JSNodeBase,
	SpreadElement,
} from '../index';
import {createBuilder} from '../utils';

export type AmbiguousFlowTypeCastExpression = JSNodeBase & {
	type: 'AmbiguousFlowTypeCastExpression';
	expression: AnyExpression | SpreadElement;
	typeAnnotation?: AnyPrimaryType;

	// This is for js-parser so that we can convert type casts to parameters

	// We should figure out some way to remove this
	optional?: boolean;
};

export const ambiguousFlowTypeCastExpression = createBuilder<AmbiguousFlowTypeCastExpression>(
	'AmbiguousFlowTypeCastExpression',
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
			typeAnnotation: true,
		},
	},
);
