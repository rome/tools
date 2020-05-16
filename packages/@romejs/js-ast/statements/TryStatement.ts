/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BlockStatement, CatchClause, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TryStatement = JSNodeBase & {
	type: 'TryStatement';
	block: BlockStatement;
	handler: undefined | CatchClause;
	finalizer: undefined | BlockStatement;
};

export const tryStatement = createBuilder<TryStatement>(
	'TryStatement',
	{
		bindingKeys: {},
		visitorKeys: {
			block: true,
			handler: true,
			finalizer: true,
		},
	},
);
