/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSInferType, tsInferType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';
import {Tokens, operator, space} from '../../tokens';

export default function TSInferType(generator: Generator, node: AnyNode): Tokens {
  node = tsInferType.assert(node);

  return [
    operator('infer'),
    space,
    ...generator.print(node.typeParameter, node),
  ];
}
