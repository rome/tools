/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  OptionalCallExpression,
  optionalCallExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function OptionalCallExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = optionalCallExpression.assert(node);

  throw new Error('unimplemented');
}
