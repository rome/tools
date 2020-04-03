/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, operator, concat} from '../../tokens';
import {importCall, AnyNode} from '@romejs/js-ast';

export default function ImportCall(builder: Builder, node: AnyNode): Tokens {
  node = importCall.assert(node);

  return [
    word('import'),
    operator('('),
    concat(builder.tokenize(node.argument, node)),
    operator(')'),
  ];
}
