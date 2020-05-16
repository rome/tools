/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ImportSpecifierLocal, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type ImportNamespaceSpecifier = JSNodeBase & {
	type: 'ImportNamespaceSpecifier';
	local: ImportSpecifierLocal;
};

export const importNamespaceSpecifier = createBuilder<ImportNamespaceSpecifier>(
	'ImportNamespaceSpecifier',
	{
		bindingKeys: {
			local: true,
		},
		visitorKeys: {
			local: true,
		},
	},
);
