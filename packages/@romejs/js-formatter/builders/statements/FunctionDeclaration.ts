/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token} from '../../tokens';
import {FunctionDeclaration} from '@romejs/js-ast';
import FunctionExpression from '../expressions/FunctionExpression';

export default function FunctionDeclaration(
  builder: Builder,
  node: FunctionDeclaration,
): Token {
  return FunctionExpression(builder, node);
}
