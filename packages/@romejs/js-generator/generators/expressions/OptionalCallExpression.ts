/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  OptionalCallExpression,
  optionalCallExpression,
  AnyNode,
} from '@romejs/js-ast';
import CallExpression from './CallExpression';

export default function OptionalCallExpression(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = optionalCallExpression.assert(node);
  return CallExpression(generator, node);
}
