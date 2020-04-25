/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AnyNode, FlowDeclareFunction} from '@romejs/js-ast';

export default function FlowDeclareFunction(
  builder: Builder,
  node: FlowDeclareFunction,
  parent: AnyNode,
): Token {
  const tokens: Array<Token> = [];
  if (parent.type !== 'ExportLocalDeclaration') {
    tokens.push('declare', space);
  }

  return concat([
    concat(tokens),
    'function',
    space,
    builder.tokenize(node.id, node),
    ';',
  ]);
}
