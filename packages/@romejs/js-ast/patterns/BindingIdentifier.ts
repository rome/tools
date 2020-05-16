/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, PatternMeta} from '../index';
import {createQuickBuilder} from '../utils';

export type BindingIdentifier = JSNodeBase & {
	type: 'BindingIdentifier';
	name: string;
	definite?: boolean;
	meta?: PatternMeta;
};

export const bindingIdentifier = createQuickBuilder<BindingIdentifier, 'name'>(
	'BindingIdentifier',
	'name',
	{
		bindingKeys: {},
		visitorKeys: {
			meta: true,
		},
	},
);
