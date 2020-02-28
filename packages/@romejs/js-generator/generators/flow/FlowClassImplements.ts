/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowClassImplements,
  flowClassImplements,
  AnyNode,
} from '@romejs/js-ast';
import FlowInterfaceExtends from './FlowInterfaceExtends';

export default function FlowClassImplements(
  generator: Generator,
  node: AnyNode,
) {
  node = flowClassImplements.assert(node);
  FlowInterfaceExtends(generator, node);
}
