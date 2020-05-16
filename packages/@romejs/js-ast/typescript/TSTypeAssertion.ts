/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyTSPrimary, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSTypeAssertion = JSNodeBase & {
	type: 'TSTypeAssertion';
	expression: AnyExpression;
	typeAnnotation: AnyTSPrimary;
};

export const tsTypeAssertion = createBuilder<TSTypeAssertion>(
	'TSTypeAssertion',
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
			typeAnnotation: true,
		},
	},
);
