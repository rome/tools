/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  FlowInterfaceExtends,
  flowInterfaceExtends,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceExtends(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = node.type === 'FlowGenericTypeAnnotation' || node.type ===
    'FlowClassImplements' ? node : flowInterfaceExtends.assert(node);

  return [
    ...builder.tokenize(node.id, node),
    ...builder.tokenize(node.typeParameters, node),
  ];
}
