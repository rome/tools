/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSTypeParameterDeclaration,
  TSTypeParameterInstantiation,
} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, group, indent, softline} from '../../tokens';
import {printCommaList} from '../utils';

export default function TSTypeParameterDeclaration(
  builder: Builder,
  node: TSTypeParameterDeclaration | TSTypeParameterInstantiation,
): Token {
  const params = node.params;
  const shouldInline =
    params.length === 1 &&
    params[0].type !== 'IntersectionTypeAnnotation' &&
    params[0].type !== 'UnionTypeAnnotation';

  if (shouldInline) {
    return concat(['<', builder.tokenize(params[0], node), '>']);
  } else {
    return group(
      concat([
        '<',
        indent(concat([softline, printCommaList(builder, params, node)])),
        softline,
        '>',
      ]),
    );
  }
}
