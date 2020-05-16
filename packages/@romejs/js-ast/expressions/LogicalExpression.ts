/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type LogicalExpression = JSNodeBase & {
	type: 'LogicalExpression';
	operator: LogicalOperator;
	left: AnyExpression;
	right: AnyExpression;
};

export type LogicalOperator = '||' | '&&' | '??';

export const logicalExpression = createBuilder<LogicalExpression>(
	'LogicalExpression',
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
