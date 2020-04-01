/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {
  FlowNullableTypeAnnotation,
  flowNullableTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowNullableTypeAnnotation(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowNullableTypeAnnotation.assert(node);

  return [operator('?'), ...generator.print(node.typeAnnotation, node)];
}
