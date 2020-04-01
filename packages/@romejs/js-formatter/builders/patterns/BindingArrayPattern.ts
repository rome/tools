/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  BindingArrayPattern,
  bindingArrayPattern,
  AnyNode,
} from '@romejs/js-ast';
import ArrayExpression from '../expressions/ArrayExpression';
import {printPatternMeta} from '../utils';

export default function BindingArrayPattern(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = bindingArrayPattern.assert(node);

  return [
    ...ArrayExpression(builder, node),
    ...printPatternMeta(builder, node, node.meta),
  ];
}
