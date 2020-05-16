/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, AnyPrimaryType, JSNodeBase, PrivateName} from '../index';
import {createBuilder} from '../utils';
import {ClassPropertyMeta} from './ClassPropertyMeta';

export type ClassPrivateProperty = JSNodeBase & {
	type: 'ClassPrivateProperty';
	key: PrivateName;
	meta: ClassPropertyMeta;
	value: undefined | AnyExpression;
	typeAnnotation?: AnyPrimaryType;
};

export const classPrivateProperty = createBuilder<ClassPrivateProperty>(
	'ClassPrivateProperty',
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			meta: true,
			value: true,
			typeAnnotation: true,
		},
	},
);
