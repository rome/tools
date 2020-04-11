/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  FlowClassImplements,
  flowClassImplements,
  AnyNode,
} from '@romejs/js-ast';
import FlowInterfaceExtends from './FlowInterfaceExtends';

export default function FlowClassImplements(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowClassImplements.assert(node);
  return FlowInterfaceExtends(builder, node);
}
