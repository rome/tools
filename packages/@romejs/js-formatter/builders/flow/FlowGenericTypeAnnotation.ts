/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  AnyNode,
  FlowGenericTypeAnnotation,
  flowGenericTypeAnnotation,
} from '@romejs/js-ast';
import FlowInterfaceExtends from './FlowInterfaceExtends';

export default function FlowGenericTypeAnnotation(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowGenericTypeAnnotation.assert(node);
  return FlowInterfaceExtends(builder, node);
}
