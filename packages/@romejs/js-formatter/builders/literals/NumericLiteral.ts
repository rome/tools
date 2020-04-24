/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {AnyNode, NumericLiteral, numericLiteral} from '@romejs/js-ast';
import {humanizeNumber} from '@romejs/string-utils';
import {number} from '@romejs/js-formatter/tokens';

export default function NumericLiteral(builder: Builder, node: AnyNode): Tokens {
  node = numericLiteral.assert(node);

  if (builder.options.format === 'pretty') {
    return [number(humanizeNumber(node.value))];
  } else {
    return [number(String(node.value))];
  }
}
