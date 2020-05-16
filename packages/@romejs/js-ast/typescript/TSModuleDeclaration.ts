/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BindingIdentifier,
	JSNodeBase,
	StringLiteral,
	TSModuleBlock,
} from '../index';
import {createBuilder} from '../utils';

export type TSModuleDeclaration = JSNodeBase & {
	type: 'TSModuleDeclaration';
	id: BindingIdentifier | StringLiteral;
	global?: boolean;
	body?: TSModuleBlock | TSModuleDeclaration;
	declare?: boolean;
};

export const tsModuleDeclaration = createBuilder<TSModuleDeclaration>(
	'TSModuleDeclaration',
	{
		bindingKeys: {},
		visitorKeys: {
			id: true,
			body: true,
		},
	},
);
