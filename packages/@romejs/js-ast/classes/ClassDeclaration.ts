/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BindingIdentifier, ClassHead, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type ClassDeclaration = JSNodeBase & {
	type: 'ClassDeclaration';
	id: BindingIdentifier;
	meta: ClassHead;
	abstract?: boolean;
	declare?: boolean;
};

export const classDeclaration = createBuilder<ClassDeclaration>(
	'ClassDeclaration',
	{
		bindingKeys: {
			id: true,
		},
		visitorKeys: {
			id: true,
			meta: true,
		},
	},
);
