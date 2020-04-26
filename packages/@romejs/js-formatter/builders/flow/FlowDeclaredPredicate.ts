/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowDeclaredPredicate} from '@romejs/js-ast';

export default function FlowDeclaredPredicate(
  builder: Builder,
  node: FlowDeclaredPredicate,
): Token {
  return concat(['%checks', space, builder.tokenize(node.value, node)]);
}
