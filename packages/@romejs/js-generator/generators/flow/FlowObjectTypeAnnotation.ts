/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator} from '../../tokens';
import {
  FlowObjectTypeAnnotation,
  flowObjectTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowObjectTypeAnnotation(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowObjectTypeAnnotation.assert(node);

  return [
    operator(node.exact ? '{|' : '{'),
    generator.printCommaList(node.properties, node),
    operator(node.exact ? '|}' : '}'),
  ];
}
