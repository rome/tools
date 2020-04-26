/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {
  FlowClassImplements,
  FlowGenericTypeAnnotation,
  FlowInterfaceExtends,
} from '@romejs/js-ast';

export default function FlowInterfaceExtends(
  builder: Builder,
  node: FlowInterfaceExtends | FlowGenericTypeAnnotation | FlowClassImplements,
): Token {
  return concat([
    builder.tokenize(node.id, node),
    builder.tokenize(node.typeParameters, node),
  ]);
}
