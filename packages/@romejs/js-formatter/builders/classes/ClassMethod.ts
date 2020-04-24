/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, space, word} from '../../tokens';
import {AnyNode, classMethod} from '@romejs/js-ast';
import {printMethod} from '../utils';

export default function ClassMethod(builder: Builder, node: AnyNode): Tokens {
  node = classMethod.assert(node);

  const tokens: Tokens = printMethod(builder, node);

  if (node.meta.static === true) {
    return [word('static'), space, concat(tokens)];
  } else {
    return tokens;
  }
}
