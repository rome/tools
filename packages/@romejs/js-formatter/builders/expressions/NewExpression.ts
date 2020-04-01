/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word} from '../../tokens';
import {NewExpression, newExpression, AnyNode} from '@romejs/js-ast';
import CallExpression from './CallExpression';

export default function NewExpression(builder: Builder, node: AnyNode): Tokens {
  node = newExpression.assert(node);

  return [word('new'), ...CallExpression(builder, node)];
}
