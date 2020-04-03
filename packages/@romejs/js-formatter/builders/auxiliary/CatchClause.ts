/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat} from '../../tokens';
import {catchClause, AnyNode} from '@romejs/js-ast';
import {word, space, operator} from '@romejs/js-formatter/tokens';

export default function CatchClause(builder: Builder, node: AnyNode): Tokens {
  node = catchClause.assert(node);

  return [
    word('catch'),
    space,
    operator('('),
    concat(builder.tokenize(node.param, node)),
    operator(')'),
    space,
    concat(builder.tokenize(node.body, node)),
  ];
}
