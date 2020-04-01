/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSOptionalType, tsOptionalType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator} from '../../tokens';

export default function TSOptionalType(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = tsOptionalType.assert(node);

  return [...generator.print(node.typeAnnotation, node), operator('?')];
}
