/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeReference, tsTypeReference, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens} from '../../tokens';

export default function TSTypeReference(builder: Builder, node: AnyNode): Tokens {
  node = tsTypeReference.assert(node);
  return [
    ...builder.tokenize(node.typeName, node),
    ...builder.tokenize(node.typeParameters, node),
  ];
}
