/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeParameterInstantiation} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token} from '../../tokens';
import TSTypeParameterDeclaration from './TSTypeParameterDeclaration';

export default function TSTypeParameterInstantiation(
	builder: Builder,
	node: TSTypeParameterInstantiation,
): Token {
	return TSTypeParameterDeclaration(builder, node);
}
