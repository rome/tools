/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  TSModuleDeclaration,
  tsModuleDeclaration,
  AnyNode,
  TSModuleBlock,
} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator, space, word} from '../../tokens';

export default function TSModuleDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsModuleDeclaration.assert(node);

  let tokens: Tokens = [];

  if (node.declare) {
    tokens.push(word('declare'));
    tokens.push(space);
  }

  if (!node.global) {
    tokens.push(
      word(node.id.type === 'BindingIdentifier' ? 'namespace' : 'module'),
    );
    tokens.push(space);
  }

  tokens = [...tokens, ...generator.print(node.id, node)];

  if (!node.body) {
    operator(';');
    return tokens;
  }

  let body: undefined | TSModuleBlock | TSModuleDeclaration = node.body;
  while (body !== undefined && body.type === 'TSModuleDeclaration') {
    tokens = [...tokens, operator('.'), ...generator.print(body.id, body)];
    body = body.body;
  }

  return [...tokens, space, ...generator.print(body, node)];
}
