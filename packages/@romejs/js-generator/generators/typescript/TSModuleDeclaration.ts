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

export default function TSModuleDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node = tsModuleDeclaration.assert(node);

  if (node.declare) {
    generator.word('declare');
    generator.space();
  }

  if (!node.global) {
    generator.word(
      node.id.type === 'BindingIdentifier' ? 'namespace' : 'module',
    );
    generator.space();
  }
  generator.print(node.id, node);

  if (!node.body) {
    generator.token(';');
    return;
  }

  let body: undefined | TSModuleBlock | TSModuleDeclaration = node.body;
  while (body !== undefined && body.type === 'TSModuleDeclaration') {
    generator.token('.');
    generator.print(body.id, body);
    body = body.body;
  }

  generator.space();
  generator.print(body, node);
}
