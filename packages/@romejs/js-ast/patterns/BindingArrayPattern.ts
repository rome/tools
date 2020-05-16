/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {
	AnyParamBindingPattern,
	AnyTargetBindingPattern,
	ArrayHole,
	JSNodeBase,
	PatternMeta,
} from '../index';
import {createBuilder} from '../utils';

export type BindingArrayPattern = JSNodeBase & {
	type: 'BindingArrayPattern';
	meta?: PatternMeta;
	elements: Array<ArrayHole | AnyParamBindingPattern>;
	rest: undefined | AnyTargetBindingPattern;
};

export const bindingArrayPattern = createBuilder<BindingArrayPattern>(
	'BindingArrayPattern',
	{
		bindingKeys: {
			elements: true,
			rest: true,
		},
		visitorKeys: {
			elements: true,
			rest: true,
			meta: true,
		},
	},
);
