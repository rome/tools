/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  FunctionDeclaration,
  functionDeclaration,
  AnyNode,
} from '@romejs/js-ast';
import FunctionExpression from '../expressions/FunctionExpression';

export default function FunctionDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = functionDeclaration.assert(node);
  return FunctionExpression(builder, node);
}
