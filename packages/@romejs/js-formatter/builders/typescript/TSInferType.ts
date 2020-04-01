/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSInferType, tsInferType, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space} from '../../tokens';

export default function TSInferType(builder: Builder, node: AnyNode): Tokens {
  node = tsInferType.assert(node);

  return [operator('infer'), space, ...builder.print(node.typeParameter, node)];
}
