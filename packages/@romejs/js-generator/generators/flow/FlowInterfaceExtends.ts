/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {
  FlowInterfaceExtends,
  flowInterfaceExtends,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceExtends(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node =
    node.type === 'FlowGenericTypeAnnotation' ||
    node.type === 'FlowClassImplements'
      ? node
      : flowInterfaceExtends.assert(node);

  return [
    ...generator.print(node.id, node),
    ...generator.print(node.typeParameters, node),
  ];
}
