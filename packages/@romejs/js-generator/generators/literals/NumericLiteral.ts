/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {NumericLiteral, numericLiteral, AnyNode} from '@romejs/js-ast';
import {humanizeNumber} from '@romejs/string-utils';
import {number} from '@romejs/js-generator/tokens';

export default function NumericLiteral(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = numericLiteral.assert(node);

  if (generator.options.format === 'pretty') {
    return [number(humanizeNumber(node.value))];
  } else {
    return [number(String(node.value))];
  }
}
