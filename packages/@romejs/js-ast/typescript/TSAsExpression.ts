/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyTSPrimary, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSAsExpression = JSNodeBase & {
	type: 'TSAsExpression';
	typeAnnotation: AnyTSPrimary;
	expression: AnyExpression;
};

export const tsAsExpression = createBuilder<TSAsExpression>(
	'TSAsExpression',
	{
		bindingKeys: {},
		visitorKeys: {expression: true, typeAnnotation: true},
	},
);
