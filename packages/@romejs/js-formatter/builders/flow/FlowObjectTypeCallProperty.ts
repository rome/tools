/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowObjectTypeCallProperty} from '@romejs/js-ast';

export default function FlowObjectTypeCallProperty(
  builder: Builder,
  node: FlowObjectTypeCallProperty,
): Token {
  if (node.static === true) {
    return concat(['static', space, builder.tokenize(node.value, node)]);
  } else {
    return builder.tokenize(node.value, node);
  }
}
