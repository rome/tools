/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyStatement, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSModuleBlock = JSNodeBase & {
	type: 'TSModuleBlock';
	body: Array<AnyStatement>;
};

export const tsModuleBlock = createBuilder<TSModuleBlock>(
	'TSModuleBlock',
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
