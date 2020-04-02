/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {ClassMethod, classMethod, AnyNode} from '@romejs/js-ast';
import {printMethod} from '../utils';

export default function ClassMethod(builder: Builder, node: AnyNode): Tokens {
  node = classMethod.assert(node);

  let tokens: Tokens = [];

  if (node.meta.static === true) {
    tokens = [word('static'), space];
  }

  return [...tokens, ...printMethod(builder, node)];
}
