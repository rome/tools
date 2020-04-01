/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space, word} from '../../tokens';
import {
  ImportNamespaceSpecifier,
  importNamespaceSpecifier,
  AnyNode,
} from '@romejs/js-ast';

export default function ImportNamespaceSpecifier(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = importNamespaceSpecifier.assert(node);

  return [
    operator('*'),
    space,
    word('as'),
    space,
    ...generator.print(node.local.name, node),
  ];
}
