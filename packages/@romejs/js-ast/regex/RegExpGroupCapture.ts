/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRegExpExpression, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type RegExpGroupCapture = JSNodeBase & {
	type: 'RegExpGroupCapture';
	expression: AnyRegExpExpression;
	name?: string;
};

export const regExpGroupCapture = createBuilder<RegExpGroupCapture>(
	'RegExpGroupCapture',
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
