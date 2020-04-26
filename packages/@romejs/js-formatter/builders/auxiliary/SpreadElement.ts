/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {SpreadElement} from '@romejs/js-ast';
import {Token, concat} from '../../tokens';

export default function SpreadElement(
  builder: Builder,
  node: SpreadElement,
): Token {
  return concat(['...', builder.tokenize(node.argument, node)]);
}
