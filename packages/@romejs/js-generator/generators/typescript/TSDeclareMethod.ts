/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSDeclareMethod, tsDeclareMethod, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens} from '../../tokens';
import {printMethod} from '../utils';

export default function TSDeclareMethod(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsDeclareMethod.assert(node);

  return [
    ...generator.print(node.meta, node),
    ...generator.print(node.key, node),
    ...printMethod(generator, node),
  ];
}
