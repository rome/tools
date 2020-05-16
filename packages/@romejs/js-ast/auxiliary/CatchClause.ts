/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyBindingPattern, BlockStatement, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type CatchClause = JSNodeBase & {
	type: 'CatchClause';
	param?: AnyBindingPattern;
	body: BlockStatement;
};

export const catchClause = createBuilder<CatchClause>(
	'CatchClause',
	{
		bindingKeys: {
			param: true,
		},
		visitorKeys: {
			param: true,
			body: true,
		},
	},
);
