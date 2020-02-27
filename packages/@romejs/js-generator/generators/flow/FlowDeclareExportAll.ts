/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowDeclareExportAll,
  flowDeclareExportAll,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareExportAll(
  generator: Generator,
  node: AnyNode,
) {
  node = flowDeclareExportAll.assert(node);

  flowDeclareExportAll.assert(node);
  throw new Error('unimplemented');
}
