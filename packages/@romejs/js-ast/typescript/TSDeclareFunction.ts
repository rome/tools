/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BindingIdentifier, JSNodeBase} from '../index';
import {createBuilder} from '../utils';
import {FunctionHead} from '../auxiliary/FunctionHead';

export type TSDeclareFunction = JSNodeBase & {
	type: 'TSDeclareFunction';
	id: BindingIdentifier;
	head: FunctionHead;

	// For consistency with FunctionDeclaration, this can mostly be ignored
	declare?: boolean;
};

export const tsDeclareFunction = createBuilder<TSDeclareFunction>(
	'TSDeclareFunction',
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			head: true,
		},
	},
);
