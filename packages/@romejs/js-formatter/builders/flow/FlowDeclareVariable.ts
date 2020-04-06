/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, operator} from '../../tokens';
import {
  FlowDeclareVariable,
  flowDeclareVariable,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareVariable(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowDeclareVariable.assert(node);

  let tokens: Tokens = [];

  if (parent.type !== 'ExportLocalDeclaration') {
    tokens.push(word('declare'));
    tokens.push(space);
  }

  tokens.push(word('var'));
  tokens.push(space);

  const {id} = node;

  tokens = tokens.concat(builder.tokenize(id, node));

  if (id.meta !== undefined) {
    tokens = tokens.concat(builder.tokenize(id.meta.typeAnnotation, node));
  }

  tokens.push(operator(';'));

  return tokens;
}
