/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, operator, word} from '../../tokens';
import {
  FlowDeclareFunction,
  flowDeclareFunction,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareFunction(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowDeclareFunction.assert(node);

  let tokens: Tokens = [];
  if (parent.type !== 'ExportLocalDeclaration') {
    tokens = [word('declare'), space];
  }

  return [
    ...tokens,
    word('function'),
    space,
    ...generator.print(node.id, node),
    operator(';'),
  ];
}
