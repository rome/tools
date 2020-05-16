/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyObjectPropertyKey, AnyPrimaryType, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type TSPropertySignature = JSNodeBase & {
	type: 'TSPropertySignature';
	key: AnyObjectPropertyKey;
	optional?: boolean;
	readonly?: boolean;
	typeAnnotation?: AnyPrimaryType;
};

export const tsPropertySignature = createBuilder<TSPropertySignature>(
	'TSPropertySignature',
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			typeAnnotation: true,
		},
	},
);
