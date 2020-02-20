/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowDeclareExportDefault,
  flowDeclareExportDefault,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareExportDefault(
  generator: Generator,
  node: AnyNode,
) {
  node = flowDeclareExportDefault.assert(node);

  flowDeclareExportDefault.assert(node);
  throw new Error('unimplemented');
}
