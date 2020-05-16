/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, JSNodeBase} from '../index';
import {createQuickBuilder} from '../utils';

export type ComputedMemberProperty = JSNodeBase & {
	type: 'ComputedMemberProperty';
	value: AnyExpression;
	optional?: boolean;
};

export const computedMemberProperty = createQuickBuilder<
	ComputedMemberProperty,
	'value'
>(
	'ComputedMemberProperty',
	'value',
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
