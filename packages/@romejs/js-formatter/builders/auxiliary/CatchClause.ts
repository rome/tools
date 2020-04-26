/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {CatchClause} from '@romejs/js-ast';

export default function CatchClause(builder: Builder, node: CatchClause): Token {
  if (node.param) {
    return concat([
      'catch',
      space,
      '(',
      builder.tokenize(node.param, node),
      ') ',
      builder.tokenize(node.body, node),
    ]);
  } else {
    return concat(['catch', space, builder.tokenize(node.body, node)]);
  }
}
