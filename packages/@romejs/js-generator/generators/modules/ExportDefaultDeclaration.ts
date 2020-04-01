/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space} from '../../tokens';
import {
  ExportDefaultDeclaration,
  exportDefaultDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import {_ExportDeclaration} from './ExportLocalDeclaration';

export default function ExportDefaultDeclaration(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = exportDefaultDeclaration.assert(node);

  return [
    word('export'),
    word('default'),
    space,
    ..._ExportDeclaration(generator, node),
  ];
}
