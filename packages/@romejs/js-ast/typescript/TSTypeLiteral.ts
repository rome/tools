/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSTypeElement, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSTypeLiteral = JSNodeBase & {
	type: 'TSTypeLiteral';
	members: Array<AnyTSTypeElement>;
};

export const tsTypeLiteral = createBuilder<TSTypeLiteral>(
	'TSTypeLiteral',
	{
		bindingKeys: {},
		visitorKeys: {
			members: true,
		},
	},
);
