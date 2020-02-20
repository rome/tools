/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  BindingArrayPattern,
  bindingArrayPattern,
  AnyNode,
} from '@romejs/js-ast';
import ArrayExpression from '../expressions/ArrayExpression';

export default function BindingArrayPattern(
  generator: Generator,
  node: AnyNode,
) {
  node = bindingArrayPattern.assert(node);

  ArrayExpression(generator, node);
}
